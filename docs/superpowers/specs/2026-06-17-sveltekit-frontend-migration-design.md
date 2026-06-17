# Frontend migration to SvelteKit, v1 (parity)

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
  rendered (crawlable). It lives in `frontend/`, started from one of the
  maintainer's existing SvelteKit templates, so its internal conventions
  (styling, component library, lint/format/test) come from that template. This
  spec adapts to that scaffold rather than prescribing one.
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

## Scope (v1, parity only)

In scope:

- SvelteKit reimplementation of the existing routes: home with course-code
  lookup, `/course/{code}` (metadata + bill-of-materials), `/sets`, `/build`
  (inventory tool with localStorage persistence and reset), `/about`.
- The public `/api/*` passthrough.
- The one-container Fly deployment with `s6-overlay`.

Out of scope (later specs, on the new foundation):

- The 3D course viewer, browse/search/sort/filter, server-side i18n (`/{lang}/`),
  and any other new interactive features.

## Testing

- The Rust API tests stay as they are.
- The frontend uses whatever testing the maintainer's template ships (for
  example vitest and Playwright).
- A short post-deploy smoke check: pages server-render with real content, and
  `/api/*` passes through to the Rust backend.

## Sequencing

Write this spec now. Hold the implementation plan until the maintainer copies
their SvelteKit project into `frontend/`, so the plan matches the template's
actual conventions instead of guessing. The backend changes (strip the page
layer, reuse the axum 0.8 API) can be planned independently of the template.
