# Blog Demo (Via + loco.rs)

This example shows how a pair of Via resources (`articles.via`, `comments.via`) can be
generated into Rust + TypeScript artefacts and imported into a loco.rs application.

## Regenerate code

```bash
cargo run --manifest-path ../../via-core/Cargo.toml --bin via -- gen \
  --app app \
  --out generated
```

## Run the app

The binary simply materialises the generated routes to prove they compile. Hook the
`via_generated::controllers::*::routes()` into a real loco.rs application (see
`locors_test` for a full integration).

```bash
cargo run --manifest-path ../blog/Cargo.toml
```

## TypeScript artefacts

Generated TypeScript lives under `generated/ts/` and can be imported via:

```ts
import type { Article, ArticleCreateParams } from './generated/ts';
```

## Next steps

- Replace the placeholder controller bodies with actual database calls.
- Integrate the generated routes into a proper loco.rs `App` (controllers, views,
  migrations).
- Add request specs / integration tests once the app is wired to persistence.
