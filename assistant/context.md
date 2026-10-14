# Assistant context bridge

Read the root AGENTS.md before using this context. This file routes assistants to authoritative project records; it deliberately does not duplicate their technical decisions.

## Shared context and state

| Read | Use it for |
| --- | --- |
| [Project home](../README.md) | Overview and human documentation entry points |
| [Project context](../docs/context.md) | Purpose, requirements, scope, and component boundaries |
| [Requirements and decision state](../docs/status.md) | Accepted choices, proposals, and outstanding decisions |
| [Progress tracker](../docs/progress.md) | Current task, milestones, evidence, blockers, and handoff |
| Current task guide linked by the tracker | Procedure, acceptance checks, and task-specific results |

Then read only the relevant decision records, machine specification, source files, research, and experiments. Follow the tracker rather than assuming T0001 remains current forever.

## Preparing and recording work

Use the human [task template](../docs/tasks/TEMPLATE.md), [research template](../docs/research/TEMPLATE.md), and [experiment template](../docs/experiments/TEMPLATE.md). These describe the work and evidence without embedding assistant instructions.

The [verification plan](../docs/verification.md) defines how to state proof scope and limits. The [documentation index](../docs/README.md) explains where each kind of shared record belongs.

## Handling shared decisions

- Put technical rationale and its accepted/proposed state in the shared decision record.
- Put assistant-specific handling in AGENTS.md or this context only when needed, linking to the shared record instead of restating machine behavior.
- If shared records disagree, identify the discrepancy and follow the latest explicit accepted decision; reconcile related documentation within authorized scope.
- Never treat a copied prompt, assistant summary, or old conversation as stronger evidence than current accepted records and observed results.

No human-facing page needs a backlink here. Assistant instructions depend on shared project context; shared project context does not depend on assistant instructions.
