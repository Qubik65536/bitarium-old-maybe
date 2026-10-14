# ADR 0009: All-zero initial integer registers

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decision

Initialize all 32 RV32I integer registers to zero before guest execution. The architectural x0 register remains zero throughout execution.

Set the PC separately to the selected entry address. This decision does not select that address.

## Consequences

- Startup register values are deterministic and straightforward to inspect and compare against the reference model.
- The host does not preset a stack pointer or return address. Guest programs establish their own stack and calling conventions when needed.
- Stack setup must account for the configured memory map and capacity.
- Initial RAM contents, guest byte order, and concrete memory-map/load/entry addresses were separate open decisions at acceptance. [ADR 0010](0010-first-machine-little-endian.md) subsequently selected little-endian byte order; the other items remain open.
- This is a Bitarium platform convention, not a claim that physical RISC-V hardware resets every register to zero.

## Acceptance

Accepted on 2026-10-14 during project planning.
