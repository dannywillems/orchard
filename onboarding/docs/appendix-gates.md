---
sidebar_position: 21
title: "Appendix: Action Circuit Gate Constraints"
description: Every gate of the Orchard Action circuit, grouped by source-level create_gate, with named constraints and KaTeX polynomials.
---

# Appendix: Action Circuit Gate Constraints

This appendix lists every gate of the Orchard Action circuit: 55 source-level gates holding 193 polynomial constraints in total. Each
polynomial $P$ vanishes on every valid assignment: $P = 0$.

**Provenance.** The gates are read from the `Debug` rendering of
the freshly configured (pre-`compress_selectors`)
`halo2_proofs::plonk::ConstraintSystem`, emitted by the
`dump_action_constraint_system` test in the orchard crate and
vendored at
`onboarding/data/orchard-action-constraint-system.txt`. Unlike the
pinned verifying key, this rendering keeps every
`meta.create_gate(...)` name, the per-constraint labels passed to
`Constraints::with_selector`, and the original polynomials before
Halo 2's selector-compression pass rewrites them. To regenerate
after a circuit change, run `make appendix-gates` from the
`onboarding/` directory.

**Why this shape.** Grouping by source-level gate (rather than by
the compressed fixed column of the verifying key) keeps the doc
next to the code: each gate below is one `create_gate` call, each
named constraint is one proof obligation, and the polynomial is the
exact expression to formalise. This is the obligation list for
verifying the gates one at a time.

**Notation.**

- $A_c$, $A_c^{(+r)}$, $A_c^{(-r)}$: advice column $c$ at the
  current row, rotated by $+r$ or $-r$.
- $F_c$, $I_c$: fixed and instance column $c$ (with the same
  rotation notation).
- Each constraint is enforced only when the gate's selector is
  active. That selector factor is peeled off and shown as
  "selector $q_n$" in the heading, so the polynomial below is the
  constraint body alone.
- Constants are rendered in hex. Values below `0xffff` are shown in
  full; larger values are truncated to a six-hex-digit head
  followed by `\ldots` to keep KaTeX readable.

## Summary

| #   | Gate                                                      | Constraints | Source                                      |
| --- | --------------------------------------------------------- | ----------- | ------------------------------------------- |
| 1   | `Orchard circuit checks`                                  | 4           | Action (src/circuit.rs)                     |
| 2   | `Field element addition: c = a + b`                       | 1           | AddChip (src/circuit/gadget/add_chip.rs)    |
| 3   | `Short lookup bitshift`                                   | 1           | EccChip / utilities (halo2_gadgets)         |
| 4   | `witness point`                                           | 2           | EccChip / utilities (halo2_gadgets)         |
| 5   | `witness non-identity point`                              | 1           | EccChip / utilities (halo2_gadgets)         |
| 6   | `incomplete addition`                                     | 2           | EccChip / utilities (halo2_gadgets)         |
| 7   | `complete addition`                                       | 12          | EccChip / utilities (halo2_gadgets)         |
| 8   | `q_mul_1 == 1 checks`                                     | 1           | EccChip / utilities (halo2_gadgets)         |
| 9   | `q_mul_2 == 1 checks`                                     | 6           | EccChip / utilities (halo2_gadgets)         |
| 10  | `q_mul_3 == 1 checks`                                     | 4           | EccChip / utilities (halo2_gadgets)         |
| 11  | `q_mul_1 == 1 checks`                                     | 1           | EccChip / utilities (halo2_gadgets)         |
| 12  | `q_mul_2 == 1 checks`                                     | 6           | EccChip / utilities (halo2_gadgets)         |
| 13  | `q_mul_3 == 1 checks`                                     | 4           | EccChip / utilities (halo2_gadgets)         |
| 14  | `Decompose scalar for complete bits of variable-base mul` | 2           | EccChip / utilities (halo2_gadgets)         |
| 15  | `overflow checks`                                         | 5           | EccChip / utilities (halo2_gadgets)         |
| 16  | `LSB check`                                               | 3           | EccChip / utilities (halo2_gadgets)         |
| 17  | `range check`                                             | 1           | EccChip / utilities (halo2_gadgets)         |
| 18  | `Running sum coordinates check`                           | 3           | EccChip / utilities (halo2_gadgets)         |
| 19  | `Full-width fixed-base scalar mul`                        | 4           | EccChip / utilities (halo2_gadgets)         |
| 20  | `Short fixed-base mul gate`                               | 4           | EccChip / utilities (halo2_gadgets)         |
| 21  | `Canonicity checks`                                       | 8           | EccChip / utilities (halo2_gadgets)         |
| 22  | `full round`                                              | 3           | PoseidonChip (halo2_gadgets)                |
| 23  | `partial rounds`                                          | 4           | PoseidonChip (halo2_gadgets)                |
| 24  | `pad-and-add`                                             | 3           | PoseidonChip (halo2_gadgets)                |
| 25  | `Initial y_Q`                                             | 1           | SinsemillaChip (halo2_gadgets)              |
| 26  | `Sinsemilla gate`                                         | 2           | SinsemillaChip (halo2_gadgets)              |
| 27  | `a' = b ⋅ swap + a ⋅ (1-swap)`                            | 3           | MerkleChip (halo2_gadgets)                  |
| 28  | `Decomposition check`                                     | 4           | MerkleChip (halo2_gadgets)                  |
| 29  | `Initial y_Q`                                             | 1           | SinsemillaChip (halo2_gadgets)              |
| 30  | `Sinsemilla gate`                                         | 2           | SinsemillaChip (halo2_gadgets)              |
| 31  | `a' = b ⋅ swap + a ⋅ (1-swap)`                            | 3           | MerkleChip (halo2_gadgets)                  |
| 32  | `Decomposition check`                                     | 4           | MerkleChip (halo2_gadgets)                  |
| 33  | `CommitIvk canonicity check`                              | 14          | CommitIvkChip (src/circuit/commit_ivk.rs)   |
| 34  | `NoteCommit MessagePiece b`                               | 3           | NoteCommitChip (src/circuit/note_commit.rs) |
| 35  | `NoteCommit MessagePiece d`                               | 3           | NoteCommitChip (src/circuit/note_commit.rs) |
| 36  | `NoteCommit MessagePiece e`                               | 1           | NoteCommitChip (src/circuit/note_commit.rs) |
| 37  | `NoteCommit MessagePiece g`                               | 2           | NoteCommitChip (src/circuit/note_commit.rs) |
| 38  | `NoteCommit MessagePiece h`                               | 2           | NoteCommitChip (src/circuit/note_commit.rs) |
| 39  | `NoteCommit input g_d`                                    | 5           | NoteCommitChip (src/circuit/note_commit.rs) |
| 40  | `NoteCommit input pk_d`                                   | 4           | NoteCommitChip (src/circuit/note_commit.rs) |
| 41  | `NoteCommit input value`                                  | 1           | NoteCommitChip (src/circuit/note_commit.rs) |
| 42  | `NoteCommit input rho`                                    | 4           | NoteCommitChip (src/circuit/note_commit.rs) |
| 43  | `NoteCommit input psi`                                    | 5           | NoteCommitChip (src/circuit/note_commit.rs) |
| 44  | `y coordinate checks`                                     | 7           | NoteCommitChip (src/circuit/note_commit.rs) |
| 45  | `NoteCommit MessagePiece b`                               | 3           | NoteCommitChip (src/circuit/note_commit.rs) |
| 46  | `NoteCommit MessagePiece d`                               | 3           | NoteCommitChip (src/circuit/note_commit.rs) |
| 47  | `NoteCommit MessagePiece e`                               | 1           | NoteCommitChip (src/circuit/note_commit.rs) |
| 48  | `NoteCommit MessagePiece g`                               | 2           | NoteCommitChip (src/circuit/note_commit.rs) |
| 49  | `NoteCommit MessagePiece h`                               | 2           | NoteCommitChip (src/circuit/note_commit.rs) |
| 50  | `NoteCommit input g_d`                                    | 5           | NoteCommitChip (src/circuit/note_commit.rs) |
| 51  | `NoteCommit input pk_d`                                   | 4           | NoteCommitChip (src/circuit/note_commit.rs) |
| 52  | `NoteCommit input value`                                  | 1           | NoteCommitChip (src/circuit/note_commit.rs) |
| 53  | `NoteCommit input rho`                                    | 4           | NoteCommitChip (src/circuit/note_commit.rs) |
| 54  | `NoteCommit input psi`                                    | 5           | NoteCommitChip (src/circuit/note_commit.rs) |
| 55  | `y coordinate checks`                                     | 7           | NoteCommitChip (src/circuit/note_commit.rs) |

