// Render the Orchard Action circuit's gates as a readable Markdown
// appendix. The input is the Rust `Debug` rendering of the freshly
// configured (pre-`compress_selectors`) `halo2_proofs::plonk::ConstraintSystem`,
// produced by the `dump_action_constraint_system` test in the orchard
// crate (run with `ORCHARD_DUMP_CONSTRAINT_SYSTEM=1`).
//
// Unlike the pinned verifying key, this dump retains every source-level
// `meta.create_gate(...)` name, the per-constraint labels passed to
// `Constraints::with_selector`, and the original (envelope-free)
// polynomials. We group the output by source-level gate so the appendix
// can serve as the obligation list for formally verifying each gate, with
// the doc sitting next to the code it describes.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process;

#[derive(Debug, Clone)]
enum Node {
    Call(String, Vec<Node>),
    Struct(String, Vec<(String, Node)>),
    Array(Vec<Node>),
    Hex(String),
    Int(String),
    Str(String),
    Ident(String),
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            s: s.as_bytes(),
            i: 0,
        }
    }

    fn skip_ws(&mut self) {
        while self.i < self.s.len() {
            let c = self.s[self.i];
            if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
                self.i += 1;
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_ws();
        if self.i < self.s.len() {
            Some(self.s[self.i])
        } else {
            None
        }
    }

    fn eat(&mut self, c: u8) -> bool {
        self.skip_ws();
        if self.i < self.s.len() && self.s[self.i] == c {
            self.i += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, c: u8) {
        if !self.eat(c) {
            panic!(
                "expected '{}' at offset {} (got {:?})",
                c as char,
                self.i,
                self.s.get(self.i).map(|&b| b as char)
            );
        }
    }

    fn parse_ident(&mut self) -> String {
        self.skip_ws();
        let start = self.i;
        while self.i < self.s.len() {
            let c = self.s[self.i];
            let is_ident = c.is_ascii_alphanumeric() || c == b'_';
            if !is_ident {
                break;
            }
            self.i += 1;
        }
        std::str::from_utf8(&self.s[start..self.i])
            .unwrap()
            .to_string()
    }

    fn parse_expr(&mut self) -> Node {
        let c = self.peek().expect("unexpected EOF");
        if c == b'"' {
            self.i += 1;
            let start = self.i;
            while self.i < self.s.len() && self.s[self.i] != b'"' {
                if self.s[self.i] == b'\\' && self.i + 1 < self.s.len() {
                    self.i += 2;
                } else {
                    self.i += 1;
                }
            }
            let val = std::str::from_utf8(&self.s[start..self.i])
                .unwrap()
                .to_string();
            self.i += 1;
            return Node::Str(val);
        }
        if c == b'[' {
            self.i += 1;
            let mut items = vec![];
            while !self.eat(b']') {
                items.push(self.parse_expr());
                self.eat(b',');
            }
            return Node::Array(items);
        }
        if c.is_ascii_digit() || c == b'-' {
            if c == b'0' && self.i + 1 < self.s.len() && self.s[self.i + 1] == b'x' {
                let start = self.i;
                self.i += 2;
                while self.i < self.s.len() && self.s[self.i].is_ascii_hexdigit() {
                    self.i += 1;
                }
                return Node::Hex(
                    std::str::from_utf8(&self.s[start..self.i])
                        .unwrap()
                        .to_string(),
                );
            }
            let start = self.i;
            if self.s[self.i] == b'-' {
                self.i += 1;
            }
            while self.i < self.s.len() && self.s[self.i].is_ascii_digit() {
                self.i += 1;
            }
            return Node::Int(
                std::str::from_utf8(&self.s[start..self.i])
                    .unwrap()
                    .to_string(),
            );
        }
        let name = self.parse_ident();
        if self.peek() == Some(b'(') {
            self.i += 1;
            let mut args = vec![];
            while !self.eat(b')') {
                args.push(self.parse_expr());
                self.eat(b',');
            }
            return Node::Call(name, args);
        }
        if self.peek() == Some(b'{') {
            self.i += 1;
            let mut fields = vec![];
            while !self.eat(b'}') {
                let field_name = self.parse_ident();
                self.expect(b':');
                let val = self.parse_expr();
                fields.push((field_name, val));
                self.eat(b',');
            }
            return Node::Struct(name, fields);
        }
        Node::Ident(name)
    }
}

/// Locate `gates: [ ... ]` and parse the array of `Gate { ... }` structs.
fn extract_gates(s: &str) -> Vec<Node> {
    let marker = "gates: [";
    let idx = s
        .find(marker)
        .expect("could not locate the gates: [ marker; is this a ConstraintSystem Debug dump?");
    let start = idx + marker.len() - 1;
    let mut p = Parser::new(&s[start..]);
    match p.parse_expr() {
        Node::Array(items) => items,
        other => panic!("expected an Array of gates at the marker, got {:?}", other),
    }
}

fn field<'a>(fields: &'a [(String, Node)], name: &str) -> Option<&'a Node> {
    fields.iter().find(|(k, _)| k == name).map(|(_, v)| v)
}

