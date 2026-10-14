# T0002: Checked C3 byte access

![Guide: Ready](../assets/badges/ready.svg)

[Task guides](README.md) · [Progress](../progress.md)

Status: Ready

Milestone: [M01](../progress.md#milestones)

Dependencies: T0001 Done on the current host; no guest memory-map or FFI decision required

Guide readiness: ready; implementation and acceptance checks not performed

## Objective and completion criterion

Hand-write byte read and write operations over caller-owned storage. Valid accesses return success; out-of-range accesses return failure without touching storage or a read-output value. Finish when the tests below pass and the static library still builds, with actual commands and outcomes recorded.

## Why this task matters

RAM begins as addressed bytes. This task makes the storage boundary explicit before adding little-endian words, guest address translation, CPU operations, or foreign calls. It is one increment of M01, not the whole milestone.

## Prerequisites and starting state

[T0001](T0001-development-environment.md) is Done with [local evidence](../experiments/20261014.md). C3 0.8.5 built the static-library scaffold on macOS ARM64. Inspect `memory/src/main.c3` and `memory/project.json`: the current source has scaffold add/sub operations, while the configuration already includes `src/**` and `test/**`. Check your installed compiler version before working; if it differs, record that difference rather than assuming identical behavior.

The RAM base, startup contents, allocation policy, and Rust ABI are still open in [decision state](../status.md). This exercise uses storage offsets and an explicitly initialized fixture; it does not settle those decisions.

## Research and references

Read [checked-byte research](../research/c3-checked-byte-access.md), especially the type and ownership distinctions. Keep [C3 arrays](https://c3-lang.org/language-common/arrays/) open for slice syntax and [basic types](https://c3-lang.org/language-fundamentals/basic-types-and-values/) for integer widths. [Build commands](https://c3-lang.org/build-your-project/build-commands/) explains test discovery. These rolling docs were inspected on 2026-10-14; compiler diagnostics take priority for version-specific syntax.

## Files involved

| File | Your work |
| --- | --- |
| `memory/src/main.c3` | Inspect scaffold; add the two small operations here, or put them in `memory/src/bytes.c3` using module `memory` |
| `memory/test/bytes.c3` | Create C3 unit tests with descriptive names and the `@test` attribute |
| `memory/project.json` | Inspect existing source/test-source paths; no new build target is expected |
| `docs/experiments/` | Create an actual-results record using its template after running checks |
| This task and `docs/progress.md` | Link evidence and reconcile task state after checks |

`memory/build/` contains generated output and is ignored. No Rust or OCaml changes are needed.

## Concepts to understand

**Bytes and offsets.** Use C3 `char` for an unsigned byte (0–255) and `usz` for an offset within host storage. `uint` is a 32-bit machine value; it is not interchangeable with a host-sized index. A future guest address must be translated and validated separately. No modular arithmetic is needed here.

**Borrowing.** A mutable `char[]` slice refers to someone else's storage. The caller keeps it alive during the call. Your functions neither allocate/free that storage nor keep a pointer to it afterward. Fixed arrays passed by value are copied, so write tests must inspect the original array.

**Validity.** For length N, valid byte offsets are 0 through N−1. Compare the offset directly against N before indexing. Avoid computing offset+1 or N−1 as your general validity check; they introduce avoidable overflow or empty-buffer underflow. Every byte offset is aligned for this operation. Endianness matters only when several bytes encode a wider value.

### Exercise contract

This is a local teaching contract, not an accepted machine or C ABI design. Implement a read operation taking a mutable byte slice, unsigned host offset, and non-null output pointer, returning a boolean status. Implement a write operation taking the same slice/offset plus an unsigned byte value, also returning status. Choose descriptive names yourself.

- Caller guarantees a valid slice whose backing storage lives through the call. For reads, the output pointer refers to a writable byte outside the backing slice; this excludes output/storage aliasing from the exercise.
- Successful read copies exactly the addressed byte to the output and leaves storage unchanged.
- Successful write changes exactly the addressed byte.
- An offset greater than or equal to length returns failure before indexing. Failure leaves the whole storage unchanged; a failed read also leaves its output unchanged.
- Empty storage rejects every offset. Null output pointers, dangling slices, concurrency, signed offsets, allocation, and guest traps are outside this contract. Do not attempt to test undefined caller behavior.

## Procedure

1. Write down the validity rule and ownership preconditions in source comments. Use the contract above without inventing a RAM base. Expected result: you can explain why offset 4 fails for a four-byte buffer and offset 3 succeeds.
2. Hand-write the two operations. Put validation before any subscript or output update. Expected result: each operation has one straightforward successful path and an explicit failure path. Do not add exports or Rust linkage yet.
3. Create independent C3 tests, each owning an explicitly initialized four-byte array such as `11 80 FF 44` in hexadecimal. Initialize the read output to a separate sentinel such as `A5`. Pass a slice that refers to that array. Compare against fixed expected values, not values generated by your new functions.
4. Cover the following cases. Use fresh fixtures or restore them explicitly so one case cannot conceal another's failure.

| Case | Expected observation |
| --- | --- |
| Read offsets 0 and 3 | Success; values 0x11 and 0x44; entire storage unchanged |
| Read offsets 1 and 2 | Success; 0x80 and 0xFF remain unsigned byte values |
| Write offsets 0 and 3 | Success; original array changes at that offset only |
| Write/read back 0x00, 0x80, 0xFF | Exact bytes preserved; check original array independently as well |
| Read/write offset 4 | Failure; full array equals its before-snapshot; failed-read output still 0xA5 |
| Read/write maximum `usz` value | Failure without indexing or arithmetic overflow; same preservation checks |
| Empty slice, offsets 0 and maximum `usz` | Failure; read sentinel unchanged |
| One-byte slice, offsets 0 and 1 | Offset 0 succeeds; offset 1 fails with preservation checks |

Derive the maximum host-size value from the documented type facilities your compiler supports; do not hard-code a 64-bit literal. Construct an empty slice with length zero rather than forming an inclusive range ending at −1.

5. Run the test driver from `memory/`. The first command records the compiler; the next two run your cases with safety checks enabled and disabled to establish that rejection comes from your explicit logic. The last builds the normal static library.

```sh
cd memory
c3c --version
c3c test --safe=yes
c3c test --safe=no
c3c build
```

Expected outcomes: both test invocations discover and pass your named tests, and the library build succeeds. A successful command with zero discovered tests is insufficient. These commands are instructions, not recorded runs. Inspect the configured paths or your test attributes if discovery fails; do not disable assertions to obtain a pass.

6. Record actual date, host and compiler version, source revision plus any uncommitted changes, exact commands, discovered test names/count, output and exit status. Link the experiment below. Include any failed attempt and its focused fix. This evidence supports tested cases on that host, not a formal proof or other-host support.

## Acceptance checks

- [ ] Ownership, valid-offset rule, output-pointer precondition, and failure behavior documented beside the operations.
- [ ] First/last, unsigned high-byte, and write-isolation cases pass against independently specified values.
- [ ] At-length and maximum-offset failures preserve the complete buffer and failed-read sentinel.
- [ ] Empty and one-byte buffers behave as specified.
- [ ] Named tests are discovered and pass with safety both enabled and disabled; normal static-library build succeeds.
- [ ] Actual commands, versions, source state, outcomes and evidence source recorded in a linked experiment.

## Troubleshooting

- Writes disappear: inspect whether a fixed array was copied instead of a slice borrowing the original array.
- A boundary test panics: check that validation occurs before indexing. A compiler bounds trap is not the required false result.
- Empty-buffer or maximum-offset arithmetic fails: remove unnecessary addition/subtraction from the validity check.
- Bytes above 0x7F change sign: inspect byte types and casts; use unsigned `char`, not signed `ichar`.
- No tests run: check `@test`, no-argument void test signatures, module syntax, and `test-sources` in the existing project. Record diagnostics before changing configuration.
- Only safe-mode tests pass: check for reliance on compiler runtime checks rather than explicit rejection.

## Actual results and evidence

Not performed. Research inspected sources and compiler help only. No byte-access build, test, or proof result exists yet.

Experiment record: none yet.

## Blockers and remaining work

No prerequisite blocker identified for this isolated exercise. All implementation acceptance checks remain pending. Broader machine/FFI decisions remain open and are deliberately not prerequisites here.

## Handoff

Last confirmed result: T0001 local environment checks passed. Immediate next action: document the exercise contract beside the C3 operations, then implement them by hand. After completion, prepare a separate small task for checked little-endian halfword access; M01 also still requires word access and alignment checks. Mark this task Done only after its recorded evidence satisfies the checks.