## Gates by chip

- **Action (src/circuit.rs)**: gates 1
- **AddChip (src/circuit/gadget/add_chip.rs)**: gates 2
- **CommitIvkChip (src/circuit/commit_ivk.rs)**: gates 33
- **EccChip / utilities (halo2_gadgets)**: gates 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21
- **MerkleChip (halo2_gadgets)**: gates 27, 28, 31, 32
- **NoteCommitChip (src/circuit/note_commit.rs)**: gates 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55
- **PoseidonChip (halo2_gadgets)**: gates 22, 23, 24
- **SinsemillaChip (halo2_gadgets)**: gates 25, 26, 29, 30

## Gate 1. `Orchard circuit checks`

_Source: Action (src/circuit.rs). 4 constraints._

### `v_old - v_new = magnitude * sign` (selector $q_{0}$)

$$
A_{0} + -\left(A_{1}\right) + -\left(\left(A_{2}\right) \cdot \left(A_{3}\right)\right) = 0
$$

### `Either v_old = 0, or root = anchor` (selector $q_{0}$)

$$
\left(A_{0}\right) \cdot \left(A_{4} + -\left(A_{5}\right)\right) = 0
$$

### `v_old = 0 or enable_spends = 1` (selector $q_{0}$)

$$
\left(A_{0}\right) \cdot \left(\mathtt{0x1} + -\left(A_{6}\right)\right) = 0
$$

### `v_new = 0 or enable_outputs = 1` (selector $q_{0}$)

$$
\left(A_{1}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}\right)\right) = 0
$$

## Gate 2. `Field element addition: c = a + b`

_Source: AddChip (src/circuit/gadget/add_chip.rs). 1 constraint._

### `constraint 1` (selector $q_{1}$)

$$
A_{7} + A_{8} + -\left(A_{6}\right) = 0
$$

## Gate 3. `Short lookup bitshift`

_Source: EccChip / utilities (halo2_gadgets). 1 constraint._

### `constraint 1` (selector $q_{4}$)

$$
\left(\mathtt{0x400} \cdot \left(A_{9}^{(-1)}\right)\right) \cdot \left(A_{9}^{(+1)}\right) + -\left(A_{9}\right) = 0
$$

## Gate 4. `witness point`

_Source: EccChip / utilities (halo2_gadgets). 2 constraints._

### `x == 0 v on_curve`

$$
\left(\left(q_{5}\right) \cdot \left(A_{0}\right)\right) \cdot \left(\left(A_{1}\right) \cdot \left(A_{1}\right) + -\left(\left(\left(A_{0}\right) \cdot \left(A_{0}\right)\right) \cdot \left(A_{0}\right)\right) + -\left(\mathtt{0x5}\right)\right) = 0
$$

### `y == 0 v on_curve`

$$
\left(\left(q_{5}\right) \cdot \left(A_{1}\right)\right) \cdot \left(\left(A_{1}\right) \cdot \left(A_{1}\right) + -\left(\left(\left(A_{0}\right) \cdot \left(A_{0}\right)\right) \cdot \left(A_{0}\right)\right) + -\left(\mathtt{0x5}\right)\right) = 0
$$

## Gate 5. `witness non-identity point`

_Source: EccChip / utilities (halo2_gadgets). 1 constraint._

### `on_curve` (selector $q_{6}$)

$$
\left(A_{1}\right) \cdot \left(A_{1}\right) + -\left(\left(\left(A_{0}\right) \cdot \left(A_{0}\right)\right) \cdot \left(A_{0}\right)\right) + -\left(\mathtt{0x5}\right) = 0
$$

## Gate 6. `incomplete addition`

_Source: EccChip / utilities (halo2_gadgets). 2 constraints._

### `x_r` (selector $q_{7}$)

$$
\left(\left(A_{2}^{(+1)} + A_{2} + A_{0}\right) \cdot \left(A_{0} + -\left(A_{2}\right)\right)\right) \cdot \left(A_{0} + -\left(A_{2}\right)\right) + -\left(\left(A_{1} + -\left(A_{3}\right)\right) \cdot \left(A_{1} + -\left(A_{3}\right)\right)\right) = 0
$$

### `y_r` (selector $q_{7}$)

$$
\left(A_{3}^{(+1)} + A_{3}\right) \cdot \left(A_{0} + -\left(A_{2}\right)\right) + -\left(\left(A_{1} + -\left(A_{3}\right)\right) \cdot \left(A_{2} + -\left(A_{2}^{(+1)}\right)\right)\right) = 0
$$

## Gate 7. `complete addition`

_Source: EccChip / utilities (halo2_gadgets). 12 constraints._

### `1` (selector $q_{8}$)

$$
\left(A_{2} + -\left(A_{0}\right)\right) \cdot \left(\left(A_{2} + -\left(A_{0}\right)\right) \cdot \left(A_{4}\right) + -\left(A_{3} + -\left(A_{1}\right)\right)\right) = 0
$$

### `2` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{2} + -\left(A_{0}\right)\right) \cdot \left(A_{5}\right)\right)\right) \cdot \left(\left(\left(\mathtt{0x2}\right) \cdot \left(A_{1}\right)\right) \cdot \left(A_{4}\right) + -\left(\left(\mathtt{0x3}\right) \cdot \left(\left(A_{0}\right) \cdot \left(A_{0}\right)\right)\right)\right) = 0
$$

### `3a` (selector $q_{8}$)

$$
\left(\left(\left(A_{0}\right) \cdot \left(A_{2}\right)\right) \cdot \left(A_{2} + -\left(A_{0}\right)\right)\right) \cdot \left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{0}\right) + -\left(A_{2}\right) + -\left(A_{2}^{(+1)}\right)\right) = 0
$$

### `3b` (selector $q_{8}$)

$$
\left(\left(\left(A_{0}\right) \cdot \left(A_{2}\right)\right) \cdot \left(A_{2} + -\left(A_{0}\right)\right)\right) \cdot \left(\left(A_{4}\right) \cdot \left(A_{0} + -\left(A_{2}^{(+1)}\right)\right) + -\left(A_{1}\right) + -\left(A_{3}^{(+1)}\right)\right) = 0
$$

### `3c` (selector $q_{8}$)

$$
\left(\left(\left(A_{0}\right) \cdot \left(A_{2}\right)\right) \cdot \left(A_{3} + A_{1}\right)\right) \cdot \left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{0}\right) + -\left(A_{2}\right) + -\left(A_{2}^{(+1)}\right)\right) = 0
$$

### `3d` (selector $q_{8}$)

$$
\left(\left(\left(A_{0}\right) \cdot \left(A_{2}\right)\right) \cdot \left(A_{3} + A_{1}\right)\right) \cdot \left(\left(A_{4}\right) \cdot \left(A_{0} + -\left(A_{2}^{(+1)}\right)\right) + -\left(A_{1}\right) + -\left(A_{3}^{(+1)}\right)\right) = 0
$$

### `4a` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{0}\right) \cdot \left(A_{6}\right)\right)\right) \cdot \left(A_{2}^{(+1)} + -\left(A_{2}\right)\right) = 0
$$