fn field_int(fields: &[(String, Node)], name: &str) -> i64 {
    match field(fields, name) {
        Some(Node::Int(s)) => s.parse().unwrap_or(0),
        _ => panic!("int field {} not found", name),
    }
}

fn field_rotation(fields: &[(String, Node)]) -> i64 {
    if let Some(Node::Call(_, args)) = field(fields, "rotation") {
        if let Some(Node::Int(s)) = args.first() {
            return s.parse().unwrap_or(0);
        }
    }
    0
}

fn shorten_hex(h: &str) -> String {
    let digits = h.strip_prefix("0x").unwrap_or(h);
    let trimmed = digits.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else if trimmed.len() <= 4 {
        format!("\\mathtt{{0x{}}}", trimmed)
    } else {
        format!("\\mathtt{{0x{}\\ldots}}", &trimmed[..6])
    }
}

fn rotation_suffix(r: i64) -> String {
    if r == 0 {
        String::new()
    } else if r > 0 {
        format!("^{{(+{})}}", r)
    } else {
        format!("^{{({})}}", r)
    }
}

/// Is this node the gate's activating selector, i.e. `Selector(Selector(n, _))`?
fn is_selector(node: &Node) -> bool {
    matches!(node, Node::Call(name, _) if name == "Selector")
}

/// Extract the numeric index from a `Selector(Selector(n, _))` node.
fn selector_index(node: &Node) -> Option<i64> {
    if let Node::Call(name, args) = node {
        if name == "Selector" {
            if let Some(Node::Call(_, inner)) = args.first() {
                if let Some(Node::Int(s)) = inner.first() {
                    return s.parse().ok();
                }
            }
        }
    }
    None
}

/// Every constraint is stored as `selector * body`. Peel the leading
/// selector factor off so the math block shows the body alone; return
/// `(selector_index, body)`. If the polynomial does not have the expected
/// shape, return the whole node with no selector.
fn split_selector(node: &Node) -> (Option<i64>, &Node) {
    if let Node::Call(name, args) = node {
        if name == "Product" && args.len() == 2 {
            if is_selector(&args[0]) {
                return (selector_index(&args[0]), &args[1]);
            }
            if is_selector(&args[1]) {
                return (selector_index(&args[1]), &args[0]);
            }
        }
    }
    (None, node)
}

