# Bitarium

**Build a small computer. Understand every layer.**

![Project: planning](docs/assets/badges/planning.svg) ![ISA: RV32I](docs/assets/badges/rv32i.svg) ![Languages: Rust, C3, OCaml](docs/assets/badges/languages.svg)

A personal, hand-coded playground for CPU emulation, virtual hardware, assembly tools, and formal verification. Begin with one inspectable RISC-V machine, then expand through small working experiments.

> **Current state:** documentation and foundational decisions are prepared. Local builds, implementation, and proofs have no recorded completion evidence. See the [live project tracker](docs/progress.md) for the latest position.

## Start building

| Start here | What you get |
| --- | --- |
| **1 · [Project overview](docs/context.md)** | Purpose, requirements, component boundaries, and scope |
| **2 · [Progress dashboard](docs/progress.md)** | Current task, milestones, evidence, and blockers |
| **3 · [Development setup](docs/development-environment.md)** | macOS/Linux tools and independent component builds |
| **4 · [Task guides](docs/tasks/README.md)** | Focused exercises with observable acceptance checks |

Extract this documentation template into the project root. Add source files as the implementation develops. The [getting-started guide](docs/guide.md) explains the working sequence.

## The first machine

| Area | Accepted direction |
| --- | --- |
| Instruction set | **Complete RV32I base support** by the first full MVP; small subsets along the way |
| Memory | Configurable RAM, **64 KiB default**, little-endian data |
| Startup | Raw image with separate symbols; host loads it and sets the entry PC; all integer registers initially zero |
| Alignment | Misaligned halfword/word data accesses trap to the host/debugger |
| Inspection | Terminal UI with register and memory panels |
| Hosts | Linux and macOS, each on x86_64 and ARM64 |
| Verification | Try one proof experiment, then decide the required MVP proof scope |

Concrete RAM/load/entry addresses and several execution rules remain open. The [machine specification](docs/first-machine.md) labels accepted rules and proposals separately; [decision state](docs/status.md) records the outstanding choices.

## Three languages, distinct responsibilities

| Component | Language | Responsibility |
| --- | --- | --- |
| CPU and runner | **Rust** | Registers, decode, instruction semantics, execution controls, debugger |
| Memory and devices | **C3** | Byte storage, checked access, address mapping, image placement, virtual devices |
| Assembly tools and model | **OCaml** | Assembler, disassembler, symbols, independent reference execution |

All three are host-side components. Guest programs begin as assembly. The exact interfaces and build integration remain proposed; see [component boundaries](docs/context.md#proposed-component-boundaries).

## Work in small increments

**Prepare → Build → Inspect → Record → Continue**

Start with checked memory access, execute one instruction, then run a small program. Add assembly tools and terminal inspection before completing the remaining base instructions. Verification work overlaps these increments.

The [tracker](docs/progress.md) owns milestone and task state. Each [task guide](docs/tasks/README.md) explains the exercise and its checks. A written guide means the work is ready to attempt; completion requires recorded results.

## Documentation library

| Read | Purpose |
| --- | --- |
| [Documentation index](docs/README.md) | Browse design, learning guides, progress, and records |
| [Design decisions](docs/decisions/README.md) | Accepted choices, rationale, and consequences |
| [Verification plan](docs/verification.md) | Proof candidates, assumptions, and limits |
| [Research notes](docs/research/README.md) | Primary sources, comparisons, and open questions |
| [Experiment records](docs/experiments/README.md) | Reproduction steps and actual observations |

---

*Instruction-level execution first. More architectures, guest firmware, ELF, and cycle-level experiments come later when a concrete experiment calls for them.*
