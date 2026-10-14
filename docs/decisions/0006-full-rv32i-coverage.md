# ADR 0006: Complete base RV32I support for the first full MVP

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decision

Require complete RV32I base instruction support across the CPU, assembler, disassembler, and reference model by the first full MVP. Develop through small working subsets rather than implementing the entire set at once.

## Consequences

- The initial arithmetic/store program is an intermediate milestone, not the release's complete instruction scope.
- Encoding, operand handling, state updates, and instruction-specific edge cases require validation.
- ECALL, EBREAK, FENCE, and memory-access behavior must match a documented execution environment. Those platform details still need decisions.
- Additional ISA extensions, privileged execution, a guest OS, a compiler/linker, and cycle-accurate timing are not implied by this instruction-coverage target.
- Formal proof scope remains separate: try one proof experiment before deciding release proof requirements.
- Required host targets remain Linux and macOS, each on x86_64 and ARM64.

## Reference and acceptance

[RV32I specification, version 2.1](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html).

Accepted on 2026-10-14 during project planning, after reviewing [the coverage comparison](../research/rv32i-coverage-options.md).
