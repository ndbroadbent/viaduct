# AGENTS.md

This file guides agents and contributors working in this repository. It applies to the entire repo unless a more specific AGENTS.md appears in a subdirectory.

## Principles
- Favor small, incremental changes with clear rationale.
- Keep ergonomics high: DRY, convention over configuration.
- Prefer explicit extension points (hooks/slots) over ad‑hoc customization.
- Separate generated code from human‑authored code; regeneration must be safe.

## Current Project Shape
- Name: Viaduct (DSL: Via, `.via` files). CLI: `via`.
- Core docs: `PROJECT.md` for vision/roadmap; `docs/via.ebnf` for grammar draft.

## Code/Layout Conventions (proposed)
- Via DSL sources live under `app/` (e.g., `app/resources`, `app/models`, `app/policies`).
- Generated Rust targets `generated/` and mirrors a `loco.rs` app layout.
- Hand‑written Rust (ejected code, custom jobs/integrations) lives under `src/`.
- Config under `config/`; defer to `loco.rs` conventions where possible.

## Hooks/Slots (directional)
- Provide named slots for lifecycle points: `before_save`, `after_commit`, `authorize`, `params_filter`, `serialize_json`, etc.
- Support inline Rust blocks inside Via as escape hatches; allow ejection per action/model/policy.

## Dependencies & Delegation
- Build on `loco.rs` for routing, controllers, models/ORM, migrations, jobs, scaffolding.
- Do not reimplement `loco.rs` features unless ergonomics require a thin shim.

## Tooling Preferences
- Languages: Rust, TypeScript.
- JS/TS: pnpm, Vite, TanStack, Biome (+ Ultracite).
- Docs: Starlight (Astro).
- Automation: Taskfile for tasks, Lefthook for pre‑commit hooks, Mise for version management.

## Working Agreements
- Generated code must be reproducible and idempotent.
- Keep user custom code outside `generated/`; never overwrite without an explicit opt‑in.
- Prefer additive changes to the Via grammar; document breaking changes in `PROJECT.md`.
- When in doubt, align with `loco.rs` idioms and naming.

## Out of Scope (for now)
- Dockerfiles (handled by `loco.rs`).
- Frontend scaffolding (lives in the separate `viaduct-starter-kit`).

## Next Steps for Agents
- Iterate on `docs/via.ebnf` to refine grammar; keep it minimal yet expressive.
- Prototype parser/AST in `via-core`; plan watch mode and codegen boundaries.
- Propose plugin/hook interfaces early and keep them stable.

