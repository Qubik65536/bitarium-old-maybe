# ADR 0010: Little-endian first RV32I machine

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decision

Use little-endian data memory for the first RV32I machine. Emit and fetch its base instructions using RISC-V's instruction byte layout, which also yields little-endian bytes for these 32-bit instructions.

For example, storing the 32-bit data value 0x12345678 places bytes `78 56 34 12` at increasing guest addresses.

## Consequences

- Guest behavior does not depend on host byte order. RAM is addressed as bytes; multi-byte values are assembled explicitly.
- The CPU, memory/device interface, assembler, disassembler, and reference model must agree on the selected machine's byte conventions.
- Raw images are copied as bytes without automatic byte swapping; the tools must emit the selected target's encoding.
- This choice applies to the first machine, not to every future architecture.
- The MVP does not require a big-endian variant or runtime byte-order switching.

## Future design proposal

A machine profile could select ISA, data byte order, instruction-encoding rules, and platform map. Each architecture must declare which choices it supports. Unsupported configurations must be rejected, rather than silently producing a different architecture.

Keep instruction encoding separate from data byte order: RISC-V instruction parcels have a fixed little-endian layout even when data byte order differs. Other architectures must follow their own encoding rules. RAM capacity remains a separate configurable property.

This profile design is a proposal for later extensibility, not an accepted implementation interface or added MVP requirement. Device register protocols may need their own byte-order rules.

## References

- [RISC-V RV32I specification](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html): load/store data byte order.
- [RISC-V unprivileged ISA introduction](https://docs.riscv.org/reference/isa/v20240411/unpriv/intro.html): instruction parcel storage.

## Acceptance

Accepted on 2026-10-14 during project planning.
