# Progress tracker

![Stage: Planning](assets/badges/planning.svg) ![Builds / proofs: No evidence](assets/badges/unverified.svg)

[Documentation](README.md) · [Task guides](tasks/README.md) · [Session handoff](#session-handoff)

This file is the source of truth for milestone and task progress. [Status](status.md) records requirements and decision state; [context](context.md) describes the project. Guide availability does not establish implementation completion.

## Current position

- Phase: environment preparation; no build, implementation, or proof completion has been reported or verified.
- Current milestone: M00, local development environment.
- Next ready task: [T0001, prepare the local development environment](tasks/T0001-development-environment.md).
- Latest completed implementation task: none recorded.
- Blockers for T0001: none recorded; choose the current host's setup path at the start of the task.
- Decisions needed before the first integrated guest program: RAM base, load address, entry PC, RAM initialization, fault state updates, and stopping conventions.

## State conventions

| State | Meaning |
| --- | --- |
| Not started | No work/evidence recorded |
| Ready | Prerequisites satisfied and an actionable task guide is available |
| In progress | Work has begun; record what actually happened |
| Blocked | A named dependency, decision, or failure prevents the next action |
| Review | Completion is reported, but an acceptance check or review remains |
| Done | Acceptance criteria met with linked evidence and its source identified |

Use these states for both tasks and milestones where applicable. A milestone with unfinished required tasks remains unfinished. Creating research notes or writing a tutorial can make a task ready; it does not complete the implementation task.

## Milestones

This is an initial working sequence. The product requirements are accepted; task decomposition and ordering may evolve as experiments provide evidence.

| ID | Milestone | State | Prerequisites |
| --- | --- | --- | --- |
| M00 | Local environment | Not started | Current host identified |
| M01 | Checked C3 memory | Not started | M00; explicit access contract |
| M02 | One Rust CPU step and native integration | Not started | M01; native interface specified |
| M03 | First small guest program | Not started | M02; concrete platform/fault/stopping rules accepted |
| M04 | OCaml assembly tools | Not started | M03; initial syntax and symbol format specified |
| M05 | Terminal inspection | Not started | M03; stepping/debugger behavior specified |
| M06 | Complete RV32I machine and tools | Not started | M04; independent reference-model design |
| M07 | First full MVP acceptance | Not started | M05, M06, P01; four-host evidence |
| P01 | First proof experiment and scope decision | Not started | One suitable small function and stated property; can overlap M01-M06 |

<details>
<summary>Milestone completion criteria and evidence</summary>

### M00 · Local environment

Rust and OCaml scaffold executables run; C3 static library builds; actual versions and host recorded.

**Evidence:** None reported.

### M01 · Checked C3 memory

Byte/halfword/word access, little-endian conversion, bounds, and alignment behavior checked, including rejected writes.

**Evidence:** None reported.

### M02 · One Rust CPU step and native integration

Zeroed initial integer registers, x0 invariant, one ADDI step and PC update checked; Rust accesses C3 memory through the agreed interface.

**Evidence:** None reported.

### M03 · First small guest program

Hand-encoded reference program executes with its expected register/memory trace and breakpoint behavior.

**Evidence:** None reported.

### M04 · OCaml assembly tools

Initial subset assembles with checked operands/labels and disassembles; emitted bytes run correctly.

**Evidence:** None reported.

### M05 · Terminal inspection

Register and memory panels, stepping, breakpoints, disassembly, and basic traces work on the initial program.

**Evidence:** None reported.

### M06 · Complete RV32I machine and tools

All RV32I base instructions covered by CPU/assembler/disassembler/model; minimal output works; documented cases and reference comparisons pass.

**Evidence:** None reported.

### M07 · First full MVP acceptance

All accepted requirements and the proof scope chosen after P01 are met with reproducible evidence.

**Evidence:** None reported.

### P01 · First proof experiment and scope decision

Proof attempt, assumptions, tool/version, outcome, and limits recorded; required MVP verification scope decided.

**Evidence:** None reported.

</details>

The five-instruction program is an early milestone, not the complete instruction requirement. Proof scope remains undecided until P01; the proof experiment must still take place before MVP acceptance.

## Tasks

Add small tasks as the next milestone is researched. Avoid creating a detailed backlog for every future instruction before the first increment works.

| Task | Milestone | State | Guide | Evidence |
| --- | --- | --- | --- | --- |
| **T0001 · Prepare local environment** | M00 | Ready | [Open guide](tasks/T0001-development-environment.md) | Builds not reported |

Each task's record and tutorial live together in `tasks/`. Create the next task with [the task template](tasks/TEMPLATE.md). Research notes belong in `research/`; experiment and proof results belong in `experiments/` or the relevant verification record.

## Host acceptance

These rows track full-MVP acceptance per host. A T0001 scaffold build is setup evidence and does not complete a host's MVP acceptance. One local success does not establish another host target. Record native build/run evidence separately, including actual OS/compiler versions, the source revision, and the acceptance cases tested.

| Host | State | Evidence |
| --- | --- | --- |
| Linux x86_64 | Not started | None reported |
| Linux ARM64 | Not started | None reported |
| macOS x86_64 | Not started | None reported |
| macOS ARM64 | Not started | None reported |

## Session handoff

- Last confirmed result: foundational machine choices accepted in ADR 0001-0010; no implementation/build result recorded.
- Prepared material: [environment setup](development-environment.md) and T0001.
- Work remaining in the current task: follow the selected host setup, create scaffolds locally, run independent builds, and record observed results.
- Next action: begin T0001; if setup fails, record the failing command/output and the smallest recovery step.
- Pending verification: all task acceptance checks, P01, and all four host targets.

## Updating progress

When a task changes state, update its task record, task-table row, and this handoff together. Update the milestone state if justified. Use real dates only when an event actually occurred.

For evidence, distinguish an observed command result, a source inspection, and an owner-reported result. Record commands, actual outcome, relevant paths/revision, and what was not checked. If completion is reported without sufficient detail for its acceptance criteria, use Review rather than inventing results.

For a blockage, name the cause, its impact, and the next unblocking action. When work resumes, replace stale blockers and next actions. Keep one immediate next action in the handoff.
