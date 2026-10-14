# First-machine proposal

![Specification: Accepted + proposed](assets/badges/mixed.svg) ![ISA: RV32I](assets/badges/rv32i.svg)

[Documentation](README.md) · [Context](context.md) · [Decisions](decisions/README.md)

Status: **ISA and some platform rules accepted; concrete memory/load addresses and initial implementation slice proposed**. This document is not a claim of existing implementation support.

Complete RISC-V RV32I base instruction coverage, Linux/macOS support on x86_64 and ARM64, the terminal UI, raw binaries with separate symbols, direct host loading, misaligned-data traps, configurable RAM with a 64 KiB default, all-zero initial integer registers, and little-endian data/instruction bytes are accepted. Memory-map/load/entry addresses remain proposals.

## Initial target

| Item | Behavior | State |
| --- | --- | --- |
| ISA | RV32I, instruction-level execution | **Accepted** |
| Full-MVP coverage | Complete RV32I base support | **Accepted** |
| First instructions | ADDI, ADD, LW, SW, EBREAK | Proposed increment |
| Program image | Raw binary and separate symbols | **Accepted** |
| Startup | Host loads image and establishes initial state/PC | **Accepted** |
| RAM capacity | Configurable; default 64 KiB | **Accepted** |
| RAM map | Base 0x0000; default-size range 0x0000-0xFFFF | Proposed |
| Data and instruction bytes | Little-endian | **Accepted** |
| Load address and entry PC | 0x0000 | Proposed |
| Initial integer registers | All zero; x0 remains zero | **Accepted** |
| Misaligned halfword/word data access | Trap to host/debugger | **Accepted** |
| Instruction alignment | Four-byte alignment | RV32I base rule |
| Invalid accesses/instructions | Structured error/trap; partial-update behavior unresolved | Proposed / open |
| Stopping | Pause on EBREAK; enforce an instruction limit | Proposed |

RV32I supplies instruction semantics. RAM layout, startup policy, mapped devices, and runner conventions belong to Bitarium's platform. EBREAK is a breakpoint instruction, not an invented HALT opcode. Full privileged trap handling is outside this initial slice.

The PC is initialized separately to the selected entry address. The host does not preset a stack pointer or return address. Guest programs that need a stack must initialize x2 before using it. The all-zero register decision does not specify the initial contents of RAM.

Primary reference: [RISC-V RV32I specification](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html). Read the relevant sections before hand-encoding instructions.

## Reference program

This assembly example is a specification target, not generated implementation code:

```asm
addi x1, x0, 5
addi x2, x0, 3
add  x3, x1, x2
sw   x3, 256(x0)
ebreak
```

Assuming the proposed load address, the expected result is x0 = 0, x1 = 5, x2 = 3, x3 = 8; memory bytes at 0x100 are `08 00 00 00` in the accepted little-endian order. After four successful instructions, PC is 0x10; the runner reports a breakpoint at that instruction address.

Record PC, changed registers, and changed memory after each successful step. Derive instruction bytes from the manual and check them independently.

## Before accepting

C3, complete RV32I base coverage, raw images, host loading, misaligned-data traps, RAM capacity policy, initial register values, and little-endian byte order are selected. Decide concrete load/entry and memory-map addresses, other memory/error rules, and stopping conventions. This five-instruction slice is an early milestone. The full MVP still needs the remaining base instructions, labels, terminal inspection panels, minimal output, and the verification scope agreed after one proof experiment.
