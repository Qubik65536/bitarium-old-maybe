# Design decisions

![Decision records: accepted](../assets/badges/accepted.svg)

**The choices that define the first machine, with their rationale and limits.**

[Documentation](../README.md) · [Decision state](../status.md) · [Machine specification](../first-machine.md)

## Accepted records

| ID | Decision |
| --- | --- |
| 0001 | [C3 memory and device subsystem](0001-native-subsystem-c3.md) |
| 0002 | [Existing ISA, hosts, and proof-first scope](0002-initial-foundations.md) |
| 0003 | [Terminal UI and ISA selection policy](0003-terminal-ui-and-isa-selection-policy.md) |
| 0004 | [RV32I first](0004-first-isa-rv32i.md) |
| 0005 | [x86_64 and ARM64 on Linux and macOS](0005-host-architectures.md) |
| 0006 | [Complete RV32I base coverage](0006-full-rv32i-coverage.md) |
| 0007 | [Loading, startup, and alignment](0007-loading-startup-and-alignment.md) |
| 0008 | [Configurable RAM, 64 KiB default](0008-configurable-ram.md) |
| 0009 | [All-zero initial integer registers](0009-initial-register-state.md) |
| 0010 | [Little-endian first machine](0010-first-machine-little-endian.md) |

## Add a decision

Start from [TEMPLATE.md](TEMPLATE.md). Explain the concrete problem, relevant alternatives, chosen behavior, and consequences. Record acceptance only after the choice is made. A research recommendation is a proposal until accepted.

Later decisions may resolve questions left open by earlier records. Preserve that history and link the later resolution; use [status](../status.md) to find the current decision state.
