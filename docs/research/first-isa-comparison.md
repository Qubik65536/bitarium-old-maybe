# Research: A manageable first ISA

![Research: Notes](../assets/badges/research.svg)

[Research notes](README.md) · [RV32I decision](../decisions/0004-first-isa-rv32i.md)

Status: research complete for the first-ISA decision. RV32I was selected in [ADR 0004](../decisions/0004-first-isa-rv32i.md).

## Question

Which existing instruction set is a manageable first target for an instruction-level emulator, an assembler, a terminal inspector, and one proof experiment?

## Primary sources

- [RISC-V RV32I specification](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html): programmers' model, instruction formats, immediate encodings, computational instructions, and branches.
- [MOS MCS6500 programming manual, January 1976, original-manual transcription](https://archive.org/stream/6500-50a_mcs6500pgmmanjan76/6500-50A_MCS6500pgmManJan76_djvu.txt): arithmetic, status flags, addressing, and stack operations. OCR must be checked against the scan when exact encodings matter.
- [Intel 8080 assembly manual, Rev. C, 1976, scanned original](https://vtda.org/docs/computing/Intel/98-004C_8080AssemblyProgManualRevC1976.pdf): registers, condition bits, instruction descriptions, and encoding tables.
- [RCA COSMAC VIP manual, 1978, reproduced original text](https://manualzz.com/doc/2092144/rca-cosmac-vip-cdp18s711-computer-instruction-manual): CHIP-8 programming and interpreter sections. Use the original-manual portion, not the hosting site's summaries.

## Facts relevant to implementation

RV32I has 32-bit base instructions, 32 integer registers including fixed-zero x0, consistent register-field positions, and comparisons in branch instructions rather than a condition-code register. Its branch/jump immediates are split across instruction bits, and memory alignment/environment behavior must be specified.

The 6502 has a small accumulator/index-register model, multiple addressing modes, arithmetic/status flags, and decimal arithmetic. Its small data width does not remove those semantic responsibilities. A particular 6502 variant must be identified before claiming compatibility.

The 8080 has byte-oriented registers, register pairs, an accumulator, and condition bits including parity and auxiliary carry. Instruction descriptions determine which bits change. A complete implementation therefore needs more than byte arithmetic and opcode dispatch.

CHIP-8 is an interpreted virtual instruction set, distinct from the COSMAC hardware CPU. Its specification includes display/input/timing-related operations. It is a useful interpreter experiment but a different starting emphasis from emulating a general hardware CPU.

## Assessment

These are engineering judgments, not measured development times. A small CHIP-8 interpreter is a plausible shortest path to executing existing programs. For Bitarium's CPU, memory, assembly, and verification interests, RV32I offers a strong balance of regular behavior and manageable scope. The 6502 and 8080 are viable when historical byte-oriented machines and flags are an explicit interest.

Recommendation: RV32I, beginning with the existing small arithmetic/store proposal and expanding incrementally. Choosing RV32I does not automatically require extensions, privileged execution, an OS, or full instruction coverage in the MVP.

## Open decisions

RV32I is selected, with complete base support required for the first full MVP in [ADR 0006](../decisions/0006-full-rv32i-coverage.md). Do not label intermediate subsets fully compatible. Cycle timing and undocumented hardware behavior are separate scope questions.
