# Assistant workspace

This directory is for Codex and other planning/research assistants. It contains workflow context and copyable prompts, not a second technical specification.

## Read first

1. Follow [the root instructions](../AGENTS.md).
2. Read [the context bridge](context.md), then the shared project records it identifies.
3. Use the prompt that matches the session.

## Prompt templates

| Template | When to use it |
| --- | --- |
| [Start the next task](prompts/01-start-next-task.md) | Begin a planning/research session and prepare a guide |
| [Continue after progress or a milestone](prompts/02-continue-after-milestone.md) | Reconcile results, update tracking, and prepare the next exercise; reusable every session |

The first prompt needs no implementation results. For continuation, replace the final placeholder with what actually happened, including useful commands, outputs, errors, or source changes.

## Separation of concerns

Shared technical facts and decisions belong in the human-facing project records. Assistant behavior and prompts belong here or in AGENTS.md. Reference the shared facts through the context bridge; do not create a competing assistant version of the machine specification.

Human-facing documentation should stand on its own and must not depend on or link to this directory. This directory is not implementation or completion evidence.
