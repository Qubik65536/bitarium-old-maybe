# T0001: Prepare the local development environment

![Guide: Ready](../assets/badges/ready.svg)

[Task guides](README.md) · [Progress](../progress.md) · [Setup commands](../development-environment.md)

Status: Done

Milestone: [M00](../progress.md)

Dependencies: choose the current host's setup path; no implementation prerequisites

Guide readiness: ready; acceptance checks verified on macOS ARM64

## Objective and completion criterion

Prepare one local development environment in which the generated Rust and OCaml executables run and the C3 static library builds. Record the actual host and tool versions.

## Why this task matters

Independent builds separate tool installation problems from later memory, CPU, and interoperability problems. This task prepares all three chosen languages; it does not implement or integrate the emulator.

## Starting state and files

Extracted project documentation, access to a development shell, and a selected macOS or Linux setup path. Read [context](../context.md), [status](../status.md), and [the setup guide](../development-environment.md).

The setup creates proposed component folders `runner/`, `memory/`, and `toolchain/`, plus `programs/`. If any already exist, inspect them and use the existing project rather than overwriting it with a scaffold.

## Concepts

Rustup manages Rust toolchains; Cargo builds the Rust program. C3's compiler builds its static library. Opam's local switch isolates the OCaml compiler/packages, while Dune builds OCaml source. A successful library build does not establish Rust linkage or the future native interface.

## Procedure

1. Identify the current OS/architecture and choose its installation section in [development-environment.md](../development-environment.md).
2. Install missing host prerequisites and language tools. Use existing installations when suitable; record their versions.
3. Activate the local opam switch and create only the missing component scaffolds.
4. Follow the guide's independent Rust, C3, and OCaml build/run checks. Stop at a failure and record the command and actual diagnostic before investigating it.
5. Record the host, compiler/build-tool versions, commands, and observed outcomes in a new experiment note. Link it below and in the progress tracker.

The installation and command details have one canonical home in the setup guide. Its References section links the official documentation used to prepare the procedure.

## Acceptance checks

- [X] Current host OS/version and architecture recorded.
- [X] Rust scaffold executable builds and runs.
- [X] C3 static-library scaffold builds.
- [X] OCaml scaffold executable builds and runs.
- [X] Actual compiler/build-tool versions and results recorded in a linked experiment note.
- [X] Generated build/local-switch directories excluded from version control; intended source and application lockfile retained.

Editor integration can be configured during setup, but it does not replace compiler/build checks. Completion on one host does not mark all four full-MVP host targets complete.

## Troubleshooting

- Command not found: verify tool installation and PATH in the shell that runs builds.
- Linker/compiler missing: revisit the selected host-prerequisite section.
- OCaml tools missing or from another switch: activate the local switch, or use `opam exec --`.
- C3 standard library missing: retain the compiler's bundled library and follow the setup guide's `C3C_LIB` guidance.
- Editor cannot see tools: verify compilation in the development shell first; then address the editor's environment separately.

## Actual results and evidence

Owner reported completion and tool versions. Review observed successful Rust build/run, C3 static-library build, and OCaml build/run on macOS 26.6.2 ARM64. Ignore checks exclude generated directories and retain sources and `runner/Cargo.lock`; see the linked experiment for commands and results.

Experiment record: in [experiments/20261014.md](../experiments/20261014.md).

## Blockers and remaining work

No remaining T0001 acceptance check. Other host targets, native integration, substantive emulator implementation, and proofs remain unverified.

## Handoff

Next action: prepare a task guide for checked C3 byte reads/writes under M01, including ownership, bounds, invalid-access behavior, and experiment-specific initial byte contents.
