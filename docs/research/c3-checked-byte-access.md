# Research: Checked C3 byte access over borrowed storage

![Research: Notes](../assets/badges/research.svg)

[Research notes](README.md) · [Decisions](../decisions/README.md)

Status: researched; exercise design recommended, machine interface remains open

## Question and relevance

What is the smallest useful M01 implementation after T0001 that does not depend on an accepted guest memory map or allocation/FFI design?

## Primary sources

Official rolling documentation inspected on 2026-10-14; it is not pinned to the installed compiler release. T0001 records C3 0.8.5. Read-only inspection of its `--help` and `--list-project-properties` confirmed the test command, safety option, and `test-sources` property; no memory implementation or tests were run for this research.

| Source | Section | Relevant fact |
| --- | --- | --- |
| [C3 basic types](https://c3-lang.org/language-fundamentals/basic-types-and-values/) | Integers; Slices | `char` is unsigned 8-bit, `uint` unsigned 32-bit, and `usz` host-dependent; slices expose length and underlying storage |
| [C3 arrays](https://c3-lang.org/language-common/arrays/) | Fixed Size 1D Arrays; Slice | Fixed arrays passed by value are copied; a slice is a view, not an owning allocation |
| [C3 build commands](https://c3-lang.org/build-your-project/build-commands/) | Test; Build | `c3c test` runs functions marked `@test`; `c3c build` builds a project |
| [C3 specification](https://c3-lang.org/implementation-details/specification/) | Testing and benchmarking | Test functions take no arguments and return void; assertion failure is test failure |

## Verified facts

C3's unsigned `char` can represent all byte values, including 0x80 and 0xFF, without sign extension. Its host size type is distinct from an RV32I machine word. See [basic types](https://c3-lang.org/language-fundamentals/basic-types-and-values/).

A slice borrows backing storage; passing one does not extend that storage's lifetime. Passing a fixed array by value instead makes a copy. See [arrays](https://c3-lang.org/language-common/arrays/).

C3 safe-mode slice checks are a diagnostic backstop. They do not supply a recoverable application access result. Installed compiler help exposes `--safe=yes` and `--safe=no`, so explicit validation should define the operation's behavior in either mode.

Repository observation: `memory/src/main.c3` still contains generated add/sub functions, not byte access. `memory/project.json` selects a static library and includes `src/**` and `test/**`; its generated artifacts go under `build/`. T0001 supplies local build evidence only.

## Interpretation and alternatives

Recommended exercise: two C3 functions operating on a caller-owned mutable byte slice, using a host-sized unsigned offset. They borrow storage only for the call, allocate nothing, and report out-of-range access explicitly. A boolean status plus a non-null read-output pointer is enough to learn the contract. A C3 optional/fault result is a reasonable alternative but adds error-handling syntax; neither is an accepted Rust ABI.

For a byte access, validity is simply offset less than length. This avoids addition overflow and subtraction underflow. Rejected writes should preserve the whole buffer; rejected reads should preserve the output sentinel. These are exercise expectations, not an accepted guest trap/state-update policy.

Use an explicitly initialized small array owned by each test. This teaches ownership without selecting the eventual RAM allocator or startup contents. A single byte has no endianness conversion and no additional alignment requirement.

## Proposed exercise contract

Status: proposed local exercise contract for T0002; not an accepted machine interface or C ABI. This collects the exercise assumptions in one place.

| Aspect | Exercise expectation |
| --- | --- |
| Storage | Mutable byte slice owned by the caller; no allocation, freeing, or retained pointers |
| Offset | Unsigned host-sized offset within the slice, not a guest address |
| Read | Slice, offset, and non-null writable byte-output pointer; boolean status |
| Write | Slice, offset, and unsigned byte value; boolean status |
| Validity | Offset less than slice length; validate before indexing or changing output |
| Successful read | Copy addressed byte to output; storage unchanged |
| Successful write | Change only the addressed byte |
| Failure | Return false; preserve all storage and failed-read output |
| Caller preconditions | Backing storage lives through the call; read output points outside the backing slice |
| Empty storage | Reject every offset |
| Excluded behavior | Invalid pointers, dangling slices, output/storage aliasing, concurrency, signed offsets, allocation, guest traps |

Fixture contents are explicit test inputs, not startup RAM policy. Multibyte byte order and alignment are outside this exercise. The accepted language choice is recorded in [ADR 0001](../decisions/0001-native-subsystem-c3.md); wider machine decisions remain in [decision state](../status.md).

## Open questions

Guest address translation, RAM ownership/allocation, startup RAM contents, Rust/C3 signatures, permissions/devices, multibyte access and guest fault state updates remain unresolved or deferred. This exercise does not need them. Its temporary API can change before integration.

## Proposed next action

Follow [T0002](../tasks/T0002-checked-byte-access.md). Completion requires recorded passing boundary and non-mutation checks plus a static-library build; guide availability alone supplies no execution evidence.
