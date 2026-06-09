---
sidebar_position: 17
title: Audits and Historical Issues
description: Public audit reports of Orchard, Halo 2, and Pasta, with recurring themes.
---

# Audits and Historical Issues

## 1. Why This Chapter Exists

The Orchard implementation, Halo 2, and the Pasta curves were
audited publicly before NU5 activated in 2022. A contributor
working in audit-sensitive code (the circuit, the curve
arithmetic, the key derivation) should read the relevant report
before opening a PR. After this chapter the reader knows where to
find each report and what classes of finding to expect.

## 2. Definitions

### Definition 2.1 (Audit-Point)

A tagged commit submitted to an external audit firm. The git
history of `zcash/orchard` carries branches named with
`-auditpoint` suffixes that mark the snapshots reviewed by each
firm.

### Definition 2.2 (Recurring Finding Category)

A class of vulnerability that appears across multiple reports.
The five categories that recur in the Orchard / Halo 2 reports
are listed in Section 4 below.

## 3. The Code

### 3.1 Audit Reports

The four publicly available reports most relevant to the crate:

- 2021,
  [Trail of Bits Halo 2 audit (PDF)](https://www.trailofbits.com/reports/Halo2.pdf):
  `halo2_proofs` and `halo2_gadgets`.
- 2021,
  [NCC Group Halo 2 audit (PDF)](https://research.nccgroup.com/wp-content/uploads/2022/03/NCC_Group_ZcashFoundation_E001950_Report_2021-12-15.pdf):
  same scope, independent firm.
- 2022,
  [Least Authority Orchard / Pasta audit (PDF)](https://leastauthority.com/static/publications/LeastAuthority_Zcash_Orchard_Pasta_Final_Audit_Report.pdf):
  the Orchard crate, Sinsemilla, and the Pasta curves.
- 2022, QEDIT Action circuit audit (linked from the
  [Orchard Book audit page](https://zcash.github.io/halo2/audits.html)).

### 3.2 Audit Page

The
[Halo 2 audit overview](https://zcash.github.io/halo2/audits.html)
links every report with a date and a scope. EC Co.'s
[blog post on the Orchard audit](https://electriccoin.co/blog/audit-of-zcash-orchard-pallas/)
summarises the findings.

### 3.3 Tracing Fixes

Most fixes are visible in the per-release sections of
[`CHANGELOG.md`](https://github.com/zcash/orchard/blob/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/CHANGELOG.md)
and in the matching releases of `halo2_proofs` and
`pasta_curves`. Commits between an audit-point branch and the
next release are the audit-response commits.

### 3.4 The 0.14.0 / NU6.2 Security Fixes (June 2026)

`orchard` 0.14.0 (2026-06-02) fixes two distinct, recently
disclosed problems. Both land after the
[`0.13.1`](https://github.com/zcash/orchard/releases/tag/0.13.1)
tag this course otherwise pins to, and both are recorded in the
[0.14.0 CHANGELOG](https://github.com/zcash/orchard/blob/0.14.0/CHANGELOG.md).

1. **Under-constrained EC-multiplication gadget (critical, value
   forgery).** A soundness bug in the `halo2_gadgets`
   elliptic-curve multiplication gadget left a constraint gap
   that let false inputs pass the multiplication check. Because
   the Action circuit (Chapter 5) relies on that gadget, the gap
   could in principle be used to build a valid-looking proof that
   spends Orchard value that never existed, that is, to forge ZEC
   inside the shielded pool, with nothing in the proof for a
   verifier to catch. The flaw affected all `halo2_gadgets`
   before `0.5.0` and all `orchard` before `0.14.0`, was
   disclosed on 2026-06-05 after an AI-assisted audit, and was
   patched in the field by the NU6.2 network upgrade. In the
   crate the fix appears as the new circuit-version split
   `OrchardCircuitVersion::{InsecurePreNu6_2, FixedPostNu6_2}`:
   `ProvingKey::build()` and `VerifyingKey::build()` now build the
   fixed post-NU6.2 circuit, and the insecure pre-NU6.2 circuit
   is reachable only through the explicit `*_for_version`
   constructors. Details are in the
   [`halo2_gadgets 0.5.0`](https://github.com/zcash/halo2/security/advisories)
   release notes.

2. **Non-canonical proof size and malformed `epk`
   ([GHSA-2x4w-pxqw-58v9](https://github.com/zcash/orchard/security/advisories/GHSA-2x4w-pxqw-58v9)).**
   Two malleability bugs: an authorized `Bundle` or a PCZT could
   carry a `zkproof` padded with arbitrary trailing bytes (a
   non-canonical proof length), and an `Action` could be built
   with an ephemeral key `epk` that does not encode a
   non-identity Pallas point. The fix makes
   `Bundle::<Authorized, V>::try_from_parts` the only way to
   construct an authorized bundle and rejects any proof whose
   length is not `Proof::expected_proof_size` for the action
   count; `Action::from_parts` now returns
   `Result<_, ActionFromPartsError>`, and the PCZT Transaction
   Extractor rejects both conditions through the new
   `TxExtractorError::{NonCanonicalProofSize, InvalidEpk}`
   variants.

Mapped onto Section 4: finding 1 is a witness- and
constraint-completeness failure (the _incomplete-addition_ and
_witness canonicality_ families), and finding 2 is
consensus-relevant constructor laxity, the same family as the
identity-`rk` rejection in
[#492](https://github.com/zcash/orchard/pull/492).

## 4. Failure Modes

Five recurring categories drive the audit findings. A
contributor touching code in any of these areas should expect
extra scrutiny.

- **Incomplete-addition reachability**. Sinsemilla and the ECC
  chip use chord-and-tangent addition that is undefined on equal
  or opposite inputs. Every new domain or generator choice needs
  the unreachability argument (Halo 2 Book, Section on
  incomplete addition).
- **Witness canonicality**. The prover must encode field
  elements into bits in exactly one way. Missing range checks in
  [`src/circuit/note_commit.rs`](https://github.com/zcash/orchard/blob/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/src/circuit/note_commit.rs)
  or
  [`src/circuit/commit_ivk.rs`](https://github.com/zcash/orchard/blob/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/src/circuit/commit_ivk.rs)
  is the textbook ambiguity bug.
- **Domain separation**. Sinsemilla, Poseidon, RedDSA, and the
  transcript all use distinct personalisation strings. Any new
  domain must be checked for collision against all existing
  domains.
- **Side-channels in `pasta_curves`**. Constant-time inversion
  and scalar multiplication were reviewed in
  `pasta_curves`. Bypassing the constant-time helpers (via
  `bytes` and manual reduction) reintroduces the risk.
- **Consensus-relevant constructor laxity**. Identity-valued
  `rk`, `ak`, or `cv` are now rejected at construction
  ([#492](https://github.com/zcash/orchard/pull/492)). A
  similar tightening for other types may be warranted; new
  types in `src/keys.rs` deserve a defensive constructor.

## 5. Spec Pointers

- [Halo 2 audit overview](https://zcash.github.io/halo2/audits.html).
- [Electric Coin Co., audit of Zcash Orchard and Pallas](https://electriccoin.co/blog/audit-of-zcash-orchard-pallas/).
- [`zcash/orchard` CHANGELOG.md](https://github.com/zcash/orchard/blob/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/CHANGELOG.md):
  the per-release commit-attributable changes.
- [RUSTSEC](https://rustsec.org/):
  for upstream advisories on the dependency surface.

## 6. Exercises

1. Pick one finding from the Least Authority report and locate
   the corresponding fix in this crate or in `zcash/halo2`.
   Cite the commit SHA.
2. The incomplete-addition argument lives in the
   [Halo 2 Book](https://zcash.github.io/halo2/design/gadgets/ecc/addition.html).
   Summarise in three sentences how Sinsemilla avoids the
   failure case.
3. **Code task**. Read the unit test that exercises
   [`Action::from_parts`](https://github.com/zcash/orchard/blob/f8915bc5c8d1c9fa3124ad28bcf73ce232ef3669/src/action.rs#L45) after
   [#492](https://github.com/zcash/orchard/pull/492). Adapt it
   to cover a related rejection (e.g. an identity-valued
   `nf`) and confirm that the new test passes with the current
   constructor.

## 7. Further Reading

- The git branches named `*-auditpoint` mark the exact snapshot
  each firm reviewed; checkout one and diff against the next
  release to see the response.
- [The Halo 2 audits page](https://zcash.github.io/halo2/audits.html)
  is updated as new audits land.