### `4b` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{0}\right) \cdot \left(A_{6}\right)\right)\right) \cdot \left(A_{3}^{(+1)} + -\left(A_{3}\right)\right) = 0
$$

### `5a` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{2}\right) \cdot \left(A_{7}\right)\right)\right) \cdot \left(A_{2}^{(+1)} + -\left(A_{0}\right)\right) = 0
$$

### `5b` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{2}\right) \cdot \left(A_{7}\right)\right)\right) \cdot \left(A_{3}^{(+1)} + -\left(A_{1}\right)\right) = 0
$$

### `6a` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{2} + -\left(A_{0}\right)\right) \cdot \left(A_{5}\right)\right) + -\left(\left(A_{3} + A_{1}\right) \cdot \left(A_{8}\right)\right)\right) \cdot \left(A_{2}^{(+1)}\right) = 0
$$

### `6b` (selector $q_{8}$)

$$
\left(\mathtt{0x1} + -\left(\left(A_{2} + -\left(A_{0}\right)\right) \cdot \left(A_{5}\right)\right) + -\left(\left(A_{3} + A_{1}\right) \cdot \left(A_{8}\right)\right)\right) \cdot \left(A_{3}^{(+1)}\right) = 0
$$

## Gate 8. `q_mul_1 == 1 checks`

_Source: EccChip / utilities (halo2_gadgets). 1 constraint._

### `init y_a` (selector $q_{9}$)

$$
A_{4} + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{4}^{(+1)} + A_{5}^{(+1)}\right) \cdot \left(A_{3}^{(+1)} + -\left(\left(A_{4}^{(+1)}\right) \cdot \left(A_{4}^{(+1)}\right) + -\left(A_{3}^{(+1)}\right) + -\left(A_{0}^{(+1)}\right)\right)\right)\right)\right) = 0
$$

## Gate 9. `q_mul_2 == 1 checks`

_Source: EccChip / utilities (halo2_gadgets). 6 constraints._

### `x_p_check` (selector $q_{10}$)

$$
A_{0} + -\left(A_{0}^{(+1)}\right) = 0
$$

### `y_p_check` (selector $q_{10}$)

$$
A_{1} + -\left(A_{1}^{(+1)}\right) = 0
$$

### `bool_check` (selector $q_{10}$)