fn to_latex(node: &Node) -> String {
    match node {
        Node::Call(name, args) => match name.as_str() {
            "Product" if args.len() == 2 => format!(
                "\\left({}\\right) \\cdot \\left({}\\right)",
                to_latex(&args[0]),
                to_latex(&args[1])
            ),
            "Sum" if args.len() == 2 => {
                format!("{} + {}", to_latex(&args[0]), to_latex(&args[1]))
            }
            "Negated" if args.len() == 1 => {
                format!("-\\left({}\\right)", to_latex(&args[0]))
            }
            "Constant" if args.len() == 1 => to_latex(&args[0]),
            "Scaled" if args.len() == 2 => format!(
                "{} \\cdot \\left({}\\right)",
                to_latex(&args[1]),
                to_latex(&args[0])
            ),
            // A bare selector that survived (e.g. nested inside a gadget
            // gate body): render it as q_n.
            "Selector" => match selector_index(node) {
                Some(i) => format!("q_{{{}}}", i),
                None => "q".to_string(),
            },
            "Rotation" if args.len() == 1 => to_latex(&args[0]),
            _ => format!(
                "\\mathsf{{{}}}({})",
                name,
                args.iter().map(to_latex).collect::<Vec<_>>().join(", ")
            ),
        },
        Node::Struct(name, fields) => match name.as_str() {
            "Fixed" => {
                let c = field_int(fields, "column_index");
                let r = field_rotation(fields);
                format!("F_{{{}}}{}", c, rotation_suffix(r))
            }
            "Advice" => {
                let c = field_int(fields, "column_index");
                let r = field_rotation(fields);
                format!("A_{{{}}}{}", c, rotation_suffix(r))
            }
            "Instance" => {
                let c = field_int(fields, "column_index");
                let r = field_rotation(fields);
                format!("I_{{{}}}{}", c, rotation_suffix(r))
            }
            _ => format!("\\mathsf{{{}}}\\{{\\ldots\\}}", name),
        },
        Node::Hex(h) => shorten_hex(h),
        Node::Int(s) => s.clone(),
        Node::Str(s) => format!("\\text{{{}}}", s),
        Node::Ident(s) => format!("\\mathsf{{{}}}", s),
        Node::Array(_) => "[\\ldots]".to_string(),
    }
}

/// A parsed gate: source-level name, per-constraint labels, polynomials.
struct Gate {
    name: String,
    constraint_names: Vec<String>,
    polys: Vec<Node>,
}

fn parse_gate(node: &Node) -> Gate {
    let Node::Struct(sname, fields) = node else {
        panic!("expected Gate struct, got {:?}", node);
    };
    assert_eq!(sname, "Gate", "expected a Gate struct");
    let name = match field(fields, "name") {
        Some(Node::Str(s)) => s.clone(),
        _ => "(unnamed)".to_string(),
    };
    let constraint_names = match field(fields, "constraint_names") {
        Some(Node::Array(items)) => items
            .iter()
            .map(|n| match n {
                Node::Str(s) => s.clone(),
                _ => String::new(),
            })
            .collect(),
        _ => vec![],
    };
    let polys = match field(fields, "polys") {
        Some(Node::Array(items)) => items.clone(),
        _ => vec![],
    };
    Gate {
        name,
        constraint_names,
        polys,
    }
}

/// Map a source-level gate name to the chip that defines it. Attribution
/// is by the chip that owns the gate: in-crate gates name a file in this
/// repository, gadget gates name `halo2_gadgets`. The classifier is
/// name-based and conservative; an unrecognised name falls back to the
/// generic `halo2_gadgets` bucket.
fn chip_of(name: &str) -> &'static str {
    match name {
        "Orchard circuit checks" => "Action (src/circuit.rs)",
        "Field element addition: c = a + b" => "AddChip (src/circuit/gadget/add_chip.rs)",
        "CommitIvk canonicity check" => "CommitIvkChip (src/circuit/commit_ivk.rs)",
        n if n.starts_with("NoteCommit") || n == "y coordinate checks" => {
            "NoteCommitChip (src/circuit/note_commit.rs)"
        }
        "full round" | "partial rounds" | "pad-and-add" => "PoseidonChip (halo2_gadgets)",
        "Sinsemilla gate" | "Initial y_Q" => "SinsemillaChip (halo2_gadgets)",
        "a' = b ⋅ swap + a ⋅ (1-swap)" | "Decomposition check" => "MerkleChip (halo2_gadgets)",
        _ => "EccChip / utilities (halo2_gadgets)",
    }
}

