# ADR 0004: Select RISC-V RV32I as the first ISA

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Context

The first machine should implement an existing ISA with manageable instruction-level emulator and assembler complexity. [The candidate comparison](../research/first-isa-comparison.md) considered RV32I, MOS 6502, Intel 8080, and CHIP-8.

## Decision

Select RISC-V RV32I as the first CPU target. Begin implementation incrementally, using a small reference program. Full-MVP coverage was left open at acceptance; [ADR 0006](0006-full-rv32i-coverage.md) subsequently selects complete base instruction support.

## Rationale

Fixed-length base instructions, consistent register-field positions, and direct compare-and-branch operations provide regular architectural behavior. Split immediate encodings, signedness, and alignment still require careful specification and checking.

## Consequences

- ISA-specific research, examples, assembler design, CPU state, and proof experiments now target RV32I.
- This choice does not select optional extensions, privileged execution, a guest OS, or full base-instruction coverage for the MVP.
- The initial reference program remains a proposal; its RAM layout, startup rules, and execution environment still need decisions.
- Linux/macOS support and the terminal UI remain accepted requirements.

## References and acceptance

[RISC-V RV32I specification, version 2.1](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html).

Accepted on 2026-10-14 during project planning.
