# Documentation and research instructions

Applies to files in `docs/`, in addition to the root `AGENTS.md`.

- Use concise Markdown and relative links. Use Mermaid or tables when they explain a relationship more clearly than prose.
- Keep AI-specific policies, purposes, context routing, and prompts in AGENTS.md files or the root `assistant/` directory. Human-facing pages, navigation, and templates must not link to that area or to AGENTS.md. The assistant context bridge points to shared human docs; do not duplicate technical decisions in assistant-owned records.
- Keep `context.md` for stable project context, `status.md` for requirements/decision state, `progress.md` for milestone/task progress and handoff, and `tasks/` for task records/tutorials. Do not maintain competing backlogs or next actions in other files.
- Mark decisions as proposed, accepted, rejected, or superseded. Only record acceptance supported by the owner's explicit decision.
- For research, record the question, primary sources, relevant facts, interpretation, and unresolved issues. Include specification section/version when available.
- A research template is an outline, not completed research. Never fabricate a source, experiment, test result, proof, or implementation status.
- In verification notes, name the exact property, modeled behavior, assumptions, bounds, and implementation coverage. A proof of a model does not establish correctness of an independently written implementation.
- Keep the next action small and actionable. Update affected documentation consistently when the user requests a change; avoid unrelated rewrites.
- Leave unknown details explicit. Do not create dates, schedules, features, or decisions to fill an empty section.
- On an invoked research/guide workflow, save useful research in `research/`, prepare the corresponding `tasks/` tutorial, and update `progress.md` together. Guides should teach the owner to perform the task, preserving the hand-coded policy.
- Guide completion and task execution are distinct. Track evidence source, pending checks, and blockers; never mark code, builds, platform support, or proofs complete because their tutorial was written.
- Use clear section headings, short navigation links within the human docs, and local SVG badges under `assets/badges/`. Badges label documentation state and are not live CI/proof signals. Keep their meaning synchronized with written state and evidence; do not add fabricated percentages or success indicators.
