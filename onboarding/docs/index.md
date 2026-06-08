---
sidebar_position: 0
slug: /
title: Orchard Onboarding Course
description: A graduate-level reading course on the zcash/orchard crate, pinned to tag 0.13.1.
---

# Orchard Onboarding Course

This site is a graduate-level, contribution-oriented reading course
on the [`zcash/orchard`](https://github.com/zcash/orchard) crate.
Each chapter pairs the math behind a piece of the Orchard shielded
protocol with the matching Rust code, the relevant specification,
and exercises that force the reader to touch the source. The
intended outcome is concrete: read the course, then open a real PR
against the crate.

:::warning Automatically Generated Content

This site is automatically generated using
[Claude Code](https://www.claude.com/product/claude-code). Errors
may have been introduced. **This site is not authoritative
documentation or explanation of the Orchard shielded protocol.**
The only authoritative material is what is published by the
Zcash-related organisations that maintain the protocol and its
reference implementations, in particular the
[ZIPs](https://zips.z.cash/) and the
[Zcash Protocol Specification](https://zips.z.cash/protocol/protocol.pdf).
Anything on this site that disagrees with those sources is wrong
on this site, not in them.

Concretely, the authoritative references for everything that
follows are:

1. The source files in
   [`zcash/orchard`](https://github.com/zcash/orchard/tree/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/src),
   pinned to tag
   [`0.13.1`](https://github.com/zcash/orchard/releases/tag/0.13.1).
2. The
   [Zcash Protocol Specification](https://zips.z.cash/protocol/protocol.pdf)
   (Section 4 covers shielded payment components and Section 5.4
   covers Orchard).
3. The relevant
   [ZIPs](https://zips.z.cash/), in particular
   [ZIP 32](https://zips.z.cash/zip-0032),
   [ZIP 212](https://zips.z.cash/zip-0212),
   [ZIP 224](https://zips.z.cash/zip-0224),
   [ZIP 244](https://zips.z.cash/zip-0244), and
   [ZIP 316](https://zips.z.cash/zip-0316).

To file a correction, open an issue or a pull request on the
onboarding branch of the fork:
[dannywillems/orchard `onboarding`](https://github.com/dannywillems/orchard/tree/onboarding).

:::

## What Orchard Does

Orchard is Zcash's shielded-payment protocol. It lets one party
transfer value to another while hiding the sender, the recipient,
and the amount from every chain observer, and still lets consensus
verify that no value was created and that no note was spent twice.
It is the third Zcash shielded pool, after Sprout (2016) and
Sapling (2018);
[Background: From Zerocoin to Orchard](./protocol-lineage.md)
covers the lineage and what each generation changed.

Five objects carry the whole protocol. Each has its own chapter;
this is the one-line version, expanded in the
[shielded-pool vocabulary](./protocol-lineage.md#4-the-shielded-pool-vocabulary).

- **Note**: a shielded coin, the analogue of a Bitcoin UTXO,
  holding an amount, the recipient's key material, and
  randomness. The chain never sees the note itself, only its
  **commitment** ([Chapter 9](./09-notes-nullifiers-commitments.md)).
  For the full Bitcoin-transaction analogy see
  [Chapter 12, Section 1.1](./12-bundle-and-builder.md#11-relation-to-the-bitcoin-utxo-model).
- **Note commitment tree / anchor**: every note commitment is
  appended as a leaf to a fixed-depth (32) Merkle tree. The root
  at a recent block height is the **anchor**, the public value a
  spend proves membership against
  ([Chapter 11](./11-merkle-tree.md)).
- **Nullifier**: spending a note publishes a **nullifier**
  $\mathsf{nf}$, deterministically derived from the note and the
  owner's nullifier key. Each note has exactly one nullifier, so
  the chain rejects a previously-seen one as a double-spend;
  observers cannot link a nullifier back to its commitment
  ([Chapter 9](./09-notes-nullifiers-commitments.md)).
- **Value commitment**: amounts hide inside additively
  homomorphic Pedersen commitments $\mathsf{cv^{\mathsf{net}}}$.
  The homomorphism lets a verifier check that inputs minus
  outputs equal the declared balance without learning any
  individual amount ([Chapter 13](./13-value-commitments.md)).
- **Action**: Orchard's atomic primitive, fusing one note spend
  and one note output into a single on-chain description and a
  single circuit. Sapling kept the two apart
  (`SpendDescription` and `OutputDescription`); fusing them both
  saves circuit work and hides whether either side is real
  ([Chapter 5](./05-action-circuit.md),
  [Chapter 12](./12-bundle-and-builder.md)).

## What the Action Circuit Actually Proves

Sapling has two circuits, `Spend` and `Output`, and carries one
proof per spent note and one per created note. Orchard fuses them
into a single **Action** circuit and carries **one Halo 2 proof
per bundle**, covering every Action in the transaction at once.
[Chapter 5](./05-action-circuit.md) is the full treatment; this
is the summary.

For each Action, the circuit proves, in zero knowledge, the
**conjunction** of the following, without revealing the secrets
that make them true:

1. **Membership** (spend side): the spent note's commitment
   $\mathsf{cm}_{\mathsf{old}}$ is a leaf of the tree at the
   public anchor. The Merkle path is private, so the proof
   asserts "some note" without revealing which.
2. **Spend authority**: the prover knows the spend-authorising
   key behind the validating key $\mathsf{ak}$. The public
   randomised key $\mathsf{rk}$ is a fresh re-randomisation of
   $\mathsf{ak}$, tying the proof to the spend signature checked
   outside the circuit.
3. **Nullifier integrity**: the public nullifier $\mathsf{nf}$
   is exactly the specification's nullifier of the old note under
   the owner's nullifier key $\mathsf{nk}$.
4. **Output well-formedness**: a fresh note exists whose
   commitment $\mathsf{cm}^\star_{\mathsf{new}}$ is public and
   inserted into the tree; the recipient and value stay hidden.
5. **Value consistency**: the public net value commitment
   satisfies $\mathsf{cv^{\mathsf{net}}} = [v_{\mathsf{old}} -
   v_{\mathsf{new}}]\, \mathcal{V} + [\mathsf{rcv}]\, \mathcal{R}$.
6. **Enable flags**: the public `enableSpends` and
   `enableOutputs` switch off the spend or output side when the
   Action is a dummy, so a bundle can be padded to a power-of-two
   Action count without leaking the real spend or output count.

Notably **absent** from the circuit: bundle-wide balance. That is
checked outside the SNARK, by the binding signature over the sum
of the value commitments
([Chapter 13](./13-value-commitments.md)). Because spend and
output share one description and the enable flags hide dummies,
an external observer cannot tell, from the shape of a bundle,
whether any given Action is a real spend, a real output, both, or
two dummies.

## Relationship to the Zcash Protocol Specification

The
[Zcash Protocol Specification](https://zips.z.cash/protocol/protocol.pdf)
is the canonical, normative reference for everything on this site.
It is maintained by the protocol team, updated continuously as
ZIPs are accepted, and re-released as a new PDF revision on every
substantive change. It already contains every definition,
constant, parameter, and pseudocode block that the chapters here
rely on, in far more detail and with the formality required for
consensus.

Most of the prose on this site is, by design, a paraphrase of the
specification. The reason for having a second presentation at all
is operational, not editorial:

- **HTML, not PDF.** Cross-references, full-text search, and
  responsive layout make the material easier to navigate on a
  laptop while reading code.
- **Links to the source.** Every definition that has a Rust
  counterpart links to the file, line, and pinned commit in
  `zcash/orchard` (and to `librustzcash`, `zebrad`, or `zcashd`
  where relevant). The PDF cannot do this without losing its
  pin-stability.
- **Lives next to the code.** This site is generated from
  Markdown checked in to the `onboarding` branch of the fork, so
  a change in the crate can be reflected in the same PR as a
  change in the prose.

If a statement on this site conflicts with the PDF, the PDF
wins. When in doubt, search the PDF first; this site is the
contributor-oriented index into it, not a replacement.

## How to Read This Course

Chapters are numbered `NN-slug.md` so the sidebar matches the
reading order. Every chapter follows the same seven-section
skeleton:

1. **Why This Chapter Exists**: the question the chapter answers
   and the file or function the reader will touch by the end.
2. **Definitions**: formal definitions (KaTeX for math, grammars
   or state machines otherwise). Downstream chapters cite these by
   name.
3. **The Code**: a walkthrough of the implementation with
   line-anchored live source embeds, annotated with the invariant
   each block enforces.
4. **Failure Modes**: what goes wrong if a contributor changes the
   code without understanding the chapter. Audit findings, CVEs,
   or historical bugs are cited when available.
5. **Spec Pointers**: the relevant ZIPs, sections of the Zcash
   Protocol Specification, and papers, each with the reason it is
   cited.
6. **Exercises**: at least three, of which at least one requires
   the reader to modify code or add a test.
7. **Further Reading**: optional pointers for going deeper.

The [Glossary](./glossary.md) at the bottom of the sidebar is not
a chapter; it is reference material returned to during work.

## Prerequisites

The reader is assumed to be comfortable with:

- Finite field arithmetic over $\mathbb{F}_p$ and the basics of
  elliptic curves over prime fields.
- Discrete-log based cryptography: Pedersen commitments,
  Schnorr-style signatures, key exchange.
- Cryptographic hash functions and the random oracle model.
- The Rust language at the level of reading a non-trivial crate
  with `#![no_std]`.

Familiarity with PLONK or Halo 2 is helpful but not required;
[Chapter 4 (Halo 2 Primer)](./04-halo2-primer.md) introduces the
parts of PLONKish arithmetisation needed to read
[`src/circuit.rs`](https://github.com/zcash/orchard/blob/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/src/circuit.rs).

## Notation

Used throughout, following the Zcash Protocol Specification.

| Symbol                               | Meaning                                       |
| ------------------------------------ | --------------------------------------------- |
| $\mathbb{F}_p$                       | The prime field with $p$ elements             |
| $\mathbb{G}$, $E_p$                  | A cyclic group; the curve over $\mathbb{F}_p$ |
| $[k] P$                              | Scalar multiplication of $P$ by $k$           |
| $\mathsf{Com}(m; r)$                 | Commitment to $m$ with randomness $r$         |
| $x \xleftarrow{R} S$                 | Sample $x$ uniformly from $S$                 |
| $a \mathbin{\Vert} b$                | Byte-string concatenation                     |
| $\mathsf{Sinsemilla}_D(m)$           | Sinsemilla hash of $m$ in domain $D$          |
| $\mathsf{Poseidon}(x_1, \dots, x_n)$ | Poseidon permutation output                   |
| $q$, $r$                             | Base / scalar field orders                    |
| $\mathsf{Extract}_{\mathbb{P}}(P)$   | $x$-coordinate extraction                     |

## Pin

All GitHub source links are pinned to upstream tag
[`0.13.1`](https://github.com/zcash/orchard/releases/tag/0.13.1),
commit
[`f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669`](https://github.com/zcash/orchard/tree/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669).

## Chapter Index

The sidebar on the left lists every chapter in order. The intended
reading path is linear: Chapter 1 maps the crate, Chapter 2
unlocks the contribution loop, and Chapters 3 to 19 each pick one
subsystem and follow the seven-section skeleton.

Before reading chapter 1, two short front-pages set the
context:

- [Background: From Zerocoin to Orchard](./protocol-lineage.md)
  explains the lineage. Zcash implements **Zerocash** (2014),
  not **Zerocoin** (2013); Sprout, Sapling, and Orchard each
  refine the Zerocash note model and replace specific
  primitives. The page also defines the shielded-pool
  vocabulary (note, commitment, anchor, nullifier, action) that
  every later chapter uses without ceremony.
- [Names, ZIPs, Issues, and PRs](./references.md) gives the
  botanical naming convention (Sprout / Sapling / Orchard,
  formerly Pollard) and a curated index of every external
  reference the chapters cite.
