# Frontend migration to SvelteKit (v1)

Date: 2026-06-17

## Motivation

The frontend's real problem is developer experience, not a missing feature. The
prior server-rendered approach (axum + askama) compiles HTML templates into the
Rust binary, so every template change needs a `cargo` rebuild and restart, with
no hot reload, no component model, and a layout unlike any normal frontend
project. It is slow to iterate on and feels out of place.

The maintainer's recent projects are all SvelteKit, with existing templates and
experience, so SvelteKit is the most ergonomic stack here. Run in SSR mode it
also keeps pages crawlable, which was the original reason the frontend was
rebuilt at all (an agent reading the site saw an empty client-rendered shell).

This project migrates the frontend to SvelteKit at feature parity and gets it
deployed. New interactive features come later, on the new foundation.

## What this retires

- The `frontend-ssr` branch (axum + askama pages) is not merged or deployed. Its
  page-rendering layer is dropped. Its backend modernization (axum 0.8, sqlx
  0.8, handlers split into `src/api/`) is reused. The page layouts and the
  handoff `styles.css` are reference material for the Svelte rebuild.
- The old Svelte SPA currently in `frontend/` on `main` is replaced.

## Architecture

- **Frontend:** a SvelteKit app using `@sveltejs/adapter-node`, server-side
  rendered (crawlable). It lives in `frontend/` as a fresh scaffold that follows
  the house style of the `reference/tasker` project: npm + Node 22, SvelteKit 2
  + Svelte 5 (runes), `adapter-node`, Tailwind v4 + DaisyUI with `@theme` tokens
  in a global stylesheet, strict TypeScript with the `$lib` alias, and Prettier
  + ESLint (flat config) + Vitest + Playwright. It does NOT copy tasker's
  app-specific layers (no Drizzle, no auth, no domain or service layers): this
  app is a thin SSR client over the Rust API and owns no database. The design
  layouts and the handoff `styles.css` are reference for the page markup, redone
  with Tailwind/DaisyUI.
- **Backend:** `murmelbahn-web` becomes API-only. It keeps the `/api/*` routes
  (course `dump`/`bom`/`raw`, `set/list`, `buildable`) and `/metrics`, and drops
  the askama templates, the `pages` module, and static-file serving. It is based
  on the axum 0.8 + sqlx 0.8 backend from `frontend-ssr`. In production it binds
  `127.0.0.1:8080` and is not exposed to the internet directly.
- **Core unchanged:** `murmelbahn-lib` (the parser, BOM, GraviSheet output) is
  untouched.

## Data flow and the public API

- SvelteKit server `load` functions fetch the Rust API directly using an
  environment variable for its base URL, `INTERNAL_API` (for example
  `http://localhost:8080`).
- The public `/api/*` surface stays available at the same domain. External
  consumers and the GraviSheet CSV link (`/api/course/{code}/bom?format=csv`)
  depend on it. A single SvelteKit `handle` hook in `hooks.server.ts` forwards
  any request whose path starts with `/api/` to `INTERNAL_API`, returning the
  upstream response unchanged. This one mechanism works in both dev and prod.

## Development

Two normal processes, no Rust rebuild to see a frontend change:

- `cargo run` runs the API on `:8080`.
- `npm run dev` runs SvelteKit (vite) with hot module reload on its dev port.
- `/api/*` requests proxy to `:8080` (via the same `handle` hook, or vite's dev
  proxy, whichever the template favors).

## Production on Fly (one container)

- A multi-stage Dockerfile: build the Rust binary, build the SvelteKit app, then
  a final image that carries both plus `s6-overlay` as the process supervisor.
- `s6-overlay` runs two services: the Rust API on `127.0.0.1:8080` and the
  SvelteKit Node server on `:3000`. If either exits, the supervisor handles it
  per its configured policy so the container does not sit half-up.
- `fly.toml`: `internal_port = 3000` (SvelteKit is the public face). Secrets and
  env: `DATABASE_URL` and `SETS_DIRECTORY` for Rust, `INTERNAL_API` for
  SvelteKit. Deployment stays manual (`flyctl deploy`); one deploy ships both,
  so frontend and backend are always version-locked.

## Scope (v1)

In scope:

- SvelteKit reimplementation of the existing routes: home with course-code
  lookup, `/course/{code}` (metadata + bill-of-materials), `/sets`, `/build`
  (inventory tool with localStorage persistence and reset), `/about`.
- The public `/api/*` passthrough.
- The SEO and GEO layer (per-page meta, JSON-LD, `/sitemap.xml`, `/robots.txt`,
  `/llms.txt`), described below.
