# ADR 0005: Support x86_64 and ARM64 on Linux and macOS

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decision

The first MVP supports these four native host targets:

| Operating system | CPU architecture |
| --- | --- |
| Linux | x86_64 |
| Linux | ARM64 |
| macOS | x86_64 |
| macOS | ARM64 |

## Consequences

Build instructions and validation must account for all four targets. The Rust application, C3 library, and OCaml tools must be built appropriately for the selected host. This decision does not require cross-compilation; the build approach remains open.

The emulated machine is RV32I regardless of host CPU architecture. Exact OS/compiler versions and available validation machines remain open. No target has been validated yet.

## Acceptance

Accepted on 2026-10-14 during project planning.
