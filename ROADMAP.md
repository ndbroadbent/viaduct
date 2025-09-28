# Viaduct Roadmap

## Legend
- ✅ Delivered (available on main branch)
- 🚧 In Progress / Upcoming
- 🧭 Open Questions / Research Topics

---

## 1. Foundations
### 1.1 Via DSL & Codegen
- ✅ `via-core` crate with CLI (`via gen`, `via check`).
- ✅ Grammar + parser coverage for `resource`, `model`, `controller`, `params`, `respond_with`, `actions auto_crud`.
- ✅ Rust code generation for models (with serialize hints, optional fields) and controllers (routes, stubs).
- ✅ TypeScript generation (interfaces + params) with barrel export.
- ✅ Snapshot tests for codegen outputs (`via-core/tests/codegen_snapshots.rs`).
- 🚧 Extend grammar: inline action bodies, policy blocks, associations (belongs_to/has_many), respond blocks, slots, inline rust escapes.
- 🚧 Surface metadata in IR (resource id types, association info, respond formats).
- 🚧 Provide serde-lens for customizing field serialization (rename, omit_if_nil, computed fields).
- 🧭 Decide on DSL syntax for validators, enums, computed properties.

### 1.2 Generator Ergonomics
- ✅ Generator rewrites `generated/Cargo.toml` on each run.
- ✅ Prevent glob re-export warnings in generated modules.
- 🚧 Multi-resource scaffolding (derived file structure + aggregated IR).
- 🚧 `via watch` command for incremental regeneration + runners.
- ✅ CLI smoke tests (`assert_cmd`) covering errors & happy paths (gen/check success + failure cases).
- 🧭 Evaluate caching strategy for IR (timestamp vs hash) to avoid redundant writes.

---

## 2. Demo Applications
### 2.1 `locors_test` Integration Repo
- ✅ Article resource generated + wired into loco.rs app.
- ✅ Comment resource added and routes mounted.
- 🚧 Replace placeholder controller bodies with real SeaORM CRUD + respond negotiation (HTML + JSON).
- 🚧 Generate/request migrations for new resources (SeaORM entities or direct SQL).
- 🚧 Seed data in dev/test (fixture YAML or generated seeds).
- 🚧 Add integration tests hitting generated routes (use loco testing tools / axum router).
- 🚧 Document manual steps (migrate, seed, run) and provide automation via Taskfile or cargo xtask.
- 🧭 Decide on approach for views/templates (Askama vs HTML placeholders).

### 2.2 `examples/blog` Demo
- ✅ Minimal Articles & Comments Via resources, generated outputs, README instructions.
- 🚧 Wire to an Axum router + in-memory storage (or embed loco.rs context) for a walkthrough.
- 🚧 Provide HTML demo (list/articles) and CLI instructions (curl commands).
- 🚧 Add `Taskfile.yml` to regenerate, migrate, run server.
- 🚧 Add runtime tests (reqwest-based) verifying endpoints.
- 🧭 Optionally, publish the demo as a separate crate or dockerized example.

---

## 3. Testing Strategy
### 3.1 Codegen & Parser
- ✅ Snapshot tests for sample resource (Rust + TS + manifest).
- 🚧 Add fixtures covering optional fields, serialize flags, respond_with custom list, associations.
- 🚧 Parser unit tests capturing error diagnostics (invalid syntax, missing sections).
- ✅ CLI smoke tests to `cargo check` generated crate outputs (ensures generated crate compiles).

### 3.2 Runtime (Generated Apps)
- 🚧 Request specs in `locors_test/tests` hitting generated routes, verifying DB side effects, JSON payloads.
- 🚧 Setup test environment (sqlite in-memory + migrations per test run).
- 🚧 Provide test helpers for generating Via snapshots + verifying compiled code compiles (integration harness).
- 🧭 Consider golden tests for HTMX/HTML templates once views exist.

### 3.3 TypeScript
- 🚧 Add Node/PNPM workspace for `generated/ts`, run `tsc --noEmit` in CI.
- 🚧 Add a Vitest spec to ensure types compose correctly (e.g., create payload typed as `ArticleCreateParams`).

---

## 4. Feature Enhancements
### 4.1 Controller Behavior & Responders
- 🚧 Auto-generate HTML responders (render via templates), support JSON/HTML negotiation.
- 🚧 Include success/error responses, status codes, error handling templates.
- 🚧 Support nested routes and association-specific endpoints (e.g., comments under articles).
- 🧭 Determine strategy for background jobs, policies, validations within DSL.

### 4.2 Models & Associations
- 🚧 DSL syntax for associations (belongs_to, has_many, polymorphic) translating to SeaORM relations.
- 🚧 Generate SeaORM entity modifications or re-run entity generation automatically.
- 🚧 Derive argument types (UUID vs integer) and propagate into controllers + TS.

### 4.3 CLI & Tooling
- 🚧 `via new` scaffold command for new Via projects.
- 🚧 `via gen types` (TS-only output) and `via gen rust` (Rust-only) for selective regeneration.
- 🚧 Support config file or app manifest to specify defaults (output dirs, crate metadata).
- 🧭 Evaluate plugin API for custom codegen (responders/auth/analytics).

---

## 5. Documentation & DX
- ✅ README + PROJECT narrative updated for MVP.
- ✅ ROADMAP tracking deliverables.
- 🚧 Document usage of `via-core` CLI (flags, environment variables).
- 🚧 API reference for generated Rust (module layout, naming conventions).
- 🚧 Tutorial: “From Via file to running loco.rs app” with code snippets.
- 🧭 Plan doc site (Starlight) aligning with DSL grammar reference, cookbook, plugin guide.

---

## 6. Release Prep & CI
- 🚧 Set up CI pipeline (fmt, clippy, via-core tests, generator smoke tests, TS type checks).
- 🚧 Pre-commit integration (Lefthook) for formatting / test hooks.
- 🚧 Versioning strategy for generator + compatibility matrix (via-core vs generated crate).
- 🧭 Determine distribution model (crate release, GitHub template, docs hosting).
