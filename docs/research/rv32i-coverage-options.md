# Research: First-MVP RV32I instruction coverage

![Research: Notes](../assets/badges/research.svg)

[Research notes](README.md) · [Coverage decision](../decisions/0006-full-rv32i-coverage.md)

Status: comparison complete. Complete base RV32I support was selected in [ADR 0006](../decisions/0006-full-rv32i-coverage.md).

## Two possible completion criteria

| Choice | Required instruction support | Main trade-off |
| --- | --- | --- |
| Documented subset | An explicitly selected instruction list; unimplemented operations produce a diagnostic | Earlier release, with restricted guest programs |
| Complete base RV32I | All base instructions with behavior consistent with the documented execution environment | A clearer architectural endpoint, with more semantic and encoding cases to check |

The official RV32I specification lists 40 unique base instructions. ECALL and EBREAK request traps; FENCE provides ordering behavior. These need execution-environment definitions rather than merely being recognized by the decoder.

## Proposed development approach

In both cases, implement small vertical slices. The existing five-instruction reference program can run long before the final milestone. A complete-base release target does not mean implementing everything at once.

Accepted choice: complete base RV32I for the first full MVP, while keeping the virtual platform small. This gives the CPU, assembler, disassembler, and reference model a shared finite scope.

No compiler, guest OS, privileged architecture, extra ISA extensions, graphical desktop interface, or cycle-accurate timing is implied by either proposed release scope. Formal proof coverage is decided separately after the initial proof experiment.

## Reference

[RISC-V RV32I specification, version 2.1](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html), including environment-call/breakpoint and memory-ordering sections.
