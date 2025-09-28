# Viaduct (Via)

Rails ergonomics, Rust performance. Viaduct is a high‑level web framework that adds a concise, typed DSL (Via) on top of [loco.rs](https://loco.rs/), compiling to idiomatic Rust for production‑grade speed and safety.

- DSL: Via (`.via`) with strong typing and pervasive inference
- Defaults: HTML + JSON, automatic CRUD, JSON:API endpoints
- Responders: DRY content negotiation for HTML/JSON
- Params: typed schemas, coercions, structured error responses
- Auth/RBAC: users, sessions/CSRF, JWT, OAuth, roles/permissions (extensions)
- DX: watch mode to regenerate Rust and run via `cargo`
- Extensibility: escape hatches, partials, ejection, and a first‑class plugin system
- Observability: structured JSON logs and OpenTelemetry
- TypeScript E2E types: generates TS models alongside Rust code
- No‑breaking upgrades: codemods auto‑migrate your code across versions

Status: **exploratory/experimental**. Scope evolves as ideas crystallize. MIT licensed.

## MVP 0.1 (October 2025)

The first end-to-end spike is now checked in under `via-core/`. It provides a minimal
`via` CLI that parses `.via` resources and emits generated Rust stubs plus an IR JSON
artifact.

What works today:

- `resource` files with `model` and `controller` blocks.
- `model` fields with optional `?` suffix and `serialize: false` flag.
- `controller` sections for `params { editable { ... } }`, `respond_with […]`, and
  `actions auto_crud`.
- Codegen for `generated/src/models/*.rs`, `generated/src/controllers/*.rs`,
  `generated/src/{lib,models/mod,controllers/mod}.rs`, and `generated/via.ir.json`.
- Generated Rust ships as a standalone crate (`generated/` → `via-generated`) that is
  wired into the sample `locors_test` loco.rs app.
- The generator owns `generated/Cargo.toml`; it is rewritten on each run with the
  dependencies required by the stubs.

Try it from the repo root:

```bash
cargo run --manifest-path via-core/Cargo.toml --bin via -- gen \
  --app app \
  --out generated
```

This reads files under `app/` (see `app/resources/articles.via` for the sample resource)
and regenerates the Rust stubs in `generated/`.

You can confirm the integration with the example loco.rs app by running:

```bash
cargo check --manifest-path locors_test/Cargo.toml
```

DSL subset implemented in the MVP:

- `resource Name { … }` with single `model { … }` and `controller { … }` blocks.
- `field name: Type` and `field name?: Type` (nullability inferred); `serialize: false`.
- Built-in types mapped to Rust primitives (`String`, `bool`, `i64`, etc.). Date/time
  types currently map to `String` with TODO comments.
- `params { editable { field, … } }` expands to `NameCreateParams` and
  `NameUpdateParams` structs.
- `respond_with [html, json]` captured as metadata constant.
- Auto-generated controller stubs for CRUD actions plus route scaffolding.
- Controller stubs accept `State<AppContext>` and return JSON placeholders (`{"todo":
  "Article#index"}`) so the generated crate compiles cleanly inside a loco.rs app.

Known limitations (tracked in PROJECT.md):

- Controllers emit placeholder handlers with `todo!()` and minimal signatures; they do
  not yet wire into `AppContext` or real responders.
- Watch mode, incremental rebuilds, and plugin hooks are not implemented.
- No grammar support yet for `policy`, inline `rust { ... }`, `respond { ... }`, or
  per-action overrides; the README examples below remain aspirational.
- Generated code currently assumes the consumer will add the appropriate `loco.rs`
  imports and update routing glue in the host app.

## Example

```ruby
model Post {
  field title: String
  field description?: Text
  field published_at?: DateTime
  field hidden: Boolean serialize: false

  belongs_to user
  has_many comments
  belongs_to owner: User
  belongs_to commentable: Polymorphic<Post, Image>

  index(title)
  validate title presence

  slot before_save {
    rust {
      # normalize title, compute slugs, etc.
    }
  }
}
```

```ruby
controller Post {
  params {
    # 'editable' is shorthand for create + update
    editable { title, description }
  }

  respond_with [html, json]

  # Call `skip_default_actions` to skip default CRUD actions
  # (new, create, edit, update, show, destroy)

  # Override the default show action
  action show {
    @post = Post.find(params.id)
    @team_name = post&.author&.team&.name

    # This respond block is the default behavior and can be omitted.
    respond {
      html { render "posts/show" }
      json { @post }
    }
  }

  # Override an action with custom Rust code
  action create override -> rust("src/controllers/posts.rs#create")
}
```

```ruby
policy Post {
  scope {
    # where(team_id: current_team.id)
  }
  rules {
    index   { true }
    show    { record.team_id == current_team.id }
    create  { user.role == admin || user.role == editor }
    update  { user.role == admin || user.role == editor }
    destroy { user.role == admin }
  }
}
```

## Layered Stack

- Rust
- [loco.rs](https://loco.rs/) (runtime, routing, ORM, jobs, scaffolding)
- Via DSL → generates `loco.rs` controllers/models/policies/views
- Extensions (auth, responders, params, conventions)
- Starter kit (admin, webhooks, Stripe, modern frontend integrations)

### Generated TypeScript (example)

From the Via resource above, `via gen types` emits:

```ts
// models/Post.ts
export interface Post {
  id: string; // UUID or DB id (configurable)
  title: string;
  description?: string;
  publishedAt?: string; // ISO 8601 DateTime
  // `hidden` is model-only (serialize: false) and not present in API output

  userId: string; // belongs_to user (inferred)
  ownerId?: string; // belongs_to owner: User
  commentable?: {
    type: "Post" | "Image"; // polymorphic target
    id: string;
  };
}

// Params inferred from `params { editable { title, description } }`
export type PostCreateParams = {
  title: string;
  description?: string;
};

export type PostUpdateParams = {
  title?: string;
  description?: string;
};
```

## Type Safety & Upgrades

- End‑to‑end type safety: Via definitions emit both idiomatic `loco.rs` code and generated TypeScript models/schemas for client apps. Keep server and frontend in lockstep.
- Zero breaking changes philosophy: every Viaduct release ships with a codemod (either automatically generated or hand-written) that upgrades your Via code and generated outputs. Includes seamless migrations across underlying `loco.rs` or Rust version shifts when needed.

## Packages (planned)

- `via-core` — parser, type inference, codegen, watch, hooks API
- `via-responders` — responders DSL + codegen
- `via-auth` — users, sessions/CSRF, JWT, OAuth, RBAC primitives
- `viaduct-starter-kit` — admin, webhooks, Stripe, Vite/TanStack/Next/Vue, Tailwind

## Docs & References

- Project vision and roadmap: PROJECT.md
- Via grammar (draft EBNF): docs/via.ebnf
- Agent/developer guidance: AGENTS.md
- loco.rs: https://loco.rs/

## Tooling Preferences

- Rust, Cargo, clippy
- TypeScript, Pnpm, Vite, TanStack, Biome (+ Ultracite)
- Starlight (Astro) for docs
- Lefthook, Taskfile, Mise

## License

MIT
