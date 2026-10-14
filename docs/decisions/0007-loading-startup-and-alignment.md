# ADR 0007: Raw images, direct host loading, and alignment traps

![Decision: Accepted](../assets/badges/accepted.svg)

[Decision records](README.md) · [Decision state](../status.md)

Status: accepted

## Decisions

- The MVP assembler emits a raw binary with separate symbol information.
- The host loader places program bytes in memory and establishes the initial machine state and PC. No guest boot ROM is required for the MVP.
- Misaligned data accesses report alignment traps: halfword accesses require addresses divisible by two; word accesses require addresses divisible by four. The host/debugger reports the offending instruction and address.

## Rationale

Raw images keep the initial toolchain focused on instruction encoding and memory placement. Direct host loading lets experiments begin with application instructions. Alignment traps make invalid data placement observable and keep the initial access contract small.

## Consequences

- Load address and entry point must be supplied through a documented convention/configuration; their exact values are not chosen yet.
- Symbol-map schema, memory size, byte order, initial register values, and loading/error rules were open at acceptance. [ADR 0008](0008-configurable-ram.md) subsequently selected configurable RAM with a 64 KiB default, [ADR 0009](0009-initial-register-state.md) selected all-zero integer registers, and [ADR 0010](0010-first-machine-little-endian.md) selected little-endian byte order. The other items remain open.
- ELF and guest boot ROM are later possibilities; adding them does not require changing CPU instruction semantics.
- Reporting misaligned data traps is an execution-environment choice compatible with complete RV32I instruction coverage. It is distinct from instruction-address alignment.
- Guest exception-handler machinery is not introduced by this decision. Exact pause/resume and termination conventions remain to be documented.

## Reference and acceptance

[RV32I load/store specification](https://docs.riscv.org/reference/isa/v20260120/unpriv/rv32.html), especially execution-environment-dependent misaligned access behavior.

Accepted on 2026-10-14 after comparing program formats, startup paths, and alignment policies.
