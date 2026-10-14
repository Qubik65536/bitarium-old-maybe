# Getting started

![Type: Guide](assets/badges/guide.svg)

[Documentation](README.md) · [Progress](progress.md) · [Task guides](tasks/README.md)

## Working sequence

Start with a specification and a small observable program. Keep each implementation increment understandable and executable.

There is enough agreed to prepare the tools and begin memory operations. Follow the [development-environment guide](development-environment.md) for installation, component scaffolds, and separate local builds. Resolve concrete addresses and remaining execution rules before the first integrated program.

The [progress tracker](progress.md) owns milestone order, completion criteria, and current task state. Follow [task guides](tasks/README.md) for the actual exercises. Research, write the guide, perform the task, record the outcome, and then choose the next increment. A prepared guide is distinct from completed work.

Verification work overlaps these increments. Start one proof experiment when the first relevant small function exists, then decide the required release scope from its outcome.

## First work session

1. Read the current context, status, and progress tracker.
2. Verify the local toolchains using `development-environment.md`. Resolve the platform choices needed by the current increment; defer broader build automation until separate builds work.
3. Calculate the reference program's expected state after each instruction.
4. Write down the memory access contract: addresses, widths, byte order, permissions, alignment, and errors.
5. Implement one small memory operation yourself, then check its boundary behavior.

Do not begin with a plugin architecture or a generic CPU framework. Add abstractions when a concrete need appears.

