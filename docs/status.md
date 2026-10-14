# Current status

![Stage: Planning](assets/badges/planning.svg)

[Documentation](README.md) · [Progress](progress.md) · [Decisions](decisions/README.md)

Stage: planning. This template records no completed implementation or proof.

## Established requirements

- Personal experimentation with computer architecture and low-level systems.
- Assembly programs first; three languages in the first full MVP.
- Formal verification is an explicit interest, alongside ordinary tests.
- Existing ISA first; RISC-V RV32I selected.
- First MVP supports Linux and macOS, each on x86_64 and ARM64.
- Run one proof experiment before deciding the MVP's formal verification scope.
- Terminal UI with register and memory panels.
- Complete RV32I base instruction support for the first full MVP, developed through small working subsets.
- Raw binaries with separate symbols, direct host loading, and misaligned-data traps.
- Configurable RAM with a default capacity of 64 KiB.
- All-zero initial integer registers; entry PC set separately; guest establishes its own stack pointer.
- Little-endian data and instruction bytes for the first RV32I machine.

## Accepted language decision

Rust, OCaml, and C3 will participate in the first full MVP. C3 is selected for the native memory/device subsystem. See [ADR 0001](decisions/0001-native-subsystem-c3.md).

The initial ISA direction, host systems, and proof-first scope decision are recorded in [ADR 0002](decisions/0002-initial-foundations.md).

RISC-V RV32I is selected in [ADR 0004](decisions/0004-first-isa-rv32i.md).

## Decision state

| Decision | Current state |
| --- | --- |
| First ISA | RISC-V RV32I selected; ADR 0004 accepted |
| ISA selection criterion | Manageable implementation complexity |
| Full-MVP instruction coverage | Complete RV32I base instruction support; ADR 0006 accepted |
| User interface | Terminal UI with register and memory panels; accepted |
| Terminal UI library | Not selected |
| ISA direction | Existing ISA first; accepted |
| Host operating systems | Linux and macOS; accepted |
| Host CPU architectures | x86_64 and ARM64 on both Linux and macOS; accepted |
| CPU/application language | Rust |
| Toolchain/model language | OCaml |
| Native memory/device language | C3 selected; ADR 0001 accepted |
| Program image | Raw binary with separate symbol information; accepted |
| Startup method | Host loads image and establishes initial state/PC; accepted |
| Misaligned data access | Trap for misaligned halfword/word access; accepted |
| RAM capacity | Configurable; default 64 KiB; ADR 0008 accepted |
| Guest byte order | Little-endian for the first RV32I machine; ADR 0010 accepted |
| Future architecture byte-order configuration | Machine profiles proposed; only supported choices per architecture; outside MVP |
| Initial register values | All zero; no preset stack pointer or return address; ADR 0009 accepted |
| Memory map and load/entry addresses | Proposal in `first-machine.md`; not accepted |
| Verification workflow | Cameleer/Why3 for OCaml and Kani for Rust proposed; feasibility not established |
| Verification scope | One proof experiment first; decide release requirements afterward |
| Build tooling and versions | Not selected |

## Current objective

Use [the progress tracker](progress.md) for the current milestone, task, blockers, evidence, and next action. Use [task guides](tasks/README.md) for the procedure and acceptance checks. This file owns requirements and decision state, rather than a second task backlog.

## Updating this file

Update this file when requirements or accepted decisions change. Update `progress.md` and the relevant task record for execution progress. Do not infer a completed implementation, setup, or proof from the existence of a guide.
