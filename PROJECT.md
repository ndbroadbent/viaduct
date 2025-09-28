# Viaduct (Via): Rails Ergonomics, Rust Performance

This project explores a high‑level web framework that delivers the flexibility and productivity of Ruby on Rails while compiling to Rust for performance and memory safety. It tightly targets and builds on top of [loco.rs](https://loco.rs/), adding a new mini‑language (DSL) and batteries‑included conventions that generate idiomatic Rust code.

License: MIT • Tone: pragmatic with a streak of visionary


## Audience
- Primary: Rails developers seeking Rust’s performance without sacrificing ergonomics.
- Secondary: Any teams building modern web apps that value convention over configuration.


## MVP Snapshot (Oct 2025)
- `via-core/` crate exposes a `via` CLI (`gen` + `check`).
- DSL support: `resource` with `model` + `controller`, `field` optionality via `?`,
  `serialize: false`, `params { editable { … } }`, `respond_with [ … ]`, and
  `actions auto_crud`.
- Code generation emits `generated/src/models/*.rs`, `generated/src/controllers/*.rs`,
  root `mod.rs` files, and `generated/via.ir.json` for downstream tooling.
- Example input lives at `app/resources/articles.via` and represents the current grammar
  baseline.
- Generated code is compiled as a `via-generated` crate and consumed by the
  `locors_test` sample app (see `App::routes` for how the routes are wired).
- `generated/Cargo.toml` is owned by the generator; reruns will rewrite it with the
  dependencies needed for the emitted stubs.
- TypeScript interfaces are emitted under `generated/ts/` with a barrel export to ease
  consumption from front-end apps.
- Controller stubs respond with JSON placeholders such as
  `{ "todo": "Resource#update", "id": "...", "payload": {...} }` and accept
  `State<AppContext)` plus appropriate `Path` / `Json` extractors so the host app can
  compile/run immediately.
- `examples/blog` demonstrates a minimal multi-resource Via app (Articles & Comments)
  with generated Rust + TypeScript artefacts.
- Controllers currently stub handlers with `todo!()`; wiring into `AppContext`
  responders/policies/jobs remains follow-up work.
- Missing pieces: watch mode, granular actions, policies, inline Rust escapes, richer
  type mapping, association DSL, and plugin hooks.


## Naming (decision)
- Project name: Viaduct
- DSL: Via (`.via` files)
- CLI: `via` (e.g., `via new`, `via watch`, `via gen resource Post`)
- Starter kit: `viaduct-starter-kit`
- Packages: `via-core`, `via-auth`, `via-responders`

Runner‑up (not chosen for now): Conductor (CLI `cnd`, extension `.cdsl`).

Use with caution or avoid: Railgun (`.rg`, clashes with ripgrep), Ferrum (Ruby gem exists), Switchyard (`yard` collision), Alloy (conflicts with Alloy language), Signal, Roundhouse.


## Layered Architecture
From lowest to highest level:

0. Rust (language)
1. loco.rs (runtime, scaffolding, jobs, routes, ORM via SeaORM, etc.)
2. Via mini‑language (`.via`) that compiles to `loco.rs` models/controllers/policies/views
3. Extensions & conventions layer (auth, sessions, JWT, RBAC, OAuth, responders, params)
4. Starter kit (webhooks, admin panel, Stripe, modern frontend integrations, Tailwind)

Tight coupling: v1+ is explicitly built on `loco.rs`.


## Core Goals
- Rails‑like developer experience with statically‑checked, inferred types.
- Default HTML+API with automatic CRUD and JSON:API endpoints.
- “Responders” pattern to DRY action responses (HTML/JSON negotiation).
- Strong, ergonomic parameter typing/validation with useful error responses.
- Escape hatches everywhere: inline Rust, partial overrides, full ejection.
- Watch mode that regenerates Rust from the DSL and rebuilds/runs automatically.
- Structured JSON logging and tracing with OpenTelemetry from day one.
- A rich plugin/hook system to extend codegen and behavior safely.
 - End‑to‑end type safety with TypeScript: generate TS models from Via definitions.
 - No‑breaking upgrades: ship codemods that migrate user code and generated outputs (including `loco.rs`/Rust transitions).

Non‑goals (core layer): frontend/Vite/Tailwind/Stripe (live in Starter Kit); Dockerfile (covered by `loco.rs`).


## The Mini‑Language (Via DSL)
A new, statically typed but highly inferred language that compiles to Rust (`loco.rs` idioms).

Principles
- Types inferred from a single declaration point (e.g., model fields or DB schema).
- Strong typing with ergonomic affordances: enums with exhaustiveness checks, optional chaining (`&.`), implicit param coercions (e.g., string → integer) where safe.
- Concise, Rails‑like expressiveness; DRY; convention over configuration.
- Aware of Rust ownership/borrowing but abstracts it away; only surfaces when truly needed.

Ergonomic Safety
- Nullability with `Option` semantics but without explicit `Option<T>` notation in user code.
- Optional chaining: `post&.author&.team&.name` short‑circuits to `nil`/`None`.
- Typed params with defaults, transformations, and coercions.
- Exhaustive `match` enforcement for enums and responders.

Escape Hatches
- Inline Rust blocks inside DSL definitions.
- Partials/slots to customize key lifecycle points.
- Eject any action/controller/model/policy to hand‑written Rust, retaining the rest of the DSL.


## DSL Sketches (illustrative)
Note: Syntax is exploratory; exact grammar TBD.

Resource + Model
```
resource Post {
  model {
    field title: String
    field description?: Text
    field published_at?: DateTime

    belongs_to user: User
    has_many comments: Comment
  }

  # Typed params (create/update) with defaults and coercions
  params {
    create { title: String, description?: Text }
    update { title?: String, description?: Text }
  }

  # Responders: negotiate HTML/JSON automatically
  respond_with [html, json]

  # Slots for customization (filled inline or by plugins)
  slot before_save { /* rust { … } or DSL */ }
  slot after_save  { /* optional */ }

  # Example: inline Rust escape hatch
  on publish { rust {
    use crate::workers::ReportWorker;
    ReportWorker::perform_later(ctx, ReportArgs { user_id: current_user.id })?;
  }}
}
```

Controller Actions (defaulted)
```
resource Post {
  actions auto_crud
  # Eject single action when needed
  action create override -> rust("src/controllers/posts.rs#create")
}
```

Policies (Pundit/CanCan‑style)
```
policy PostPolicy {
  scope { where(team_id: current_team.id) }  # row‑level scoping
  rules {
    index  { allow if user.role.in([admin, editor, viewer]) }
    show   { allow if record.team_id == current_team.id }
    create { allow if user.role.in([admin, editor]) }
    update { allow if user.role.in([admin, editor]) }
    destroy{ allow if user.role == admin }
  }
}
```

Responders
```
action show {
  let post = Post.find(params.id)?
  respond {
    html { render "posts/show", post }
    json { post }
  }
}
```

Optional Chaining
```
let team_name = post&.author&.team&.name  # None if any link is missing
```


## Auth, Sessions, RBAC, OAuth (Layer 3)
- Built‑in: users, teams/orgs, memberships, roles, permissions; flexible RBAC policies.
- Auth flavors: cookie sessions + CSRF (HTML), JWT (API), OAuth providers.
- Default models and endpoints with ejection/overrides as needed.
- Policy layer integrates across controllers/models and row‑level scoping.


## Jobs & Background Processing
- Delegate to `loco.rs` job system (retries/backoff/cron/DLQ as provided).
- Write job logic in Rust; DSL may later add sugar for scheduling/invocation hooks.


## Observability & Ops
- Structured JSON logs using `tracing`.
- OpenTelemetry traces/metrics; action spans around routing, DB, templates, jobs.
- Environments/config: follow `loco.rs` (dev/test/prod; env vars + config files).


## Code Generation & Layout (proposal)
- Keep generated Rust separate from user‑authored Rust to allow safe regeneration.
- Proposed layout:
  - `app/` — human‑authored code in the DSL: `app/models`, `app/resources`, `app/policies`, `app/views` (if applicable)
  - `generated/` — compiler output in Rust mirroring a `loco.rs` project tree
  - `src/` — user hand‑written Rust (ejected files, custom jobs, low‑level code)
  - `config/` — framework + `loco.rs` config
- Regeneration is idempotent and non‑destructive. Custom code lives outside `generated/`.
- Partial overrides and named slots allow customization without full ejection.


## Dev Workflow
- `via watch` watches `app/` for `.via` DSL changes, regenerates `generated/` Rust, then runs `cargo` (or delegates to the `loco.rs` dev server) for hot iterations.
- Delegate commands like `db:migrate`, `db:seed`, `jobs`, `test` to `loco.rs` where appropriate.
- Console/REPL is out of initial scope; may later expose a params/policy playground.
 - TypeScript output: `via gen types` emits synchronized TS models/schemas; `via watch` can also emit TS on change.


## Responders & Params (Details)
- Responders unify action outcomes for HTML/JSON with negotiated content types.
- Params are inferred from model fields when types are omitted; explicit types remain supported.
- Shorthand `editable { ... }` expands to both `create` and `update` profiles; requiredness/optional behavior derives from nullability/defaults/validations.
- Typed params enforce schemas at the controller boundary with clear error responses.
- Common conversions (string→int, string→date) applied safely; errors reported in a structured way (422 with details for API; flash + form errors for HTML).

## Serialization
- Field-level controls on the model determine API exposure by default:
  - `field hidden: Boolean serialize: false` excludes a field from serialized output and generated TS types.
  - Future: `serialize: rename("publishedAt")`, `omit_if_nil`, computed attributes via slots.
- Responders use these rules to render JSON/JSON:API; per-action overrides can adjust exposure.

## Typed HTML Views
- Goal: compile human-friendly template files into typed Askama structs to catch errors at compile time while keeping templates readable.
- Source of truth remains plain templates (e.g., `assets/views/posts/show.html`) using Jinja-like syntax (`{{ var }}`, `{% for %}`), but codegen emits:
  - An Askama struct with the correct fields and lifetimes.
  - A render shim that returns a `Response` and integrates with `loco.rs`.
- In Via, controller `respond` blocks declare which template to render and the context; types are inferred from model names, with optional annotations for clarity.

Example (Via):
```
controller Post {
  action show {
    let post = Post.find(params.id)
    respond { html { render "posts/show", post } }
  }
}
```

Generated (Rust):
```
#[derive(Template)]
#[template(path = "posts/show.html", ext = "html")]
struct PostsShow<'a> { post: &'a posts::Model }

fn render_post_show(v: &impl ViewRenderer, post: &posts::Model) -> Result<Response> {
    let body = PostsShow { post }.render()?;
    format::render().html(body)
}
```

- Lists, maps, and nested contexts are supported (e.g., `items: &'a [posts::Model]`).
- Partials and slots compile into additional typed templates.
- Tera remains available as a plugin for teams that prefer dynamic templates; default path favors typed Askama for compile-time safety.


## Plugin System (Critical from Day 1)
- Plugins provide:
  - New DSL constructs or annotations.
  - Codegen templates/partials for resources, policies, responders, params, telemetry.
  - Lifecycle hooks: project init, before/after generate, before/after build, per‑resource slots (e.g., `params_filter`, `authorize`, `serialize_json`, `before_save`, `after_commit`).
- Versioned packages; stable hook interfaces; conflict detection and clear precedence rules.


## Milestones (non‑binding, exploratory)
1. Grammar + parser for the DSL; AST with types and nullability.
2. Codegen to idiomatic `loco.rs` for: models, resources (routes+controllers), policies, responders.
3. Typed params: schema → validation → structured errors.
4. Watch mode: incremental regenerate → `cargo` build/run; source maps back to DSL.
5. Auth/RBAC layer: users, teams, memberships, roles/permissions; sessions + JWT; pluggable OAuth.
6. Observability: JSON logs + OpenTelemetry spans/metrics out of the box.
7. Plugin/hook system: stabilize core extension points; publish example plugins.
8. TypeScript generation: emit TS models/schemas and an example TS app consuming them.
9. Starter kit (separate package): admin panel, webhooks, Stripe, modern frontend stacks.

## Versioning & Upgrades
- Zero‑breaking‑changes commitment at the Via level.
- Each release includes a codemod that:
  - Updates Via syntax/usages in user code.
  - Regenerates Rust and TS outputs with any required transformations.
  - Applies safe migrations for underlying `loco.rs` and Rust toolchain shifts when necessary.
- Codemods are idempotent, testable, and provide dry‑run and diff modes.


## What We Will Not Rebuild
- ORM, migrations, associations, validations, callbacks: use `loco.rs`/SeaORM features, extend where ergonomic.
- loco.rs scaffolding and dev commands: prefer delegation.
- Dockerfile generation: defer to `loco.rs`.


## Open Questions
- Exact DSL syntax and file extensions (`.lrs`, `.cdsl`, etc.).
- Directory integration with `loco.rs` defaults (confirm best alignment).
- Depth of default generators for HTML scaffolds given `loco.rs` coverage.
- Plugin packaging format and versioning constraints.
- Source maps and error reporting from Rust back to DSL lines.


## Next Steps
- Harden the `via` CLI UX: discover project roots automatically, ship a binary name
  alias, surface rich parse errors, and add `--watch` plus dry-run/verbose modes.
- Extend the grammar to cover actions with bodies, `policy` blocks, inline `respond`
  declarations, associations, slots, and inline `rust { ... }` escapes.
- Improve type fidelity: map dates/times to `chrono`, UUIDs to `uuid`, allow explicit
  imports, and surface compile-time hints in generated code + IR.
- Wire generated controllers into a real `loco.rs` app (`locors_test/`) with real
  responders, params extraction, persistence, and tests.
- Emit TypeScript models (and OpenAPI metadata) from the shared IR alongside Rust.
- Design and implement the plugin/hook API; provide at least one responder and one auth
  plugin as reference implementations.
- Deliver `via watch` for incremental regeneration plus an opt-in `build.rs`
  integration path.

## Packages (initial layout)
- `via-core`: parser, type inference, codegen to `loco.rs`, watch pipeline, hooks API.
- `via-responders`: responders DSL + codegen, content negotiation, HTML/JSON bridges.
- `via-auth`: users, sessions/CSRF, JWT, OAuth providers, RBAC primitives and policies.
- `viaduct-starter-kit`: admin panel, webhooks, Stripe, frontend integrations (separate package).
- `via-types` (planned): generates TypeScript models/schemas and client helpers from Via definitions.

## Tooling Preferences
- Rust, TypeScript
- pnpm for JS package management
- Vite + TanStack for frontend integrations (starter kit)
- Biome (+ Ultracite) for JS/TS linting
- Starlight (Astro) for documentation site
- Lefthook for pre-commit hooks
- Taskfile for task automation
- Mise for version management

## Boundary & Codegen
- Source of truth: Via DSL in `app/` (models, controllers, policies, views).
- Generated code is disposable; user code is not. Never edit `generated/`.
- `src/` remains open for custom Rust. Any unit can be ejected from `generated/` into `src/`.

Directory ownership (proposed)
- `app/` — Via DSL: `app/models`, `app/controllers`, `app/policies`, `app/views`.
- `generated/` — Via → loco.rs output: controllers, policies, typed views, migrations.
- `src/` — user-authored Rust: ejected files, custom jobs, initializers, integrations.
- `migration/` — SeaORM migrations generated by Via (including `lib.rs`, built dynamically).
- `assets/` — human-friendly templates; Via compiles to typed Askama structs.

Workflow
- `via watch` regenerates `generated/` and `migration/`, then runs `cargo` (or `cargo loco start`).
- Regeneration is idempotent. `src/` is never clobbered.
- Ejection: `via eject controller Post` (or `model`, `policy`, `action`, `view`) moves a unit to `src/` and marks it as user-owned.

Migrations
- Model-first: Via models define schema. `via gen migration` diffs AST vs last applied and emits forward/backward migrations into `migration/src/*`.
- `migration/src/lib.rs` is generated; do not edit by hand.
- Raw SQL blocks supported inside a Via migration when necessary.

Entities
- Interim: Via can call `cargo loco db entities` after migrations to refresh SeaORM entities.
- Later: Via directly generates SeaORM entities from models to remove the extra step.

Views
- Source templates in `assets/views` (Jinja-like). Via emits typed Askama structs and render shims.
- Controllers call typed renderers; missing fields become compile errors.

Generators
- Wrap most generators (models/controllers/policies/views/migrations) in Via.
- Delegate loco-native generators for niche areas (jobs, mailers, scheduler, deployment) unless Via adds ergonomic value.

CLI
- Via: `new`, `gen`, `watch`, `gen types`, `gen migration`, `eject`.
- Delegate: `cargo loco start`, `db migrate/reset`, `jobs`, `scheduler`.

## Dependency Wiring (IoC)
- Explicit wiring; no reflection-based containers or service locator.
- Composition root builds an `App`/`Context`; pass small grouped deps to controllers/jobs/policies.
- Define ports as traits; implement adapters (real and fakes). Use `Arc<dyn Trait>` for pluggability or generics `T: Trait` for zero-cost wiring.
- Prefer builders for complex setup; construct at the edges, not inside handlers.
- Testing swaps fakes at the composition root; no mocking framework required.
- Via integration: generated code requests only declared deps; plugins register providers explicitly.
