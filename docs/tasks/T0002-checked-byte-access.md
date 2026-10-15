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

## Before you start

[T0001](T0001-development-environment.md) is Done with [local setup evidence](../experiments/20261014.md), including C3 0.8.5. Open `memory/src/memory.c3`: it currently exports add/sub functions. Those give you a working starting module; this task adds byte operations in `memory/src/bytes.c3` and tests in `memory/test/bytes.c3`.

The C3 language choice belongs to [ADR 0001](../decisions/0001-native-subsystem-c3.md). Read the [proposed exercise contract](../research/c3-checked-byte-access.md#proposed-exercise-contract) for this task's temporary interface and preconditions. Wider choices stay in [decision state](../status.md#decision-state). No new architecture decision is needed to follow this exercise.

## Walkthrough

### 1. Draw the storage you will access

Think of memory here as four numbered boxes, each holding one byte. A byte holds a value from 0 to 255. The number identifying a box is its **offset** from the beginning of this buffer.

Draw this fixture in your notes; values are hexadecimal:

| Offset | 0 | 1 | 2 | 3 |
| --- | --- | --- | --- | --- |
| Byte | 11 | 80 | FF | 44 |

Length 4 means there are four boxes, not that box 4 exists. Offset 3 names the last box. Offset 4 names no box. This is why checking the boundary is part of the operation rather than relying on a crash afterward.

Open `memory/src/bytes.c3`, use the existing `memory` module, and write a short comment describing what the two byte operations will do. Link or refer to the proposed contract instead of inventing guest addresses.

**Checkpoint:** You can explain why offset 0 is valid, offset 4 is invalid, and an empty buffer has no valid offset.

### 2. Give the functions access to the original storage

An **array** owns its elements. A **slice** is a view of existing elements: it tells the function where those elements are and how many there are. Imagine handing someone the location of your four boxes rather than giving them four copied boxes. Changes through that view affect the original storage.

Read the Slice section of [C3 arrays](https://c3-lang.org/language-common/arrays/). In `memory/src/bytes.c3`, write the declarations and bodies for the two operations incrementally. Use a mutable byte slice for storage. The caller will keep the array alive during each call; your operations should not save it for later.

For types, consult [C3 integers](https://c3-lang.org/language-fundamentals/basic-types-and-values/#integers): `char` holds an unsigned byte and `usz` holds a host-sized unsigned offset. The high byte 0xFF must remain 255. A guest register value and a host storage index serve different purposes even when both happen to contain 3.

The read operation needs somewhere to return the byte as well as a success/failure result. Follow the proposed contract's separate output-pointer approach: the caller provides a writable byte outside the buffer. The write operation instead receives the new byte value. Let both return boolean status. Choose descriptive names yourself.

**Checkpoint:** You can identify who owns the array, which argument describes its length, and where a successful read puts the byte. Confirm your parameter types against the contract before filling in the behavior.

### 3. Make one read succeed and one read fail

Start with the read operation. Compare the offset directly with the slice length before indexing. For a four-byte slice, offset 4 must fail immediately. There is no need to compute offset+1; that addition introduces an overflow problem at the largest representable offset.

After validation succeeds, copy the addressed byte to the output and return success. On failure, return false without changing the output. This lets the caller distinguish failure from successfully reading zero.

Create `memory/test/bytes.c3`. Write a no-argument, void-returning test function marked `@test`; see [C3 testing](https://c3-lang.org/build-your-project/build-commands/). Give it its own explicitly initialized four-byte array from step 1 and a separate output byte initialized to 0xA5. That initial value is a **sentinel**: a recognizable value used to detect an unexpected write.

Write assertions for a read at offset 0 and, using a fresh sentinel, a failed read at offset 4. Check status, output, and the original array. Use fixed expected values, not values calculated through your new operations.

To compile and run this first checkpoint, run the test driver from the C3 project directory:

```sh
cd memory
c3c test --safe=yes
```

**Checkpoint:** Your named test is discovered. Offset 0 succeeds with 0x11; offset 4 fails with output still 0xA5. Storage remains `11 80 FF 44` after both reads. A bounds-check panic means rejection happened too late.

### 4. Change one byte and inspect its neighbors

Now write the store operation in `memory/src/bytes.c3`. Reuse the same validity rule. Only after validation should it change the addressed element and return success.

Add a test with a fresh fixture. Write 0x00 at offset 1. Inspect the original array directly: it should now contain `11 00 FF 44`. Checking neighbors matters: merely reading back offset 1 could miss an accidental write to another position.

Add a failed write at offset 4. Take a copy of the whole array before this call and compare every element afterward. This **snapshot** lets you check preservation even when earlier valid writes changed the fixture.

Run the same test command again from `memory/`.

**Checkpoint:** A valid write changes exactly one byte in the original array. An invalid write returns false and preserves the full before-snapshot. If the original array never changes, check whether you passed an array by value rather than a slice.

### 5. Extend the tests to the edges

You have checked the central behavior. Now target cases where assumptions often break. Add these cases in small groups, running tests after each group. Use fresh fixtures or explicitly restore them so earlier cases cannot hide a failure.

| Add this case | Why it matters | Expected observation |
| --- | --- | --- |
| Read offsets 1, 2, 3 | High bytes and the final valid offset | Success with 0x80, 0xFF, 0x44; storage unchanged |
| Write offsets 0 and 3 | Both ends of storage | Only the addressed byte changes |
| Write/read back 0x00, 0x80, 0xFF | Zero and high byte values | Exact byte preserved; inspect storage independently too |
| Maximum `usz` offset, read and write | Unnecessary arithmetic can overflow | False; entire buffer and failed-read sentinel unchanged |
| Empty slice, offsets 0 and maximum `usz` | Length zero has no final element | False; failed-read sentinel unchanged |
| One-byte slice, offsets 0 and 1 | Smallest nonempty buffer | Offset 0 succeeds; offset 1 fails with preservation checks |

Use your compiler's type facilities to obtain the maximum `usz` value rather than assuming the host is 64-bit. For an empty slice, use a zero-length view of valid backing storage; avoid constructing an inclusive range ending at −1. Consult the slicing forms in [C3 arrays](https://c3-lang.org/language-common/arrays/).

**Checkpoint:** Each rejection checks the boolean result, the complete storage snapshot, and the read sentinel where applicable. You can point to a named test covering each row.

### 6. Check the implementation without compiler bounds checks

Compiler safety checks help diagnose mistakes, but your functions promise to return failure themselves. Run the tests with safety enabled and disabled, then rebuild the library. From `memory/`:

```sh
c3c --version
c3c test --safe=yes
c3c test --safe=no
c3c build
```

The version command records the tool used. Both test runs should discover and pass your named cases. The final build checks that the normal library still compiles; it does not exercise the operations by itself. Generated C3 output stays under the configured build directory.

**Checkpoint:** Both modes pass the same cases, and the library builds. Zero discovered tests is insufficient. These are expected results, not evidence that the checks have already run.

### 7. Save what actually happened

Create an experiment note from [the experiment template](../experiments/TEMPLATE.md). Record actual date, host/compiler versions, source revision and uncommitted changes, exact commands, test names/count, output and exit status. Include any failing attempt and the fix you made.

Link that note in this task's evidence section. Compare the results with every acceptance check below before changing task state. Tests cover those cases on that host; they do not prove all inputs correct.

**Checkpoint:** Another session can reproduce your checks from the note, and unfinished checks are still visibly pending.

## References and scope

The [research note](../research/c3-checked-byte-access.md) holds source findings, alternatives, and the proposed exercise contract. This walkthrough implements only that byte-access exercise. Follow the linked decision records for platform requirements rather than extending this exercise into a guest memory map or Rust memory interface.

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

Last confirmed result: T0001 local environment checks passed. Immediate next action: follow step 1 and draw the byte fixture before creating the byte-access source file. After completion, prepare a separate small task for checked little-endian halfword access; M01 also still requires word access and alignment checks. Mark this task Done only after its recorded evidence satisfies the checks.
