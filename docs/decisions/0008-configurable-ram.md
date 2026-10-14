# ADR 0008: Configurable RAM with a 64 KiB default

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decision

Provide configurable RAM capacity, defaulting to 64 KiB (65,536 bytes).

## Consequences

- Experiments can request a different capacity without changing CPU instruction semantics.
- The RAM base address and device address map remain open. A base of zero is still a proposal.
- Image loading and memory access must validate ranges against the configured capacity.
- Guest stack setup must use the configured memory map rather than a hard-coded default-size address.
- Guest byte order and initial register values were open at acceptance. [ADR 0009](0009-initial-register-state.md) subsequently selected all-zero integer registers, without a preset stack pointer; [ADR 0010](0010-first-machine-little-endian.md) selected little-endian byte order.
- Configuration syntax, capacity limits, and validation behavior remain to be specified.

## Acceptance

Accepted on 2026-10-14 during project planning.
