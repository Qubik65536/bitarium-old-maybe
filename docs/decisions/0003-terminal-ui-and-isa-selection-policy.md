# ADR 0003: Terminal UI and simplicity-first ISA selection

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decisions

- Use a terminal UI with register and memory panels for the first MVP.
- Compare existing ISAs and prioritize manageable implementation complexity.
- Decide full-MVP instruction coverage after choosing the ISA.

## Consequences

The first ISA was open at this record's acceptance. The subsequent selection of RV32I is recorded in [ADR 0004](0004-first-isa-rv32i.md). Register width alone does not establish implementation difficulty; instruction formats, addressing modes, state updates, and required platform behavior informed the comparison.

The terminal UI library, panel layout, controls, and exact compatibility scope remain open. [ADR 0005](0005-host-architectures.md) subsequently selects the host architectures. A desktop GUI remains a possible later extension.

## Acceptance

Accepted on 2026-10-14 from the project planning answers. See [the ISA comparison](../research/first-isa-comparison.md).