- GitHub Actions CI for the Rust crates and the frontend, checks only, described
  below.
- The one-container Fly deployment with `s6-overlay`.

Out of scope (later specs, on the new foundation):

- The 3D course viewer, browse/search/sort/filter, server-side i18n (`/{lang}/`),
  and any other new interactive features.

## SEO and GEO

Replicate the pattern from `reference/stackable-apps/apps/hub`. This is exactly
the crawlability and agent-readability the frontend rebuild was started for (an
agent reading the site saw an empty shell).

- `src/lib/seo.ts`: `SITE_NAME`, a default title and description, and a
  `buildPageTitle(pageTitle)` helper.
- `src/lib/components/SeoHead.svelte`: one component rendered once in the root
  layout. It reads `page.data.seo` plus `origin`/`path` and emits the title,
  description, canonical link, Open Graph and Twitter tags, and a permissive
  `robots` directive.
- Each `+page.server.ts` returns a `seo` object (`title`, `description`); the
  root `+layout.server.ts` returns `origin` and `path` from the request URL so
  canonical and Open Graph URLs are absolute.
- `src/lib/jsonld.ts`: a helper that escapes and injects JSON-LD via `{@html ...}`
  inside `svelte:head`. Add schema.org data where it fits: a `WebSite` and
  `Organization` block on the home page, and a per-course block on
  `/course/{code}` (for example a `CreativeWork` naming the course).
- `/sitemap.xml` as a `+server.ts` endpoint: the static pages, plus optionally
  the cached courses (queried through the Rust API). Cache for an hour.
- `/robots.txt` as a `+server.ts` endpoint: allow everything and explicitly
  welcome the named AI crawlers (GPTBot, ChatGPT-User, ClaudeBot, PerplexityBot,
  Google-Extended, and the rest of the hub list), pointing at `/sitemap.xml` and
  `/llms.txt`.
- `/llms.txt` as a `+server.ts` endpoint following the llmstxt.org convention: a
  short markdown summary of the site and the public `/api/*` so agents can use
  the data. Build it from a shared function so a later `/llms` HTML preview can
  reuse it.

Rendering stays SSR (no prerender), because the course and sets pages are
dynamic.

## Continuous integration

Two GitHub Actions workflows, checks only. Deployment stays manual via `flyctl
deploy`. Use the maintained actions, not the deprecated `actions-rs/*` ones that
the `stackablectl` reference uses.

- `.github/workflows/rust-ci.yml`: triggers on push to `main` and on pull
  requests, path-filtered to the Rust crates, `Cargo.*`, and the workflow file.
  One job on `ubuntu-latest`: `dtolnay/rust-toolchain@stable` (components
  `rustfmt`, `clippy`), `Swatinem/rust-cache`, then `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets` with warnings denied, and `cargo test
  --workspace`. Add a `concurrency` group with `cancel-in-progress`. Independent
  of the frontend, so it can land first.
- `.github/workflows/frontend-ci.yml`: triggers on push to `main` and pull
  requests, path-filtered to `frontend/**` and the workflow file. One job on
  `ubuntu-latest`: `actions/setup-node@v4` (Node 22, npm cache keyed to
  `frontend/package-lock.json`), `npm ci`, then `npm run lint`, `npm run check`,
  `npm run test:unit -- --run`, and `npm run build`. Only meaningful once the
  SvelteKit app exists in `frontend/`.

## Testing

- The Rust API tests stay as they are.
- The frontend uses whatever testing the maintainer's template ships (for
  example vitest and Playwright).
- A short post-deploy smoke check: pages server-render with real content, and
  `/api/*` passes through to the Rust backend.

## Sequencing

The frontend conventions are known from `reference/tasker` and the app is a
fresh scaffold, so the plan does not need to wait for a project copy-in. Land in
this order, each step independently shippable:

1. Rust CI (`rust-ci.yml`) on the current backend. Independent quick win that
   also starts surfacing the backend's checks.
2. Backend becomes API-only on axum 0.8: reuse the `frontend-ssr` modernization
   (axum 0.8, sqlx 0.8, `src/api/`), drop the page-rendering layer, bind
   `127.0.0.1`.
3. Scaffold the SvelteKit app in `frontend/`, reimplement the routes, add the
   SEO/GEO layer, wire the `/api` passthrough, and add `frontend-ci.yml`.
4. The one-container Dockerfile + `s6-overlay` + `fly.toml`, then deploy.
