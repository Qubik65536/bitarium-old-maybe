# ADR 0001: Use C3 for the native memory and device subsystem

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Context

The first full MVP uses three host-side implementation languages. Rust provides CPU execution and application controls; OCaml provides assembly tools and a reference model. The native memory and device subsystem needs a language with explicit storage control and an interface callable from Rust.

## Alternatives

Zig and C3 were compared. C and C++ were also discussed as viable alternatives. Zig's standalone narrow integer types are convenient; C3 bitstructs can express the device layouts needed by the current proposal. Both Zig and C3 can provide a C ABI.

## Decision

Use C3 for RAM/ROM storage, address mapping, image placement, and minimal virtual devices. Its explicit storage control, bitstructs, and C ABI support fit this subsystem. No performance advantage or formal-verification guarantee is claimed by this choice.

## Consequences

- The three-language MVP is Rust + OCaml + C3.
- Future plans should target C3 for the native subsystem.
- A narrow C ABI remains the proposed Rust integration route; exact ownership, error rules, and signatures still need specification.
- C3 contracts may document and check internal requirements, but do not establish general formal correctness.
- Compiler versions, build integration, and a direct proof workflow for the C3 implementation remain open.
- This does not select the first ISA or accept the proposed memory map.

## References and acceptance

- [C3 C interoperability](https://c3-lang.org/language-common/cinterop/)
- [C3 bitstructs](https://c3-lang.org/language-common/bitstructs/)
- [C3 contracts](https://c3-lang.org/language-common/contracts/)

Accepted on 2026-10-14.