$$
\left(A_{9} + -\left(\mathtt{0x2} \cdot \left(A_{9}^{(-1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{9} + -\left(\mathtt{0x2} \cdot \left(A_{9}^{(-1)}\right)\right)\right)\right) = 0
$$

### `gradient_1` (selector $q_{10}$)

$$
\left(A_{4}\right) \cdot \left(A_{3} + -\left(A_{0}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{4} + A_{5}\right) \cdot \left(A_{3} + -\left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{3}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + \left(\mathtt{0x2} \cdot \left(A_{9} + -\left(\mathtt{0x2} \cdot \left(A_{9}^{(-1)}\right)\right)\right) + -\left(\mathtt{0x1}\right)\right) \cdot \left(A_{1}\right) = 0
$$

### `secant_line` (selector $q_{10}$)

$$
\left(A_{5}\right) \cdot \left(A_{5}\right) + -\left(A_{3}^{(+1)}\right) + -\left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{3}\right) + -\left(A_{0}\right)\right) + -\left(A_{3}\right) = 0
$$

### `gradient_2` (selector $q_{10}$)

$$
\left(A_{5}\right) \cdot \left(A_{3} + -\left(A_{3}^{(+1)}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{4} + A_{5}\right) \cdot \left(A_{3} + -\left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{3}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{4}^{(+1)} + A_{5}^{(+1)}\right) \cdot \left(A_{3}^{(+1)} + -\left(\left(A_{4}^{(+1)}\right) \cdot \left(A_{4}^{(+1)}\right) + -\left(A_{3}^{(+1)}\right) + -\left(A_{0}^{(+1)}\right)\right)\right)\right)\right) = 0
$$

## Gate 10. `q_mul_3 == 1 checks`

_Source: EccChip / utilities (halo2_gadgets). 4 constraints._

### `bool_check` (selector $q_{11}$)

$$
\left(A_{9} + -\left(\mathtt{0x2} \cdot \left(A_{9}^{(-1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{9} + -\left(\mathtt{0x2} \cdot \left(A_{9}^{(-1)}\right)\right)\right)\right) = 0
$$

### `gradient_1` (selector $q_{11}$)

$$
\left(A_{4}\right) \cdot \left(A_{3} + -\left(A_{0}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{4} + A_{5}\right) \cdot \left(A_{3} + -\left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{3}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + \left(\mathtt{0x2} \cdot \left(A_{9} + -\left(\mathtt{0x2} \cdot \left(A_{9}^{(-1)}\right)\right)\right) + -\left(\mathtt{0x1}\right)\right) \cdot \left(A_{1}\right) = 0
$$

### `secant_line` (selector $q_{11}$)

$$
\left(A_{5}\right) \cdot \left(A_{5}\right) + -\left(A_{3}^{(+1)}\right) + -\left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{3}\right) + -\left(A_{0}\right)\right) + -\left(A_{3}\right) = 0
$$

### `gradient_2` (selector $q_{11}$)

$$
\left(A_{5}\right) \cdot \left(A_{3} + -\left(A_{3}^{(+1)}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{4} + A_{5}\right) \cdot \left(A_{3} + -\left(\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{3}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + -\left(A_{4}^{(+1)}\right) = 0
$$

## Gate 11. `q_mul_1 == 1 checks`

_Source: EccChip / utilities (halo2_gadgets). 1 constraint._

### `init y_a` (selector $q_{12}$)

$$
A_{8} + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{8}^{(+1)} + A_{2}^{(+1)}\right) \cdot \left(A_{7}^{(+1)} + -\left(\left(A_{8}^{(+1)}\right) \cdot \left(A_{8}^{(+1)}\right) + -\left(A_{7}^{(+1)}\right) + -\left(A_{0}^{(+1)}\right)\right)\right)\right)\right) = 0
$$

## Gate 12. `q_mul_2 == 1 checks`

_Source: EccChip / utilities (halo2_gadgets). 6 constraints._

### `x_p_check` (selector $q_{13}$)

$$
A_{0} + -\left(A_{0}^{(+1)}\right) = 0
$$

### `y_p_check` (selector $q_{13}$)

$$
A_{1} + -\left(A_{1}^{(+1)}\right) = 0
$$

### `bool_check` (selector $q_{13}$)

$$
\left(A_{6} + -\left(\mathtt{0x2} \cdot \left(A_{6}^{(-1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{6} + -\left(\mathtt{0x2} \cdot \left(A_{6}^{(-1)}\right)\right)\right)\right) = 0
$$

### `gradient_1` (selector $q_{13}$)

$$
\left(A_{8}\right) \cdot \left(A_{7} + -\left(A_{0}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{8} + A_{2}\right) \cdot \left(A_{7} + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{7}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + \left(\mathtt{0x2} \cdot \left(A_{6} + -\left(\mathtt{0x2} \cdot \left(A_{6}^{(-1)}\right)\right)\right) + -\left(\mathtt{0x1}\right)\right) \cdot \left(A_{1}\right) = 0
$$

### `secant_line` (selector $q_{13}$)

$$
\left(A_{2}\right) \cdot \left(A_{2}\right) + -\left(A_{7}^{(+1)}\right) + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{7}\right) + -\left(A_{0}\right)\right) + -\left(A_{7}\right) = 0
$$

### `gradient_2` (selector $q_{13}$)

$$
\left(A_{2}\right) \cdot \left(A_{7} + -\left(A_{7}^{(+1)}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{8} + A_{2}\right) \cdot \left(A_{7} + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{7}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{8}^{(+1)} + A_{2}^{(+1)}\right) \cdot \left(A_{7}^{(+1)} + -\left(\left(A_{8}^{(+1)}\right) \cdot \left(A_{8}^{(+1)}\right) + -\left(A_{7}^{(+1)}\right) + -\left(A_{0}^{(+1)}\right)\right)\right)\right)\right) = 0
$$

## Gate 13. `q_mul_3 == 1 checks`

_Source: EccChip / utilities (halo2_gadgets). 4 constraints._

### `bool_check` (selector $q_{14}$)

$$
\left(A_{6} + -\left(\mathtt{0x2} \cdot \left(A_{6}^{(-1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{6} + -\left(\mathtt{0x2} \cdot \left(A_{6}^{(-1)}\right)\right)\right)\right) = 0
$$

### `gradient_1` (selector $q_{14}$)

$$
\left(A_{8}\right) \cdot \left(A_{7} + -\left(A_{0}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{8} + A_{2}\right) \cdot \left(A_{7} + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{7}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + \left(\mathtt{0x2} \cdot \left(A_{6} + -\left(\mathtt{0x2} \cdot \left(A_{6}^{(-1)}\right)\right)\right) + -\left(\mathtt{0x1}\right)\right) \cdot \left(A_{1}\right) = 0
$$

### `secant_line` (selector $q_{14}$)

$$
\left(A_{2}\right) \cdot \left(A_{2}\right) + -\left(A_{7}^{(+1)}\right) + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{7}\right) + -\left(A_{0}\right)\right) + -\left(A_{7}\right) = 0
$$

### `gradient_2` (selector $q_{14}$)

$$
\left(A_{2}\right) \cdot \left(A_{7} + -\left(A_{7}^{(+1)}\right)\right) + -\left(\mathtt{0x200000\ldots} \cdot \left(\left(A_{8} + A_{2}\right) \cdot \left(A_{7} + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{7}\right) + -\left(A_{0}\right)\right)\right)\right)\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

## Gate 14. `Decompose scalar for complete bits of variable-base mul`

_Source: EccChip / utilities (halo2_gadgets). 2 constraints._

### `bool_check` (selector $q_{15}$)

$$
\left(A_{9}^{(+1)} + -\left(\left(\mathtt{0x2}\right) \cdot \left(A_{9}^{(-1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{9}^{(+1)} + -\left(\left(\mathtt{0x2}\right) \cdot \left(A_{9}^{(-1)}\right)\right)\right)\right) = 0
$$

### `y_switch` (selector $q_{15}$)

$$
\left(A_{9}^{(+1)} + -\left(\left(\mathtt{0x2}\right) \cdot \left(A_{9}^{(-1)}\right)\right)\right) \cdot \left(A_{9} + -\left(A_{1}^{(-1)}\right)\right) + \left(\mathtt{0x1} + -\left(A_{9}^{(+1)} + -\left(\left(\mathtt{0x2}\right) \cdot \left(A_{9}^{(-1)}\right)\right)\right)\right) \cdot \left(A_{9} + A_{1}^{(-1)}\right) = 0
$$

## Gate 15. `overflow checks`

_Source: EccChip / utilities (halo2_gadgets). 5 constraints._

### `s_check` (selector $q_{16}$)

$$
A_{8} + -\left(A_{7} + \left(A_{7}^{(-1)}\right) \cdot \left(\left(\mathtt{0x100000\ldots}\right) \cdot \left(\mathtt{0x40}\right)\right)\right) = 0
$$

### `recovery` (selector $q_{16}$)

$$
A_{6}^{(-1)} + -\left(A_{7}\right) + -\left(\mathtt{0x224698\ldots}\right) = 0
$$

### `lo_zero` (selector $q_{16}$)

$$
\left(A_{7}^{(-1)}\right) \cdot \left(A_{6} + -\left(\mathtt{0x100000\ldots}\right)\right) = 0
$$

### `s_minus_lo_130_check` (selector $q_{16}$)

$$
\left(A_{7}^{(-1)}\right) \cdot \left(A_{7}^{(+1)}\right) = 0
$$

### `canonicity` (selector $q_{16}$)

$$
\left(\left(\mathtt{0x1} + -\left(A_{7}^{(-1)}\right)\right) \cdot \left(\mathtt{0x1} + -\left(\left(A_{6}\right) \cdot \left(A_{6}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{7}^{(+1)}\right) = 0
$$

## Gate 16. `LSB check`

_Source: EccChip / utilities (halo2_gadgets). 3 constraints._

### `bool_check` (selector $q_{17}$)

$$
\left(A_{9}^{(+1)} + -\left(\mathtt{0x2} \cdot \left(A_{9}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{9}^{(+1)} + -\left(\mathtt{0x2} \cdot \left(A_{9}\right)\right)\right)\right) = 0
$$

### `lsb_x` (selector $q_{17}$)

$$
\left(A_{9}^{(+1)} + -\left(\mathtt{0x2} \cdot \left(A_{9}\right)\right)\right) \cdot \left(A_{0}\right) + \left(\mathtt{0x1} + -\left(A_{9}^{(+1)} + -\left(\mathtt{0x2} \cdot \left(A_{9}\right)\right)\right)\right) \cdot \left(A_{0} + -\left(A_{0}^{(+1)}\right)\right) = 0
$$

### `lsb_y` (selector $q_{17}$)

$$
\left(A_{9}^{(+1)} + -\left(\mathtt{0x2} \cdot \left(A_{9}\right)\right)\right) \cdot \left(A_{1}\right) + \left(\mathtt{0x1} + -\left(A_{9}^{(+1)} + -\left(\mathtt{0x2} \cdot \left(A_{9}\right)\right)\right)\right) \cdot \left(A_{1} + A_{1}^{(+1)}\right) = 0
$$

## Gate 17. `range check`

_Source: EccChip / utilities (halo2_gadgets). 1 constraint._

### `constraint 1` (selector $q_{18}$)

$$
\left(\left(\left(\left(\left(\left(\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right)\right) \cdot \left(\mathtt{0x2} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right)\right) \cdot \left(\mathtt{0x3} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right)\right) \cdot \left(\mathtt{0x4} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right)\right) \cdot \left(\mathtt{0x5} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right)\right) \cdot \left(\mathtt{0x6} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right)\right) \cdot \left(\mathtt{0x7} + -\left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) = 0
$$

## Gate 18. `Running sum coordinates check`

_Source: EccChip / utilities (halo2_gadgets). 3 constraints._

### `check x` (selector $q_{18}$)

$$
0 + \left(\mathtt{0x1}\right) \cdot \left(F_{3}\right) + \left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{4}\right) + \left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{5}\right) + \left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{6}\right) + \left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{7}\right) + \left(\left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{8}\right) + \left(\left(\left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{9}\right) + \left(\left(\left(\left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(A_{4} + -\left(\mathtt{0x8} \cdot \left(A_{4}^{(+1)}\right)\right)\right)\right) \cdot \left(F_{10}\right) + -\left(A_{0}\right) = 0
$$

### `check y` (selector $q_{18}$)

$$
\left(A_{5}\right) \cdot \left(A_{5}\right) + -\left(A_{1}\right) + -\left(F_{11}\right) = 0
$$

### `on-curve` (selector $q_{18}$)

$$
\left(A_{1}\right) \cdot \left(A_{1}\right) + -\left(\left(\left(A_{0}\right) \cdot \left(A_{0}\right)\right) \cdot \left(A_{0}\right)\right) + -\left(\mathtt{0x5}\right) = 0
$$

## Gate 19. `Full-width fixed-base scalar mul`

_Source: EccChip / utilities (halo2_gadgets). 4 constraints._

### `check x` (selector $q_{19}$)

$$
0 + \left(\mathtt{0x1}\right) \cdot \left(F_{3}\right) + \left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{4}\right) + \left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{5}\right) + \left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{6}\right) + \left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{7}\right) + \left(\left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{8}\right) + \left(\left(\left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{9}\right) + \left(\left(\left(\left(\left(\left(\left(\left(\mathtt{0x1}\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(A_{4}\right)\right) \cdot \left(F_{10}\right) + -\left(A_{0}\right) = 0
$$

### `check y` (selector $q_{19}$)

$$
\left(A_{5}\right) \cdot \left(A_{5}\right) + -\left(A_{1}\right) + -\left(F_{11}\right) = 0
$$

### `on-curve` (selector $q_{19}$)

$$
\left(A_{1}\right) \cdot \left(A_{1}\right) + -\left(\left(\left(A_{0}\right) \cdot \left(A_{0}\right)\right) \cdot \left(A_{0}\right)\right) + -\left(\mathtt{0x5}\right) = 0
$$

### `window range check` (selector $q_{19}$)

$$
\left(\left(\left(\left(\left(\left(\left(A_{4}\right) \cdot \left(\mathtt{0x1} + -\left(A_{4}\right)\right)\right) \cdot \left(\mathtt{0x2} + -\left(A_{4}\right)\right)\right) \cdot \left(\mathtt{0x3} + -\left(A_{4}\right)\right)\right) \cdot \left(\mathtt{0x4} + -\left(A_{4}\right)\right)\right) \cdot \left(\mathtt{0x5} + -\left(A_{4}\right)\right)\right) \cdot \left(\mathtt{0x6} + -\left(A_{4}\right)\right)\right) \cdot \left(\mathtt{0x7} + -\left(A_{4}\right)\right) = 0
$$

## Gate 20. `Short fixed-base mul gate`

_Source: EccChip / utilities (halo2_gadgets). 4 constraints._

### `last_window_check` (selector $q_{20}$)

$$
\left(A_{5}\right) \cdot \left(\mathtt{0x1} + -\left(A_{5}\right)\right) = 0
$$

### `sign_check` (selector $q_{20}$)

$$
\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(\mathtt{0x1}\right) = 0
$$

### `y_check` (selector $q_{20}$)

$$
\left(A_{1} + -\left(A_{3}\right)\right) \cdot \left(A_{1} + A_{3}\right) = 0
$$

### `negation_check` (selector $q_{20}$)

$$
\left(A_{4}\right) \cdot \left(A_{1}\right) + -\left(A_{3}\right) = 0
$$

## Gate 21. `Canonicity checks`

_Source: EccChip / utilities (halo2_gadgets). 8 constraints._

### `MSB = 1 => alpha_1 = 0` (selector $q_{21}$)

$$
\left(A_{8}\right) \cdot \left(A_{7}\right) = 0
$$

### `MSB = 1 => alpha_0_hi_120 = 0` (selector $q_{21}$)

$$
\left(A_{8}\right) \cdot \left(A_{7}^{(+1)} + -\left(\left(A_{8}^{(-1)}\right) \cdot \left(\mathtt{0x100000\ldots}\right)\right)\right) = 0
$$

### `MSB = 1 => a_43 = 0 or 1` (selector $q_{21}$)

$$
\left(A_{8}\right) \cdot \left(\left(A_{8}^{(+1)} + -\left(\mathtt{0x8} \cdot \left(A_{7}^{(+1)}\right)\right)\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}^{(+1)} + -\left(\mathtt{0x8} \cdot \left(A_{7}^{(+1)}\right)\right)\right)\right)\right) = 0
$$

### `MSB = 1 => z_13_alpha_0_prime = 0` (selector $q_{21}$)

$$
\left(A_{8}\right) \cdot \left(A_{6}^{(+1)}\right) = 0
$$

### `alpha_1_range_check` (selector $q_{21}$)

$$
\left(\left(\left(A_{7}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}\right)\right)\right) \cdot \left(\mathtt{0x2} + -\left(A_{7}\right)\right)\right) \cdot \left(\mathtt{0x3} + -\left(A_{7}\right)\right) = 0
$$

### `alpha_2_range_check` (selector $q_{21}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `z_84_alpha_check` (selector $q_{21}$)

$$
A_{8}^{(-1)} + -\left(A_{7} + \mathtt{0x4} \cdot \left(A_{8}\right)\right) = 0
$$

### `alpha_0_prime check` (selector $q_{21}$)

$$
A_{6} + -\left(A_{6}^{(-1)} + -\left(\mathtt{0x100000\ldots} \cdot \left(A_{8}^{(-1)}\right)\right) + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right)\right) = 0
$$

## Gate 22. `full round`

_Source: PoseidonChip (halo2_gadgets). 3 constraints._

### `constraint 1` (selector $q_{22}$)

$$
\mathtt{0xab5e5b\ldots} \cdot \left(\left(\left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right) \cdot \left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right)\right) \cdot \left(A_{6} + F_{5}\right)\right) + \mathtt{0x319166\ldots} \cdot \left(\left(\left(\left(A_{7} + F_{6}\right) \cdot \left(A_{7} + F_{6}\right)\right) \cdot \left(\left(A_{7} + F_{6}\right) \cdot \left(A_{7} + F_{6}\right)\right)\right) \cdot \left(A_{7} + F_{6}\right)\right) + \mathtt{0x7c045d\ldots} \cdot \left(\left(\left(\left(A_{8} + F_{7}\right) \cdot \left(A_{8} + F_{7}\right)\right) \cdot \left(\left(A_{8} + F_{7}\right) \cdot \left(A_{8} + F_{7}\right)\right)\right) \cdot \left(A_{8} + F_{7}\right)\right) + -\left(A_{6}^{(+1)}\right) = 0
$$

### `constraint 2` (selector $q_{22}$)

$$
\mathtt{0x233162\ldots} \cdot \left(\left(\left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right) \cdot \left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right)\right) \cdot \left(A_{6} + F_{5}\right)\right) + \mathtt{0x25cae2\ldots} \cdot \left(\left(\left(\left(A_{7} + F_{6}\right) \cdot \left(A_{7} + F_{6}\right)\right) \cdot \left(\left(A_{7} + F_{6}\right) \cdot \left(A_{7} + F_{6}\right)\right)\right) \cdot \left(A_{7} + F_{6}\right)\right) + \mathtt{0x22f5b5\ldots} \cdot \left(\left(\left(\left(A_{8} + F_{7}\right) \cdot \left(A_{8} + F_{7}\right)\right) \cdot \left(\left(A_{8} + F_{7}\right) \cdot \left(A_{8} + F_{7}\right)\right)\right) \cdot \left(A_{8} + F_{7}\right)\right) + -\left(A_{7}^{(+1)}\right) = 0
$$

### `constraint 3` (selector $q_{22}$)

$$
\mathtt{0x2e29dd\ldots} \cdot \left(\left(\left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right) \cdot \left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right)\right) \cdot \left(A_{6} + F_{5}\right)\right) + \mathtt{0x1d1aab\ldots} \cdot \left(\left(\left(\left(A_{7} + F_{6}\right) \cdot \left(A_{7} + F_{6}\right)\right) \cdot \left(\left(A_{7} + F_{6}\right) \cdot \left(A_{7} + F_{6}\right)\right)\right) \cdot \left(A_{7} + F_{6}\right)\right) + \mathtt{0x3bf763\ldots} \cdot \left(\left(\left(\left(A_{8} + F_{7}\right) \cdot \left(A_{8} + F_{7}\right)\right) \cdot \left(\left(A_{8} + F_{7}\right) \cdot \left(A_{8} + F_{7}\right)\right)\right) \cdot \left(A_{8} + F_{7}\right)\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

## Gate 23. `partial rounds`

_Source: PoseidonChip (halo2_gadgets). 4 constraints._

### `constraint 1` (selector $q_{23}$)

$$
\left(\left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right) \cdot \left(\left(A_{6} + F_{5}\right) \cdot \left(A_{6} + F_{5}\right)\right)\right) \cdot \left(A_{6} + F_{5}\right) + -\left(A_{5}\right) = 0
$$

### `constraint 2` (selector $q_{23}$)

$$
\left(\left(\left(\mathtt{0xab5e5b\ldots} \cdot \left(A_{5}\right) + \mathtt{0x319166\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x7c045d\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{8}\right) \cdot \left(\mathtt{0xab5e5b\ldots} \cdot \left(A_{5}\right) + \mathtt{0x319166\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x7c045d\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{8}\right)\right) \cdot \left(\left(\mathtt{0xab5e5b\ldots} \cdot \left(A_{5}\right) + \mathtt{0x319166\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x7c045d\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{8}\right) \cdot \left(\mathtt{0xab5e5b\ldots} \cdot \left(A_{5}\right) + \mathtt{0x319166\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x7c045d\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{8}\right)\right)\right) \cdot \left(\mathtt{0xab5e5b\ldots} \cdot \left(A_{5}\right) + \mathtt{0x319166\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x7c045d\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{8}\right) + -\left(\mathtt{0x2cc057\ldots} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x32e7c4\ldots} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0x2eae5d\ldots} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

### `constraint 3` (selector $q_{23}$)

$$
\mathtt{0x233162\ldots} \cdot \left(A_{5}\right) + \mathtt{0x25cae2\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x22f5b5\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{9} + -\left(\mathtt{0x7bf368\ldots} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x2aec69\ldots} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0x952e02\ldots} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

### `constraint 4` (selector $q_{23}$)

$$
\mathtt{0x2e29dd\ldots} \cdot \left(A_{5}\right) + \mathtt{0x1d1aab\ldots} \cdot \left(A_{7} + F_{6}\right) + \mathtt{0x3bf763\ldots} \cdot \left(A_{8} + F_{7}\right) + F_{10} + -\left(\mathtt{0x2fcbba\ldots} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x1ec737\ldots} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0xd0c2ef\ldots} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 24. `pad-and-add`

_Source: PoseidonChip (halo2_gadgets). 3 constraints._

### `constraint 1` (selector $q_{24}$)

$$
A_{6}^{(-1)} + A_{6} + -\left(A_{6}^{(+1)}\right) = 0
$$

### `constraint 2` (selector $q_{24}$)

$$
A_{7}^{(-1)} + A_{7} + -\left(A_{7}^{(+1)}\right) = 0
$$

### `constraint 3` (selector $q_{24}$)

$$
A_{8}^{(-1)} + -\left(A_{8}^{(+1)}\right) = 0
$$

## Gate 25. `Initial y_Q`

_Source: SinsemillaChip (halo2_gadgets). 1 constraint._

### `init_y_q_check` (selector $q_{26}$)

$$
\mathtt{0x2} \cdot \left(F_{3}\right) + -\left(\left(A_{3} + A_{4}\right) \cdot \left(A_{0} + -\left(\left(A_{3}\right) \cdot \left(A_{3}\right) + -\left(A_{0}\right) + -\left(A_{1}\right)\right)\right)\right) = 0
$$

## Gate 26. `Sinsemilla gate`

_Source: SinsemillaChip (halo2_gadgets). 2 constraints._

### `Secant line` (selector $q_{25}$)

$$
\left(A_{4}\right) \cdot \left(A_{4}\right) + -\left(A_{0}^{(+1)} + \left(A_{3}\right) \cdot \left(A_{3}\right) + -\left(A_{0}\right) + -\left(A_{1}\right) + A_{0}\right) = 0
$$

### `y check` (selector $q_{25}$)

$$
\left(\mathtt{0x4} \cdot \left(A_{4}\right)\right) \cdot \left(A_{0} + -\left(A_{0}^{(+1)}\right)\right) + -\left(\mathtt{0x2} \cdot \left(\left(A_{3} + A_{4}\right) \cdot \left(A_{0} + -\left(\left(A_{3}\right) \cdot \left(A_{3}\right) + -\left(A_{0}\right) + -\left(A_{1}\right)\right)\right)\right) + \left(\mathtt{0x2} + -\left(\left(F_{12}\right) \cdot \left(F_{12} + -\left(\mathtt{0x1}\right)\right)\right)\right) \cdot \left(\left(A_{3}^{(+1)} + A_{4}^{(+1)}\right) \cdot \left(A_{0}^{(+1)} + -\left(\left(A_{3}^{(+1)}\right) \cdot \left(A_{3}^{(+1)}\right) + -\left(A_{0}^{(+1)}\right) + -\left(A_{1}^{(+1)}\right)\right)\right)\right) + \left(\mathtt{0x2} \cdot \left(\left(F_{12}\right) \cdot \left(F_{12} + -\left(\mathtt{0x1}\right)\right)\right)\right) \cdot \left(A_{3}^{(+1)}\right)\right) = 0
$$

## Gate 27. `a' = b ⋅ swap + a ⋅ (1-swap)`

_Source: MerkleChip (halo2_gadgets). 3 constraints._

### `a check` (selector $q_{27}$)

$$
A_{2} + -\left(\left(A_{4}\right) \cdot \left(A_{1}\right) + \left(\mathtt{0x1} + -\left(A_{4}\right)\right) \cdot \left(A_{0}\right)\right) = 0
$$

### `b check` (selector $q_{27}$)

$$
A_{3} + -\left(\left(A_{4}\right) \cdot \left(A_{0}\right) + \left(\mathtt{0x1} + -\left(A_{4}\right)\right) \cdot \left(A_{1}\right)\right) = 0
$$

### `swap is bool` (selector $q_{27}$)

$$
\left(A_{4}\right) \cdot \left(\mathtt{0x1} + -\left(A_{4}\right)\right) = 0
$$

## Gate 28. `Decomposition check`

_Source: MerkleChip (halo2_gadgets). 4 constraints._

### `l_check` (selector $q_{28}$)

$$
A_{0} + -\left(\mathtt{0x400} \cdot \left(A_{0}^{(+1)}\right)\right) + -\left(A_{4}^{(+1)}\right) = 0
$$

### `left_check` (selector $q_{28}$)

$$
A_{0}^{(+1)} + \mathtt{0x100000\ldots} \cdot \left(A_{1} + -\left(\mathtt{0x400} \cdot \left(A_{1}^{(+1)}\right)\right) + \mathtt{0x400} \cdot \left(A_{2}^{(+1)}\right)\right) + -\left(A_{3}\right) = 0
$$

### `right_check` (selector $q_{28}$)

$$
A_{3}^{(+1)} + \mathtt{0x20} \cdot \left(A_{2}\right) + -\left(A_{4}\right) = 0
$$

### `b1_b2_check` (selector $q_{28}$)

$$
A_{1}^{(+1)} + -\left(A_{2}^{(+1)} + \mathtt{0x20} \cdot \left(A_{3}^{(+1)}\right)\right) = 0
$$

## Gate 29. `Initial y_Q`

_Source: SinsemillaChip (halo2_gadgets). 1 constraint._

### `init_y_q_check` (selector $q_{30}$)

$$
\mathtt{0x2} \cdot \left(F_{4}\right) + -\left(\left(A_{8} + A_{9}\right) \cdot \left(A_{5} + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{5}\right) + -\left(A_{6}\right)\right)\right)\right) = 0
$$

## Gate 30. `Sinsemilla gate`

_Source: SinsemillaChip (halo2_gadgets). 2 constraints._

### `Secant line` (selector $q_{29}$)

$$
\left(A_{9}\right) \cdot \left(A_{9}\right) + -\left(A_{5}^{(+1)} + \left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{5}\right) + -\left(A_{6}\right) + A_{5}\right) = 0
$$

### `y check` (selector $q_{29}$)

$$
\left(\mathtt{0x4} \cdot \left(A_{9}\right)\right) \cdot \left(A_{5} + -\left(A_{5}^{(+1)}\right)\right) + -\left(\mathtt{0x2} \cdot \left(\left(A_{8} + A_{9}\right) \cdot \left(A_{5} + -\left(\left(A_{8}\right) \cdot \left(A_{8}\right) + -\left(A_{5}\right) + -\left(A_{6}\right)\right)\right)\right) + \left(\mathtt{0x2} + -\left(\left(F_{13}\right) \cdot \left(F_{13} + -\left(\mathtt{0x1}\right)\right)\right)\right) \cdot \left(\left(A_{8}^{(+1)} + A_{9}^{(+1)}\right) \cdot \left(A_{5}^{(+1)} + -\left(\left(A_{8}^{(+1)}\right) \cdot \left(A_{8}^{(+1)}\right) + -\left(A_{5}^{(+1)}\right) + -\left(A_{6}^{(+1)}\right)\right)\right)\right) + \left(\mathtt{0x2} \cdot \left(\left(F_{13}\right) \cdot \left(F_{13} + -\left(\mathtt{0x1}\right)\right)\right)\right) \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 31. `a' = b ⋅ swap + a ⋅ (1-swap)`

_Source: MerkleChip (halo2_gadgets). 3 constraints._

### `a check` (selector $q_{31}$)

$$
A_{7} + -\left(\left(A_{9}\right) \cdot \left(A_{6}\right) + \left(\mathtt{0x1} + -\left(A_{9}\right)\right) \cdot \left(A_{5}\right)\right) = 0
$$

### `b check` (selector $q_{31}$)

$$
A_{8} + -\left(\left(A_{9}\right) \cdot \left(A_{5}\right) + \left(\mathtt{0x1} + -\left(A_{9}\right)\right) \cdot \left(A_{6}\right)\right) = 0
$$

### `swap is bool` (selector $q_{31}$)

$$
\left(A_{9}\right) \cdot \left(\mathtt{0x1} + -\left(A_{9}\right)\right) = 0
$$

## Gate 32. `Decomposition check`

_Source: MerkleChip (halo2_gadgets). 4 constraints._

### `l_check` (selector $q_{32}$)

$$
A_{5} + -\left(\mathtt{0x400} \cdot \left(A_{5}^{(+1)}\right)\right) + -\left(A_{9}^{(+1)}\right) = 0
$$

### `left_check` (selector $q_{32}$)

$$
A_{5}^{(+1)} + \mathtt{0x100000\ldots} \cdot \left(A_{6} + -\left(\mathtt{0x400} \cdot \left(A_{6}^{(+1)}\right)\right) + \mathtt{0x400} \cdot \left(A_{7}^{(+1)}\right)\right) + -\left(A_{8}\right) = 0
$$

### `right_check` (selector $q_{32}$)

$$
A_{8}^{(+1)} + \mathtt{0x20} \cdot \left(A_{7}\right) + -\left(A_{9}\right) = 0
$$

### `b1_b2_check` (selector $q_{32}$)

$$
A_{6}^{(+1)} + -\left(A_{7}^{(+1)} + \mathtt{0x20} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 33. `CommitIvk canonicity check`

_Source: CommitIvkChip (src/circuit/commit_ivk.rs). 14 constraints._

### `b1_bool_check` (selector $q_{33}$)

$$
\left(A_{4}\right) \cdot \left(\mathtt{0x1} + -\left(A_{4}\right)\right) = 0
$$

### `d1_bool_check` (selector $q_{33}$)

$$
\left(A_{4}^{(+1)}\right) \cdot \left(\mathtt{0x1} + -\left(A_{4}^{(+1)}\right)\right) = 0
$$

### `b_decomposition_check` (selector $q_{33}$)

$$
A_{2} + -\left(A_{3} + \mathtt{0x10} \cdot \left(A_{4}\right) + \mathtt{0x20} \cdot \left(A_{5}\right)\right) = 0
$$

### `d_decomposition_check` (selector $q_{33}$)

$$
A_{2}^{(+1)} + -\left(A_{3}^{(+1)} + \mathtt{0x200} \cdot \left(A_{4}^{(+1)}\right)\right) = 0
$$

### `ak_decomposition_check` (selector $q_{33}$)

$$
A_{1} + \mathtt{0x400000\ldots} \cdot \left(A_{3}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{4}\right) + -\left(A_{0}\right) = 0
$$

### `nk_decomposition_check` (selector $q_{33}$)

$$
A_{5} + \mathtt{0x20} \cdot \left(A_{1}^{(+1)}\right) + \mathtt{0x200000\ldots} \cdot \left(A_{3}^{(+1)}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{4}^{(+1)}\right) + -\left(A_{0}^{(+1)}\right) = 0
$$

### `b0_canon_check` (selector $q_{33}$)

$$
\left(A_{4}\right) \cdot \left(A_{3}\right) = 0
$$

### `z13_a_check` (selector $q_{33}$)

$$
\left(A_{4}\right) \cdot \left(A_{6}\right) = 0
$$

### `a_prime_check` (selector $q_{33}$)

$$
A_{1} + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{7}\right) = 0
$$

### `z13_a_prime` (selector $q_{33}$)

$$
\left(A_{4}\right) \cdot \left(A_{8}\right) = 0
$$

### `c0_canon_check` (selector $q_{33}$)

$$
\left(A_{4}^{(+1)}\right) \cdot \left(A_{3}^{(+1)}\right) = 0
$$

### `z13_c_check` (selector $q_{33}$)

$$
\left(A_{4}^{(+1)}\right) \cdot \left(A_{6}^{(+1)}\right) = 0
$$

### `b2_c_prime_check` (selector $q_{33}$)

$$
A_{5} + \mathtt{0x20} \cdot \left(A_{1}^{(+1)}\right) + \mathtt{0x100000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{7}^{(+1)}\right) = 0
$$

### `z14_b2_c_prime` (selector $q_{33}$)

$$
\left(A_{4}^{(+1)}\right) \cdot \left(A_{8}^{(+1)}\right) = 0
$$

## Gate 34. `NoteCommit MessagePiece b`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 3 constraints._

### `bool_check b_1` (selector $q_{34}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `bool_check b_2` (selector $q_{34}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}^{(+1)}\right)\right) = 0
$$

### `decomposition` (selector $q_{34}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x20} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0x40} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 35. `NoteCommit MessagePiece d`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 3 constraints._

### `bool_check d_0` (selector $q_{35}$)

$$
\left(A_{7}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}\right)\right) = 0
$$

### `bool_check d_1` (selector $q_{35}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `decomposition` (selector $q_{35}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x2} \cdot \left(A_{8}\right) + \mathtt{0x4} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0x400} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 36. `NoteCommit MessagePiece e`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 1 constraint._

### `decomposition` (selector $q_{36}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x40} \cdot \left(A_{8}\right)\right) = 0
$$

## Gate 37. `NoteCommit MessagePiece g`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 2 constraints._

### `bool_check g_0` (selector $q_{37}$)

$$
\left(A_{7}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}\right)\right) = 0
$$

### `decomposition` (selector $q_{37}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x2} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x400} \cdot \left(A_{7}^{(+1)}\right)\right) = 0
$$

## Gate 38. `NoteCommit MessagePiece h`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 2 constraints._

### `bool_check h_1` (selector $q_{38}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `decomposition` (selector $q_{38}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x20} \cdot \left(A_{8}\right)\right) = 0
$$

## Gate 39. `NoteCommit input g_d`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 5 constraints._

### `decomposition` (selector $q_{39}$)

$$
A_{8} + \mathtt{0x400000\ldots} \cdot \left(A_{7}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `a_prime_check` (selector $q_{39}$)

$$
A_{8} + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `b_1 = 1 => b_0` (selector $q_{39}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{7}\right) = 0
$$

### `b_1 = 1 => z13_a` (selector $q_{39}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `b_1 = 1 => z13_a_prime` (selector $q_{39}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 40. `NoteCommit input pk_d`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 4 constraints._

### `decomposition` (selector $q_{40}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `b3_c_prime_check` (selector $q_{40}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x100000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `d_0 = 1 => z13_c` (selector $q_{40}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `d_0 = 1 => z14_b3_c_prime` (selector $q_{40}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 41. `NoteCommit input value`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 1 constraint._

### `value_check` (selector $q_{41}$)

$$
A_{7} + \mathtt{0x100} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{9}\right) + -\left(A_{6}\right) = 0
$$

## Gate 42. `NoteCommit input rho`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 4 constraints._

### `decomposition` (selector $q_{42}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `e1_f_prime_check` (selector $q_{42}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x100000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `g_0 = 1 => z13_f` (selector $q_{42}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `g_0 = 1 => z14_e1_f_prime` (selector $q_{42}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 43. `NoteCommit input psi`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 5 constraints._

### `decomposition` (selector $q_{43}$)

$$
A_{7} + \mathtt{0x200} \cdot \left(A_{8}\right) + \mathtt{0x200000\ldots} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `g1_g2_prime_check` (selector $q_{43}$)

$$
A_{7} + \mathtt{0x200} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `h_1 = 1 => h_0` (selector $q_{43}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{6}^{(+1)}\right) = 0
$$

### `h_1 = 1 => z13_g` (selector $q_{43}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `h_1 = 1 => z13_g1_g2_prime` (selector $q_{43}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 44. `y coordinate checks`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 7 constraints._

### `k3_check` (selector $q_{44}$)

$$
\left(A_{9}\right) \cdot \left(\mathtt{0x1} + -\left(A_{9}\right)\right) = 0
$$

### `j_check` (selector $q_{44}$)

$$
A_{5}^{(+1)} + -\left(A_{6} + \mathtt{0x2} \cdot \left(A_{7}\right) + \mathtt{0x400} \cdot \left(A_{6}^{(+1)}\right)\right) = 0
$$

### `y_check` (selector $q_{44}$)

$$
A_{5} + -\left(A_{5}^{(+1)} + \mathtt{0x400000\ldots} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{9}\right)\right) = 0
$$

### `j_prime_check` (selector $q_{44}$)

$$
A_{5}^{(+1)} + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `k_3 = 1 => k_2 = 0` (selector $q_{44}$)

$$
\left(A_{9}\right) \cdot \left(A_{8}\right) = 0
$$

### `k_3 = 1 => z13_j = 0` (selector $q_{44}$)

$$
\left(A_{9}\right) \cdot \left(A_{7}^{(+1)}\right) = 0
$$

### `k_3 = 1 => z13_j_prime = 0` (selector $q_{44}$)

$$
\left(A_{9}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 45. `NoteCommit MessagePiece b`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 3 constraints._

### `bool_check b_1` (selector $q_{45}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `bool_check b_2` (selector $q_{45}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}^{(+1)}\right)\right) = 0
$$

### `decomposition` (selector $q_{45}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x20} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0x40} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 46. `NoteCommit MessagePiece d`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 3 constraints._

### `bool_check d_0` (selector $q_{46}$)

$$
\left(A_{7}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}\right)\right) = 0
$$

### `bool_check d_1` (selector $q_{46}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `decomposition` (selector $q_{46}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x2} \cdot \left(A_{8}\right) + \mathtt{0x4} \cdot \left(A_{7}^{(+1)}\right) + \mathtt{0x400} \cdot \left(A_{8}^{(+1)}\right)\right) = 0
$$

## Gate 47. `NoteCommit MessagePiece e`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 1 constraint._

### `decomposition` (selector $q_{47}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x40} \cdot \left(A_{8}\right)\right) = 0
$$

## Gate 48. `NoteCommit MessagePiece g`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 2 constraints._

### `bool_check g_0` (selector $q_{48}$)

$$
\left(A_{7}\right) \cdot \left(\mathtt{0x1} + -\left(A_{7}\right)\right) = 0
$$

### `decomposition` (selector $q_{48}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x2} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x400} \cdot \left(A_{7}^{(+1)}\right)\right) = 0
$$

## Gate 49. `NoteCommit MessagePiece h`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 2 constraints._

### `bool_check h_1` (selector $q_{49}$)

$$
\left(A_{8}\right) \cdot \left(\mathtt{0x1} + -\left(A_{8}\right)\right) = 0
$$

### `decomposition` (selector $q_{49}$)

$$
A_{6} + -\left(A_{7} + \mathtt{0x20} \cdot \left(A_{8}\right)\right) = 0
$$

## Gate 50. `NoteCommit input g_d`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 5 constraints._

### `decomposition` (selector $q_{50}$)

$$
A_{8} + \mathtt{0x400000\ldots} \cdot \left(A_{7}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `a_prime_check` (selector $q_{50}$)

$$
A_{8} + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `b_1 = 1 => b_0` (selector $q_{50}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{7}\right) = 0
$$

### `b_1 = 1 => z13_a` (selector $q_{50}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `b_1 = 1 => z13_a_prime` (selector $q_{50}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 51. `NoteCommit input pk_d`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 4 constraints._

### `decomposition` (selector $q_{51}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `b3_c_prime_check` (selector $q_{51}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x100000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `d_0 = 1 => z13_c` (selector $q_{51}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `d_0 = 1 => z14_b3_c_prime` (selector $q_{51}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 52. `NoteCommit input value`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 1 constraint._

### `value_check` (selector $q_{52}$)

$$
A_{7} + \mathtt{0x100} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{9}\right) + -\left(A_{6}\right) = 0
$$

## Gate 53. `NoteCommit input rho`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 4 constraints._

### `decomposition` (selector $q_{53}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `e1_f_prime_check` (selector $q_{53}$)

$$
A_{7} + \mathtt{0x10} \cdot \left(A_{8}\right) + \mathtt{0x100000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `g_0 = 1 => z13_f` (selector $q_{53}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `g_0 = 1 => z14_e1_f_prime` (selector $q_{53}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 54. `NoteCommit input psi`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 5 constraints._

### `decomposition` (selector $q_{54}$)

$$
A_{7} + \mathtt{0x200} \cdot \left(A_{8}\right) + \mathtt{0x200000\ldots} \cdot \left(A_{6}^{(+1)}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{7}^{(+1)}\right) + -\left(A_{6}\right) = 0
$$

### `g1_g2_prime_check` (selector $q_{54}$)

$$
A_{7} + \mathtt{0x200} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `h_1 = 1 => h_0` (selector $q_{54}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{6}^{(+1)}\right) = 0
$$

### `h_1 = 1 => z13_g` (selector $q_{54}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}\right) = 0
$$

### `h_1 = 1 => z13_g1_g2_prime` (selector $q_{54}$)

$$
\left(A_{7}^{(+1)}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$

## Gate 55. `y coordinate checks`

_Source: NoteCommitChip (src/circuit/note_commit.rs). 7 constraints._

### `k3_check` (selector $q_{55}$)

$$
\left(A_{9}\right) \cdot \left(\mathtt{0x1} + -\left(A_{9}\right)\right) = 0
$$

### `j_check` (selector $q_{55}$)

$$
A_{5}^{(+1)} + -\left(A_{6} + \mathtt{0x2} \cdot \left(A_{7}\right) + \mathtt{0x400} \cdot \left(A_{6}^{(+1)}\right)\right) = 0
$$

### `y_check` (selector $q_{55}$)

$$
A_{5} + -\left(A_{5}^{(+1)} + \mathtt{0x400000\ldots} \cdot \left(A_{8}\right) + \mathtt{0x400000\ldots} \cdot \left(A_{9}\right)\right) = 0
$$

### `j_prime_check` (selector $q_{55}$)

$$
A_{5}^{(+1)} + \mathtt{0x400000\ldots} + -\left(\mathtt{0x224698\ldots}\right) + -\left(A_{8}^{(+1)}\right) = 0
$$

### `k_3 = 1 => k_2 = 0` (selector $q_{55}$)

$$
\left(A_{9}\right) \cdot \left(A_{8}\right) = 0
$$

### `k_3 = 1 => z13_j = 0` (selector $q_{55}$)

$$
\left(A_{9}\right) \cdot \left(A_{7}^{(+1)}\right) = 0
$$

### `k_3 = 1 => z13_j_prime = 0` (selector $q_{55}$)

$$
\left(A_{9}\right) \cdot \left(A_{9}^{(+1)}\right) = 0
$$
