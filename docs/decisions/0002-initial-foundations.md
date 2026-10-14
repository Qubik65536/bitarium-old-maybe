# ADR 0002: Initial ISA direction, host systems, and verification scope policy

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Context

The first release needs an architecture direction, host support boundary, and a realistic way to determine formal verification requirements.

## Decisions

- Implement an existing ISA first. A custom ISA remains a possible later experiment.
- Support Linux and macOS in the first MVP.
- Perform one proof experiment before deciding the formal verification scope required for the first full MVP.

## Consequences

- Research informed the subsequent RV32I selection in [ADR 0004](0004-first-isa-rv32i.md); supported scope still needs a decision.
- Host CPU architectures were left unspecified at acceptance; [ADR 0005](0005-host-architectures.md) subsequently selects x86_64 and ARM64 on both systems. Operating-system versions remain open.
- Build and integration plans must account for both Linux and macOS.
- Initial proof targets and tools are candidates until tried. A broad proof commitment is not an acceptance requirement yet.
- This record left the exact ISA and interface style open at acceptance. Subsequent decisions select RV32I and a terminal UI; the initial memory map and build system remain open.

## Acceptance

Accepted on 2026-10-14 from the project planning answers. This record does not imply any completed implementation or proof.
