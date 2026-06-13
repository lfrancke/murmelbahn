# Frontend rebuild: server-rendered Rust (askama), v1

Date: 2026-06-13

## Problem

The current frontend is a Svelte single-page app served as a static shell by
the axum binary. Every route returns the same empty `index.html` and the content
is rendered client-side, so anything that fetches the raw HTML (crawlers, LLM
agents, link previews) sees no content. The trigger for this work was exactly
that: an agent asked to read the site reported the pages as empty.

The goal of v1 is to make the existing functionality crawlable by rendering real
HTML per route on the server, while keeping the existing JSON/CSV API intact.

## Scope

v1 reproduces what the site does today, as server-rendered pages:

- `/` home and course-code lookup form
- `/course/{code}` course metadata and bill of materials
- `/sets` all known sets and their contents
- `/build` inventory form, returns buildable courses
- `/about` about page

Explicitly out of scope for v1 (each gets its own spec later):

- Browse/search/sort/filter/pagination over courses. This needs a queryable
  course-metadata index that does not exist yet.
- The three.js 3D course viewer. This needs a JS build decision and verification
  of the dump geometry mapping.
- Per-language URLs (`/{lang}/`) and a translation catalog. v1 is English-only.

## Decisions

- **Server-rendered multi-page, not a SPA, not SvelteKit.** The Rust lib is the
  core and stays the core. A second Node service (SvelteKit) would add operational
  weight for no benefit here.
- **Templating: askama.** Compile-time checked templates, no runtime template
  engine, and it is what the design handoff was written against.
- **axum 0.6 to 0.8 migration is a prerequisite, done first as its own step/PR.**
  We are rewriting the router and adding handlers regardless, and the route macro
  syntax (`:id` to `{id}`), the server bootstrap (`axum::Server::bind`), and the
  extractors all change across that gap. Doing the migration first, with the
  current API intact and verified, avoids writing the routing twice and avoids
  tangling a framework major bump with a feature rebuild in an untested crate. It
  also clears the outstanding dependabot alerts.
- **English-only for v1.** The current DE coverage is a handful of section
  titles, so little is lost. Per-language URLs return as a clean phase 2 if wanted.
- **No JS build, no Node, in v1.** Once pages are server-rendered, the only
  client JS is the inventory enhancement, which is dependency-free vanilla JS. It
  is served as a static asset. There is no npm, no vite, no `node_modules`, and
  the `node:16-alpine` Docker stage is removed. The Dockerfile becomes Rust-only.
  A JS build returns only in phase 2 for the three.js viewer, scoped to that
  island, and even then as a minimal island bundler (vite or vendored ESM), not
  a return to the SPA.
- **The `frontend/` Svelte project is deleted.**

## Architecture

```
web/
  src/
    main.rs          axum 0.8 router: page routes + /api/* + /metrics
    pages/           askama-backed page handlers, one per route
      home.rs        GET /                 home + code-lookup form
      course.rs      GET /course/{code}    metadata + BOM table
      sets.rs        GET /sets
      build.rs       GET + POST /build     inventory form + server-rendered results
      about.rs       GET /about
      bom_view.rs    TileKind -> display category/colour mapping for the BOM table
    api/             existing JSON/CSV API moved here (dump/bom/raw, set/list, buildable)
    course_repo.rs   unchanged
  templates/         askama templates: base.html + per-page + partials (header, footer)
  static/            styles.css, inventory.js (served via ServeDir, no build)
```

- `base.html` holds the shared chrome and includes `partials/header.html` and
  `partials/footer.html` via askama `{% include %}`. Each page template extends
  the base.
- The HTML pages and the JSON/CSV API call the same `murmelbahn-lib` entry points.
  The API stays byte-for-byte compatible so API/LLM consumers and the GraviSheet
  CSV paste keep working.
- The starting markup, CSS, and `inventory.js` come from the design handoff in
  `.build-tmp/site/` (treated as inspiration, adapted to real data, not 1:1).

## Per-page data flow

| Route | Handler | Template context |
|---|---|---|
| `GET /` | static plus a GET lookup form | none; form redirects to `/course/{code}` |
| `GET /course/{code}` | `course_repo.get_course_bytes` (auto-downloads uncached codes), `SavedCourse::from_bytes`, `BillOfMaterials::from` | title, code, creation date, save version, BOM grouped for the table, links to the API `dump`/`bom`/`raw` and CSV |
| `GET /sets` | `sets_repo.sets` | list of sets, each with contents in a `<details>` block |
| `GET /build` | `sets_repo.sets` | sets rendered as number inputs |
| `POST /build` | parse form-encoded inventory, call existing `process_all` | list of buildable courses, each linking to `/course/{code}` |
| `GET /about` | static | none |

### BOM table grouping and colours

The handoff groups pieces by category (start, finish, path, action, height, rail,
wall) with colour swatches. `murmelbahn-lib` does not carry categories, so this is
a small display-side `TileKind -> category` map in the web crate (`bom_view.rs`).
It is not added to the lib. The phase-2 viewer reuses the same map.

### Build form

The inventory island is enhancement only. The form renders one
`<input type="number" name="set_<id>">` per set. POST decodes to the existing
`Inventory` type, calls the unchanged `process_all`, and renders the results panel
server-side, so it works fully with no JS. `inventory.js` adds `+/-` steppers,
`localStorage` persistence (key `mb_inventory_v1`), and a background POST to
`/api/buildable` for an inline update.

## Error handling

The current `course.rs` has `impl IntoResponse for Error { todo!() }`, a latent
panic on any course error. v1 replaces this: unknown, unfetchable, or unparseable
codes render a friendly HTML 404/error page through askama, not a panic or a raw
string. Looking up a bad code from the home form lands on that page.

## Testing

The web crate has no tests today. v1 adds:

- askama compile-time template checks (a wrong field name fails the build).
- Template-render unit tests with fixture BOM data and no database: assert the
  course template renders the expected piece rows and the build-results template
  renders course links. These do not touch Postgres.
- A short manual smoke checklist for the database-backed paths (course lookup,
  build), which need a live Postgres and the corpus.

Full end-to-end handler tests would need a test Postgres instance. v1 does not
stand that up.

## Build and deployment impact

- Dockerfile drops the `node:16-alpine` frontend build stage and becomes
  Rust-only. The static assets (`styles.css`, `inventory.js`) ship from
  `web/static/`.
- Deployment stays manual (`flyctl deploy`); there is no CI.

## Sequencing

1. axum 0.6 to 0.8 migration (own PR), existing API verified to still respond.
2. Frontend rebuild on 0.8: askama, page handlers, templates, static assets,
   delete `frontend/`, Rust-only Dockerfile.

Phase 2 specs, later: browse/search (course-metadata index) and the three.js
viewer (island JS build).
