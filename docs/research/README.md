# Research notes

![Research: Notes](../assets/badges/research.svg)

[Documentation](../README.md) · [Research template](TEMPLATE.md) · [Decisions](../decisions/README.md)

## Start a focused investigation

Copy `TEMPLATE.md` to a descriptive filename when investigating a concrete question. Keep the question narrow enough to answer before the next implementation step.

Good first topics: ISA selection, load/store semantics, memory errors, Rust-native interoperability, and proof-tool feasibility.

## Record the evidence

Prefer official manuals and documentation. Record exact versions/sections when they affect behavior. Research conclusions are recommendations until recorded as accepted design decisions.

## Investigations

- [Checked C3 byte access over borrowed storage](c3-checked-byte-access.md): types, ownership, explicit bounds checks, and the isolated T0002 exercise.