fn front_matter() {
    println!("---");
    println!("sidebar_position: 21");
    println!("title: \"Appendix: Action Circuit Gate Constraints\"");
    println!(
        "description: Every gate of the Orchard Action circuit, grouped by \
         source-level create_gate, with named constraints and KaTeX polynomials."
    );
    println!("---");
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_input = "../../data/orchard-action-constraint-system.txt".to_string();
    let in_path = args.get(1).unwrap_or(&default_input);
    let input = match fs::read_to_string(in_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: could not read {}: {}", in_path, e);
            process::exit(1);
        }
    };

    let gates: Vec<Gate> = extract_gates(&input).iter().map(parse_gate).collect();
    let total_polys: usize = gates.iter().map(|g| g.polys.len()).sum();

    front_matter();
    println!("# Appendix: Action Circuit Gate Constraints");
    println!();
    println!(
        "This appendix lists every gate of the Orchard Action circuit: {} \
         source-level gates holding {} polynomial constraints in total. Each",
        gates.len(),
        total_polys
    );
    println!("polynomial $P$ vanishes on every valid assignment: $P = 0$.");
    println!();
    println!("**Provenance.** The gates are read from the `Debug` rendering of");
    println!("the freshly configured (pre-`compress_selectors`)");
    println!("`halo2_proofs::plonk::ConstraintSystem`, emitted by the");
    println!("`dump_action_constraint_system` test in the orchard crate and");
    println!("vendored at");
    println!("`onboarding/data/orchard-action-constraint-system.txt`. Unlike the");
    println!("pinned verifying key, this rendering keeps every");
    println!("`meta.create_gate(...)` name, the per-constraint labels passed to");
    println!("`Constraints::with_selector`, and the original polynomials before");
    println!("Halo 2's selector-compression pass rewrites them. To regenerate");
    println!("after a circuit change, run `make appendix-gates` from the");
    println!("`onboarding/` directory.");
    println!();
    println!("**Why this shape.** Grouping by source-level gate (rather than by");
    println!("the compressed fixed column of the verifying key) keeps the doc");
    println!("next to the code: each gate below is one `create_gate` call, each");
    println!("named constraint is one proof obligation, and the polynomial is the");
    println!("exact expression to formalise. This is the obligation list for");
    println!("verifying the gates one at a time.");
    println!();
    println!("**Notation.**");
    println!();
    println!("- $A_c$, $A_c^{{(+r)}}$, $A_c^{{(-r)}}$: advice column $c$ at the");
    println!("  current row, rotated by $+r$ or $-r$.");
    println!("- $F_c$, $I_c$: fixed and instance column $c$ (with the same");
    println!("  rotation notation).");
    println!("- Each constraint is enforced only when the gate's selector is");
    println!("  active. That selector factor is peeled off and shown as");
    println!("  \"selector $q_n$\" in the heading, so the polynomial below is the");
    println!("  constraint body alone.");
    println!("- Constants are rendered in hex. Values below `0xffff` are shown in");
    println!("  full; larger values are truncated to a six-hex-digit head");
    println!("  followed by `\\ldots` to keep KaTeX readable.");
    println!();

    // Summary table.
    println!("## Summary");
    println!();
    println!("| # | Gate | Constraints | Source |");
    println!("| - | ---- | ----------- | ------ |");
    for (i, g) in gates.iter().enumerate() {
        println!(
            "| {} | `{}` | {} | {} |",
            i + 1,
            g.name,
            g.polys.len(),
            chip_of(&g.name)
        );
    }
    println!();

    // Group gates by owning chip for a short index.
    let mut by_chip: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (i, g) in gates.iter().enumerate() {
        by_chip.entry(chip_of(&g.name)).or_default().push(i + 1);
    }
    println!("## Gates by chip");
    println!();
    for (chip, idxs) in &by_chip {
        let list = idxs
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("- **{}**: gates {}", chip, list);
    }
    println!();

    // One section per source-level gate. Gate names and constraint labels
    // are wrapped in backticks: they are code-level identifiers and often
    // contain `_` or `*`, which MDX would otherwise read as emphasis.
    for (i, g) in gates.iter().enumerate() {
        println!("## Gate {}. `{}`", i + 1, g.name);
        println!();
        println!(
            "_Source: {}. {} constraint{}._",
            chip_of(&g.name),
            g.polys.len(),
            if g.polys.len() == 1 { "" } else { "s" }
        );
        println!();
        for (j, poly) in g.polys.iter().enumerate() {
            let label = g
                .constraint_names
                .get(j)
                .filter(|s| !s.is_empty())
                .cloned()
                .unwrap_or_else(|| format!("constraint {}", j + 1));
            let (sel, body) = split_selector(poly);
            match sel {
                Some(n) => println!("### `{}` (selector $q_{{{}}}$)", label, n),
                None => println!("### `{}`", label),
            }
            println!();
            println!("$$");
            println!("{} = 0", to_latex(body));
            println!("$$");
            println!();
        }
    }
}
