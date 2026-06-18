# SvelteKit Frontend (Step 3) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a fresh SvelteKit app in `frontend/` that server-renders the murmelbahn site (home + lookup, course, sets, build, about) as a thin SSR client over the Rust API, with the hub-style SEO/GEO layer and a frontend CI workflow.

**Architecture:** SvelteKit 2 + Svelte 5 (runes) + `adapter-node`, Tailwind v4 + DaisyUI. Server `load` functions and a `/api/*` passthrough hook call the Rust API at `INTERNAL_API` (default `http://localhost:8080`). No database, no auth, no domain layer: this app only talks to the Rust API. SEO/GEO (`seo.ts`, `SeoHead.svelte`, `jsonld.ts`, `/sitemap.xml`, `/robots.txt`, `/llms.txt`) is ported from `reference/stackable-apps/apps/hub`.

**Tech Stack:** SvelteKit 2.61, Svelte 5.55, adapter-node 5.5, Tailwind v4, DaisyUI 5, TypeScript (strict), Vitest, Playwright, Node 22, npm.

---

## Conventions for the whole plan

- The branch is `sveltekit-frontend` (already checked out). Do not switch branches.
- All `npm`/`git` commands run from the repo root unless a step says `cd frontend`.
- Sandbox: `/tmp` is read-denied. Set `TMPDIR=$PWD/.build-tmp` for any `git` command. For npm, set `TMPDIR=$PWD/.build-tmp` too (npm uses temp dirs).
- No shell heredocs; author files with the editor.
- No em dashes anywhere (code, comments, UI copy, commit messages). Use commas, colons, parentheses, hyphens, or periods. (Both reference projects enforce this.)
- Svelte 5 runes only (`$props`, `$state`, `$derived`, `$effect`), DaisyUI + Tailwind for styling, British or plain English in copy (match existing site: it is English).
- Run `cd frontend && npm run format && npm run lint && npm run check && npm run test:unit -- --run` before each commit; everything lands clean.
- Reference projects on disk (read them, do not import from them): `reference/tasker/web` (house style) and `reference/stackable-apps/apps/hub` (SEO/GEO).
- The Rust API (already API-only on this branch) serves, under `/api`: `GET /api/course/{code}/dump` (full `SavedCourse` JSON), `GET /api/course/{code}/bom` (`BillOfMaterials` JSON), `GET /api/course/{code}/raw`, `GET /api/set/list` (`{ [id]: Set }`), `POST /api/buildable` (`Inventory` JSON in, array of course metadata out). It binds `127.0.0.1:8080`.

## File structure (end state, under `frontend/`)

```
frontend/
  package.json, svelte.config.js, vite.config.ts, tsconfig.json, eslint.config.js, .prettierrc, .npmrc
  src/
    app.html, app.css, app.d.ts
    hooks.server.ts            # /api/* passthrough to the Rust API
    lib/
      seo.ts                   # SEO constants + buildPageTitle
      jsonld.ts                # safe JSON-LD <script> builder
      api.ts                   # typed fetch helpers + response types for the Rust API
      bom.ts                   # group a BillOfMaterials into display sections (pure, tested)
      llms-content.ts          # build the /llms.txt markdown body (pure, tested)
      components/
        SeoHead.svelte         # centralised <svelte:head> meta block
        Nav.svelte             # header/nav
        Footer.svelte
    routes/
      +layout.server.ts        # origin + path for SEO
      +layout.svelte           # shell: SeoHead + Nav + slot + Footer
      +page.svelte             # home + lookup form
      +page.server.ts          # seo + lookup action
      lookup/+server.ts        # GET /lookup?code= -> redirect to /course/{CODE}   (or action in +page.server.ts)
      course/[code]/+page.server.ts   # load dump + bom
      course/[code]/+page.svelte
      sets/+page.server.ts
      sets/+page.svelte
      build/+page.server.ts    # load set list + POST action -> /api/buildable
      build/+page.svelte       # inventory form, localStorage, reset
      about/+page.svelte
      sitemap.xml/+server.ts
      robots.txt/+server.ts
      llms.txt/+server.ts
.github/workflows/frontend-ci.yml
```

The old `frontend/` (the Svelte SPA) is still present on this branch and is replaced wholesale by this scaffold (Task 1 removes it).

---

### Task 1: Scaffold the SvelteKit app

**Files:**
- Delete: the existing `frontend/` contents (old SPA)
- Create: `frontend/` SvelteKit project

- [ ] **Step 1: Remove the old SPA**

```bash
TMPDIR=$PWD/.build-tmp git rm -r frontend
rm -rf frontend
```

- [ ] **Step 2: Scaffold a fresh SvelteKit app**

Use the official scaffolder, non-interactively, into `frontend/`:

```bash
cd /home/lars/dev/privat/gravitrax
TMPDIR=$PWD/.build-tmp npx sv create frontend --template minimal --types ts --no-add-ons --install npm
```

If `sv` prompts despite the flags, answer: minimal template, TypeScript, no add-ons. Then verify it built a project: `ls frontend/src/routes/+page.svelte`.

- [ ] **Step 3: Set the Node adapter and pin Node**

Edit `frontend/svelte.config.js` to match `reference/tasker/web/svelte.config.js`:

```javascript
import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter()
  }
};

export default config;
```

Create `frontend/.npmrc` (supply-chain hygiene, from tasker):

```
engine-strict=true
```

Add `"engines": { "node": "22" }` to `frontend/package.json`.

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm install @sveltejs/adapter-node@^5.5.4
```

- [ ] **Step 4: Add Tailwind v4 + DaisyUI**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm install -D tailwindcss@^4.3.0 @tailwindcss/vite@^4.3.0 daisyui@^5.5.20
```

Set `frontend/vite.config.ts` to load the Tailwind plugin and configure Vitest's two projects, mirroring `reference/tasker/web/vite.config.ts` but WITHOUT the browser project for now (we add Playwright separately and keep unit tests node-only to avoid a browser dependency in CI):

```typescript
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
});
```

- [ ] **Step 5: Lint/format tooling (copy tasker's setup)**

Install the same dev tooling tasker uses:

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm install -D prettier@^3.8.3 prettier-plugin-svelte@^4.0.1 prettier-plugin-tailwindcss@^0.8.0 eslint@^10 @eslint/js typescript-eslint eslint-plugin-svelte eslint-config-prettier globals svelte-check vitest
```

Copy `reference/tasker/web/.prettierrc` to `frontend/.prettierrc` verbatim (it is: useTabs false, singleQuote true, trailingComma none, printWidth 100, plugins prettier-plugin-svelte + prettier-plugin-tailwindcss, svelte override). Copy `reference/tasker/web/eslint.config.js` to `frontend/eslint.config.js` (flat config: @eslint/js + typescript-eslint + eslint-plugin-svelte + prettier; `no-undef` off). Remove any tasker-specific ignores that reference files we do not have.

- [ ] **Step 6: package.json scripts**

Set `frontend/package.json` `"scripts"` to (drop tasker's db/e2e/knip/icon scripts; keep the core ones):

```json
{
  "dev": "vite dev",
  "build": "vite build",
  "preview": "vite preview",
  "prepare": "svelte-kit sync || echo ''",
  "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
  "lint": "prettier --check . && eslint .",
  "format": "prettier --write .",
  "test:unit": "vitest"
}
```

- [ ] **Step 7: Verify the scaffold builds and lints**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check && TMPDIR=$PWD/../.build-tmp npm run build
```
Expected: both succeed (a minimal app builds). If `svelte-check` complains about missing `app.d.ts` types, the scaffolder created them; ensure `src/app.d.ts` exists.

- [ ] **Step 8: Commit**

```bash
TMPDIR=$PWD/.build-tmp git add frontend
TMPDIR=$PWD/.build-tmp git commit -m "Scaffold SvelteKit frontend (adapter-node, Tailwind, DaisyUI)"
```

### Task 2: Global styles and app shell

**Files:**
- Modify: `frontend/src/app.css`, `frontend/src/app.html`
- Create: `frontend/src/lib/components/Nav.svelte`, `frontend/src/lib/components/Footer.svelte`

- [ ] **Step 1: Tailwind + DaisyUI entry stylesheet**

Set `frontend/src/app.css` to load Tailwind v4 and the DaisyUI plugin (Tailwind v4 uses CSS-based config, no `tailwind.config.js`):

```css
@import 'tailwindcss';
@plugin 'daisyui';
```

Ensure `src/routes/+layout.svelte` (created in Task 5) imports it with `import '../app.css';`. (The scaffolder may have put the import in `+layout.svelte` already; if `app.css` does not exist, create it.)

- [ ] **Step 2: app.html language and base**

In `frontend/src/app.html`, set `<html lang="en">` and keep `%sveltekit.head%` / `%sveltekit.body%`. No other change.

- [ ] **Step 3: Nav component**

Create `frontend/src/lib/components/Nav.svelte`:

```svelte
<script lang="ts">
  import { page } from '$app/state';
  const path = $derived(page.url.pathname);
  const links = [
    { href: '/build', label: 'What can I build?' },
    { href: '/sets', label: 'Sets' },
    { href: '/about', label: 'About' }
  ];
</script>

<header class="border-base-300 border-b">
  <nav class="navbar mx-auto max-w-5xl px-4" aria-label="Primary">
    <a href="/" class="text-lg font-semibold">Murmelbahn <span class="text-base-content/60 text-sm font-normal">GraviTrax course index</span></a>
    <ul class="menu menu-horizontal ml-auto gap-1">
      {#each links as link}
        <li><a href={link.href} aria-current={path === link.href ? 'page' : undefined}>{link.label}</a></li>
      {/each}
    </ul>
  </nav>
</header>
```

- [ ] **Step 4: Footer component**

Create `frontend/src/lib/components/Footer.svelte`:

```svelte
<footer class="border-base-300 text-base-content/60 mt-16 border-t">
  <div class="mx-auto flex max-w-5xl flex-wrap items-center gap-4 px-4 py-6 text-sm">
    <span>Murmelbahn, an unofficial GraviTrax course index.</span>
    <a class="link" href="https://github.com/lfrancke/murmelbahn" rel="noopener">GitHub</a>
    <a class="link" href="/about">About</a>
    <span class="ml-auto">Not affiliated with Ravensburger.</span>
  </div>
</footer>
```

- [ ] **Step 5: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check
TMPDIR=$PWD/.build-tmp git add frontend/src
TMPDIR=$PWD/.build-tmp git commit -m "Add Tailwind/DaisyUI entry stylesheet and app shell components"
```

### Task 3: API client and types

**Files:**
- Create: `frontend/src/lib/api.ts`

- [ ] **Step 1: Create the typed API client**

`INTERNAL_API` is the Rust API base; in dev and prod it is `http://localhost:8080` (Rust binds there). Use `$env/dynamic/private` so it is server-only and overridable.

```typescript
import { env } from '$env/dynamic/private';

const BASE = env.INTERNAL_API ?? 'http://localhost:8080';

export interface Name {
  language_code: string;
  name: string;
}
export interface SetInfo {
  id: string;
  names: Name[];
  content: Record<string, number>;
}
export type SetList = Record<string, SetInfo>;

export interface Bom {
  layers: Record<string, number>;
  tiles: Record<string, number>;
  rails: Record<string, number>;
  walls: Record<string, number>;
  balconies: number;
  rails_small: number;
  rails_medium: number;
  rails_large: number;
  connectors: number;
}

export interface SavedCourse {
  header: { version: string };
  course: { meta_data: { title: string; creation_timestamp: number } };
}

export interface BuildableCourse {
  course_code: string;
  title: string;
  date_added_to_db: string;
  creation_timestamp: string;
}

export interface Inventory {
  sets: Record<string, number>;
  extra_elements: Record<string, number>;
}

async function getJson<T>(path: string, fetcher: typeof fetch): Promise<T | null> {
  const res = await fetcher(`${BASE}${path}`);
  if (res.status === 404) return null;
  if (!res.ok) throw new Error(`API ${path} returned ${res.status}`);
  return (await res.json()) as T;
}

export function fetchSavedCourse(code: string, fetcher: typeof fetch) {
  return getJson<SavedCourse>(`/api/course/${encodeURIComponent(code)}/dump`, fetcher);
}

export function fetchBom(code: string, fetcher: typeof fetch) {
  return getJson<Bom>(`/api/course/${encodeURIComponent(code)}/bom`, fetcher);
}

export async function fetchSets(fetcher: typeof fetch): Promise<SetList> {
  return (await getJson<SetList>('/api/set/list', fetcher)) ?? {};
}

export async function fetchBuildable(inventory: Inventory, fetcher: typeof fetch): Promise<BuildableCourse[]> {
  const res = await fetcher(`${BASE}/api/buildable`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(inventory)
  });
  if (!res.ok) throw new Error(`buildable returned ${res.status}`);
  return (await res.json()) as BuildableCourse[];
}

export function englishName(set: SetInfo): string {
  return set.names.find((n) => n.language_code === 'en')?.name ?? set.names[0]?.name ?? set.id;
}
```

(Use the passed `fetch` from load functions so SvelteKit can track/inline requests.)

- [ ] **Step 2: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check
TMPDIR=$PWD/.build-tmp git add frontend/src/lib/api.ts
TMPDIR=$PWD/.build-tmp git commit -m "Add typed API client for the Rust backend"
```

### Task 4: `/api/*` passthrough hook

**Files:**
- Create: `frontend/src/hooks.server.ts`

- [ ] **Step 1: Create the proxy hook**

External consumers hit the public domain `/api/*`; forward those to the Rust API so the documented API stays at the same URLs.

```typescript
import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BASE = env.INTERNAL_API ?? 'http://localhost:8080';

export const handle: Handle = async ({ event, resolve }) => {
  if (event.url.pathname.startsWith('/api/')) {
    const target = `${BASE}${event.url.pathname}${event.url.search}`;
    const method = event.request.method;
    const body = method === 'GET' || method === 'HEAD' ? undefined : await event.request.arrayBuffer();
    const upstream = await fetch(target, {
      method,
      headers: { 'content-type': event.request.headers.get('content-type') ?? 'application/json' },
      body
    });
    return new Response(upstream.body, {
      status: upstream.status,
      headers: { 'content-type': upstream.headers.get('content-type') ?? 'application/octet-stream' }
    });
  }
  return resolve(event);
};
```

- [ ] **Step 2: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check
TMPDIR=$PWD/.build-tmp git add frontend/src/hooks.server.ts
TMPDIR=$PWD/.build-tmp git commit -m "Proxy /api/* to the Rust backend via a server hook"
```

### Task 5: SEO foundation (seo.ts, jsonld.ts, SeoHead, layout)

**Files:**
- Create: `frontend/src/lib/seo.ts`, `frontend/src/lib/jsonld.ts`, `frontend/src/lib/components/SeoHead.svelte`, `frontend/src/routes/+layout.server.ts`
- Modify/Create: `frontend/src/routes/+layout.svelte`

- [ ] **Step 1: seo.ts**

Adapt `reference/stackable-apps/apps/hub/src/lib/seo.ts` for murmelbahn:

```typescript
export const SITE_NAME = 'Murmelbahn';
export const DEFAULT_TITLE = 'Murmelbahn, a GraviTrax course index';
export const DEFAULT_DESCRIPTION =
  'Look up any GraviTrax course by its code to see the full bill of materials, or enter the sets you own to find every track you can build.';
export const PUBLISHER = {
  name: 'Lars Francke',
  url: 'https://github.com/lfrancke'
};

export function buildPageTitle(pageTitle: string | undefined): string {
  if (!pageTitle || pageTitle === SITE_NAME) return SITE_NAME;
  return `${pageTitle} · ${SITE_NAME}`;
}
```

- [ ] **Step 2: jsonld.ts**

Copy `reference/stackable-apps/apps/hub/src/lib/jsonld.ts` verbatim (it is generic: `escapeForScript` + `jsonLdScript`). No changes needed.

- [ ] **Step 3: SeoHead.svelte**

Copy `reference/stackable-apps/apps/hub/src/lib/components/SeoHead.svelte` verbatim. It reads `page.data.seo`, `page.data.origin`, `page.data.path` and renders title/description/canonical/OG/Twitter/robots. No changes needed.

- [ ] **Step 4: +layout.server.ts**

Copy `reference/stackable-apps/apps/hub/src/routes/+layout.server.ts` verbatim:

```typescript
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = ({ url }) => {
  return { origin: url.origin, path: url.pathname };
};
```

- [ ] **Step 5: +layout.svelte renders the shell**

```svelte
<script lang="ts">
  import '../app.css';
  import SeoHead from '$lib/components/SeoHead.svelte';
  import Nav from '$lib/components/Nav.svelte';
  import Footer from '$lib/components/Footer.svelte';
  let { children } = $props();
</script>

<SeoHead />
<Nav />
<main class="mx-auto max-w-5xl px-4 py-8">
  {@render children()}
</main>
<Footer />
```

- [ ] **Step 6: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check
TMPDIR=$PWD/.build-tmp git add frontend/src
TMPDIR=$PWD/.build-tmp git commit -m "Add SEO foundation (seo, jsonld, SeoHead, layout origin/path)"
```

### Task 6: BOM grouping helper (TDD)

**Files:**
- Create: `frontend/src/lib/bom.ts`, `frontend/src/lib/bom.spec.ts`

- [ ] **Step 1: Write the failing test**

Create `frontend/src/lib/bom.spec.ts`:

```typescript
import { describe, it, expect } from 'vitest';
import { bomSections } from './bom';
import type { Bom } from './api';

const empty: Bom = {
  layers: {}, tiles: {}, rails: {}, walls: {},
  balconies: 0, rails_small: 0, rails_medium: 0, rails_large: 0, connectors: 0
};

describe('bomSections', () => {
  it('groups tiles sorted by count descending then label', () => {
    const bom: Bom = { ...empty, tiles: { Cannon: 2, Starter: 5 }, connectors: 3 };
    const sections = bomSections(bom);
    const tiles = sections.find((s) => s.title === 'Tiles');
    expect(tiles).toBeDefined();
    expect(tiles!.rows[0]).toEqual({ label: 'Starter', count: 5 });
    expect(tiles!.rows[1]).toEqual({ label: 'Cannon', count: 2 });
    expect(sections.find((s) => s.title === 'Connectors')).toBeDefined();
  });

  it('omits empty groups', () => {
    const sections = bomSections(empty);
    expect(sections).toHaveLength(0);
  });
});
```

- [ ] **Step 2: Run it, expect failure**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run test:unit -- --run bom
```
Expected: FAIL (cannot find `bomSections`).

- [ ] **Step 3: Implement `bom.ts`**

```typescript
import type { Bom } from './api';

export interface BomRow {
  label: string;
  count: number;
}
export interface BomSection {
  title: string;
  rows: BomRow[];
}

function rows(map: Record<string, number>): BomRow[] {
  return Object.entries(map)
    .filter(([, count]) => count > 0)
    .map(([label, count]) => ({ label, count }))
    .sort((a, b) => b.count - a.count || a.label.localeCompare(b.label));
}

export function bomSections(bom: Bom): BomSection[] {
  const sections: BomSection[] = [];
  for (const [title, map] of [
    ['Tiles', bom.tiles],
    ['Rails', bom.rails],
    ['Walls', bom.walls],
    ['Layers', bom.layers]
  ] as const) {
    const r = rows(map);
    if (r.length) sections.push({ title, rows: r });
  }
  if (bom.connectors > 0) {
    sections.push({ title: 'Connectors', rows: [{ label: 'Connector', count: bom.connectors }] });
  }
  return sections;
}
```

- [ ] **Step 4: Run it, expect pass; commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run test:unit -- --run bom
TMPDIR=$PWD/.build-tmp git add frontend/src/lib/bom.ts frontend/src/lib/bom.spec.ts
TMPDIR=$PWD/.build-tmp git commit -m "Add BOM grouping helper for the course page"
```

### Task 7: Home page + lookup

**Files:**
- Create: `frontend/src/routes/+page.server.ts`, `frontend/src/routes/+page.svelte`, `frontend/src/routes/lookup/+server.ts`

- [ ] **Step 1: Home seo (+page.server.ts)**

```typescript
import type { PageServerLoad } from './$types';
import { DEFAULT_DESCRIPTION } from '$lib/seo';

export const load: PageServerLoad = () => {
  return { seo: { title: undefined, description: DEFAULT_DESCRIPTION } };
};
```

- [ ] **Step 2: Lookup endpoint (lookup/+server.ts)**

Sanitise the code (alphanumeric, uppercase) and redirect; reject empties.

```typescript
import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = ({ url }) => {
  const raw = url.searchParams.get('code') ?? '';
  const code = raw.replace(/[^a-zA-Z0-9]/g, '').toUpperCase();
  if (!code) throw redirect(303, '/');
  throw redirect(303, `/course/${code}`);
};
```

- [ ] **Step 3: Home markup (+page.svelte)**

Port the hero + lookup form + tiles from `.build-tmp/site/index.html` into DaisyUI/Tailwind. The form is a GET to `/lookup`. Include WebSite + Organization JSON-LD:

```svelte
<script lang="ts">
  import { jsonLdScript } from '$lib/jsonld';
  import { SITE_NAME, DEFAULT_DESCRIPTION, PUBLISHER } from '$lib/seo';
  import { page } from '$app/state';
  const origin = $derived(page.data.origin as string | undefined);
  const websiteJsonLd = $derived({
    '@context': 'https://schema.org',
    '@type': 'WebSite',
    name: SITE_NAME,
    description: DEFAULT_DESCRIPTION,
    url: origin
  });
  const orgJsonLd = $derived({
    '@context': 'https://schema.org',
    '@type': 'Organization',
    name: PUBLISHER.name,
    url: PUBLISHER.url
  });
</script>

<svelte:head>
  {@html jsonLdScript(websiteJsonLd)}
  {@html jsonLdScript(orgJsonLd)}
</svelte:head>

<section class="prose max-w-none">
  <p class="text-base-content/60 text-sm tracking-wide uppercase">GraviTrax course database</p>
  <h1>Look up any course, and see what you can build.</h1>
  <p class="lead">Enter a course code from the GraviTrax app to inspect its construction and full bill of materials, or tell us which sets you own and find every track you can build.</p>
</section>

<form action="/lookup" method="get" role="search" class="join mt-6">
  <input name="code" placeholder="COURSE CODE" maxlength="12" autocapitalize="characters" autocomplete="off" aria-label="Course code" class="input input-bordered join-item" />
  <button type="submit" class="btn btn-primary join-item">Look up</button>
</form>

<div class="mt-10 grid gap-4 sm:grid-cols-2">
  <a href="/build" class="card bg-base-200 p-5">
    <h2 class="text-lg font-semibold">What can I build? &rarr;</h2>
    <p class="text-base-content/70 text-sm">Enter the sets you own; see which tracks are buildable.</p>
  </a>
  <a href="/sets" class="card bg-base-200 p-5">
    <h2 class="text-lg font-semibold">Sets &rarr;</h2>
    <p class="text-base-content/70 text-sm">Every set and the pieces it contains.</p>
  </a>
</div>
```

- [ ] **Step 4: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check && TMPDIR=$PWD/../.build-tmp npm run build
TMPDIR=$PWD/.build-tmp git add frontend/src/routes
TMPDIR=$PWD/.build-tmp git commit -m "Add home page and course-code lookup"
```

### Task 8: Course page

**Files:**
- Create: `frontend/src/routes/course/[code]/+page.server.ts`, `frontend/src/routes/course/[code]/+page.svelte`

- [ ] **Step 1: Load dump + bom (+page.server.ts)**

The dump gives title/version/created; the bom gives the piece counts. A missing course is a 404.

```typescript
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { fetchSavedCourse, fetchBom } from '$lib/api';
import { bomSections } from '$lib/bom';

export const load: PageServerLoad = async ({ params, fetch }) => {
  const code = params.code.toUpperCase();
  const [saved, bom] = await Promise.all([fetchSavedCourse(code, fetch), fetchBom(code, fetch)]);
  if (!saved || !bom) throw error(404, `Course ${code} could not be found.`);

  const created = new Date(saved.course.meta_data.creation_timestamp).toISOString().slice(0, 10);
  const title = saved.course.meta_data.title;
  const sections = bomSections(bom);

  return {
    code,
    title,
    version: saved.header.version,
    created,
    sections,
    seo: { title, description: `Bill of materials for the GraviTrax course ${title} (${code}).` }
  };
};
```

- [ ] **Step 2: Course markup + JSON-LD (+page.svelte)**

```svelte
<script lang="ts">
  import { jsonLdScript } from '$lib/jsonld';
  import { page } from '$app/state';
  let { data } = $props();
  const jsonLd = $derived({
    '@context': 'https://schema.org',
    '@type': 'CreativeWork',
    name: data.title,
    identifier: data.code,
    url: `${page.data.origin}/course/${data.code}`
  });
</script>

<svelte:head>{@html jsonLdScript(jsonLd)}</svelte:head>

<article>
  <p class="text-base-content/60 font-mono text-sm">Course {data.code}</p>
  <h1 class="text-2xl font-bold">{data.title}</h1>
  <p class="text-base-content/70 mt-1 text-sm">Format: {data.version} · Created: {data.created}</p>
  <p class="mt-3 flex flex-wrap gap-4 text-sm">
    <a class="link" href={`https://gravitrax.link.ravensburger.com/code/${data.code}`} rel="noopener">Open in the GraviTrax app</a>
    <a class="link" href={`/api/course/${data.code}/bom?format=csv`}>CSV</a>
    <a class="link" href={`/api/course/${data.code}/bom`}>BOM JSON</a>
    <a class="link" href={`/api/course/${data.code}/dump`}>Full dump</a>
    <a class="link" href={`/api/course/${data.code}/raw`}>Raw bytes</a>
  </p>

  <h2 class="mt-8 text-xl font-semibold">Bill of materials</h2>
  {#if data.sections.length === 0}
    <p>No pieces were counted for this course.</p>
  {:else}
    {#each data.sections as section}
      <h3 class="mt-5 font-semibold">{section.title}</h3>
      <table class="table-zebra table w-full max-w-md">
        <thead><tr><th>Piece</th><th>Count</th></tr></thead>
        <tbody>
          {#each section.rows as row}
            <tr><td>{row.label}</td><td>{row.count}</td></tr>
          {/each}
        </tbody>
      </table>
    {/each}
  {/if}
</article>
```

- [ ] **Step 3: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check && TMPDIR=$PWD/../.build-tmp npm run build
TMPDIR=$PWD/.build-tmp git add frontend/src/routes/course
TMPDIR=$PWD/.build-tmp git commit -m "Add server-rendered course page with BOM and JSON-LD"
```

### Task 9: Sets page

**Files:**
- Create: `frontend/src/routes/sets/+page.server.ts`, `frontend/src/routes/sets/+page.svelte`

- [ ] **Step 1: Load + shape sets (+page.server.ts)**

```typescript
import type { PageServerLoad } from './$types';
import { fetchSets, englishName } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const raw = await fetchSets(fetch);
  const sets = Object.values(raw)
    .map((s) => ({
      id: s.id,
      name: englishName(s),
      pieces: Object.entries(s.content)
        .map(([label, count]) => ({ label, count }))
        .sort((a, b) => b.count - a.count || a.label.localeCompare(b.label))
    }))
    .sort((a, b) => a.name.localeCompare(b.name));
  return { sets, seo: { title: 'Sets', description: 'Every GraviTrax set known to the database and the pieces it contains.' } };
};
```

- [ ] **Step 2: Sets markup (+page.svelte)** (DaisyUI `collapse` for the no-JS details)

```svelte
<script lang="ts">
  let { data } = $props();
</script>

<h1 class="text-2xl font-bold">Sets</h1>
<p class="text-base-content/70 mt-1">Every set known to the database and the pieces it contains.</p>

{#each data.sets as set}
  <details class="collapse-arrow bg-base-200 collapse mt-2">
    <summary class="collapse-title font-medium">{set.name} <span class="text-base-content/50 font-mono text-sm">({set.id})</span></summary>
    <div class="collapse-content">
      <table class="table w-full max-w-md">
        <thead><tr><th>Piece</th><th>Count</th></tr></thead>
        <tbody>
          {#each set.pieces as piece}
            <tr><td>{piece.label}</td><td>{piece.count}</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </details>
{/each}
```

- [ ] **Step 3: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check && TMPDIR=$PWD/../.build-tmp npm run build
TMPDIR=$PWD/.build-tmp git add frontend/src/routes/sets
TMPDIR=$PWD/.build-tmp git commit -m "Add sets page"
```

### Task 10: Build (inventory) page

**Files:**
- Create: `frontend/src/routes/build/+page.server.ts`, `frontend/src/routes/build/+page.svelte`

- [ ] **Step 1: Load sets + POST action (+page.server.ts)**

The no-JS path posts the form to the default action, which builds an `Inventory` and calls the Rust API. Set fields are named by set id.

```typescript
import type { PageServerLoad, Actions } from './$types';
import { fetchSets, fetchBuildable, englishName, type Inventory } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const raw = await fetchSets(fetch);
  const sets = Object.values(raw)
    .map((s) => ({ id: s.id, name: englishName(s) }))
    .sort((a, b) => a.name.localeCompare(b.name));
  return { sets, seo: { title: 'What can I build?', description: 'Enter the GraviTrax sets you own and find every track in the database you can build.' } };
};

export const actions: Actions = {
  default: async ({ request, fetch }) => {
    const form = await request.formData();
    const sets: Record<string, number> = {};
    for (const [key, value] of form.entries()) {
      const n = parseInt(String(value), 10);
      if (Number.isFinite(n) && n > 0) sets[key] = n;
    }
    const inventory: Inventory = { sets, extra_elements: {} };
    const courses = await fetchBuildable(inventory, fetch);
    return { courses };
  }
};
```

- [ ] **Step 2: Build markup with progressive enhancement (+page.svelte)**

Use `use:enhance` so the submit is an inline fetch (no full reload); `localStorage` persists the inventory; a reset clears it. With no JS the form posts normally and the action result renders.

```svelte
<script lang="ts">
  import { enhance } from '$app/forms';
  let { data, form } = $props();
  const STORAGE_KEY = 'mb_inventory';

  let formEl: HTMLFormElement;

  function restore() {
    if (typeof localStorage === 'undefined') return;
    let saved: Record<string, number> = {};
    try {
      saved = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '{}');
    } catch {
      saved = {};
    }
    for (const input of formEl.querySelectorAll<HTMLInputElement>('input[type="number"]')) {
      if (saved[input.name] != null) input.value = String(saved[input.name]);
    }
  }

  function save() {
    if (typeof localStorage === 'undefined') return;
    const sets: Record<string, number> = {};
    for (const input of formEl.querySelectorAll<HTMLInputElement>('input[type="number"]')) {
      const n = parseInt(input.value, 10);
      if (Number.isFinite(n) && n > 0) sets[input.name] = n;
    }
    localStorage.setItem(STORAGE_KEY, JSON.stringify(sets));
  }

  $effect(() => {
    restore();
  });
</script>

<h1 class="text-2xl font-bold">What can I build?</h1>
<p class="text-base-content/70 mt-1">Enter how many of each set you own, then submit. Tracks you can build are listed below.</p>

<form bind:this={formEl} method="post" use:enhance oninput={save} onreset={() => localStorage.removeItem(STORAGE_KEY)} class="mt-6">
  <div class="grid gap-x-8 gap-y-1 sm:grid-cols-2">
    {#each data.sets as set}
      <label class="flex items-center gap-3 py-1">
        <span class="flex-1 text-sm">{set.name}</span>
        <input type="number" name={set.id} min="0" value="0" inputmode="numeric" class="input input-bordered input-sm w-20 font-mono" />
      </label>
    {/each}
  </div>
  <div class="mt-4 flex gap-2">
    <button type="submit" class="btn btn-primary">Show buildable courses</button>
    <button type="reset" class="btn btn-ghost">Reset</button>
  </div>
</form>

{#if form?.courses}
  <section class="mt-8">
    <h2 class="text-xl font-semibold">Buildable courses ({form.courses.length})</h2>
    {#if form.courses.length === 0}
      <p>No courses in the database can be built with that inventory yet.</p>
    {:else}
      <ul class="mt-2 columns-1 sm:columns-2">
        {#each form.courses as course}
          <li class="py-1"><a class="link" href={`/course/${course.course_code}`}>{course.title}</a> <span class="text-base-content/50 font-mono text-xs">{course.course_code}</span></li>
        {/each}
      </ul>
    {/if}
  </section>
{/if}
```

- [ ] **Step 3: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check && TMPDIR=$PWD/../.build-tmp npm run build
TMPDIR=$PWD/.build-tmp git add frontend/src/routes/build
TMPDIR=$PWD/.build-tmp git commit -m "Add build page with inventory form, persistence, and buildable results"
```

### Task 11: About page

**Files:**
- Create: `frontend/src/routes/about/+page.server.ts`, `frontend/src/routes/about/+page.svelte`

- [ ] **Step 1: about seo (+page.server.ts)**

```typescript
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = () => {
  return { seo: { title: 'About', description: 'About the Murmelbahn GraviTrax course index.' } };
};
```

- [ ] **Step 2: about markup (+page.svelte)**

```svelte
<h1 class="text-2xl font-bold">About this site</h1>
<ul class="mt-3 list-disc space-y-2 pl-6">
  <li>Built by <a class="link" href="https://github.com/lfrancke" rel="noopener">Lars Francke</a>. It is in no way affiliated with Ravensburger. Please do not contact them about this site.</li>
  <li>It is <a class="link" href="https://github.com/lfrancke/murmelbahn" rel="noopener">open source</a>. Found a bug or want a feature? Open an issue on GitHub.</li>
  <li>See the <a class="link" href="https://github.com/lfrancke/murmelbahn/blob/main/CHANGELOG.md" rel="noopener">Changelog</a> for what is new.</li>
  <li>Also see the <a class="link" href="https://docs.google.com/spreadsheets/d/1T-hLIBz05q4QMlt7xQ63Y1SrnG_3HYWchF_nLXvYkJg/template/preview" rel="noopener">GraviSheet</a>, a spreadsheet version of this data.</li>
  <li>Contact: <span class="font-mono">murmelbahn (at) lars-francke.de</span></li>
</ul>
```

- [ ] **Step 3: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run check
TMPDIR=$PWD/.build-tmp git add frontend/src/routes/about
TMPDIR=$PWD/.build-tmp git commit -m "Add about page"
```

### Task 12: robots.txt, llms.txt, sitemap.xml

**Files:**
- Create: `frontend/src/routes/robots.txt/+server.ts`, `frontend/src/lib/llms-content.ts`, `frontend/src/lib/llms-content.spec.ts`, `frontend/src/routes/llms.txt/+server.ts`, `frontend/src/routes/sitemap.xml/+server.ts`

- [ ] **Step 1: robots.txt (+server.ts)**

Adapt `reference/stackable-apps/apps/hub/src/routes/robots.txt/+server.ts`: keep the `AI_USER_AGENTS` list and the structure, change the comment header to murmelbahn:

```typescript
import type { RequestHandler } from './$types';

const CACHE_HEADERS = {
  'Content-Type': 'text/plain; charset=utf-8',
  'Cache-Control': 'public, max-age=3600'
};

const AI_USER_AGENTS = [
  'GPTBot', 'ChatGPT-User', 'OAI-SearchBot', 'ClaudeBot', 'Claude-Web', 'anthropic-ai',
  'PerplexityBot', 'Perplexity-User', 'Google-Extended', 'Applebot-Extended', 'CCBot', 'cohere-ai', 'Bytespider'
];

export const GET: RequestHandler = ({ url }) => {
  const aiBlocks = AI_USER_AGENTS.map((ua) => `User-agent: ${ua}\nAllow: /`).join('\n\n');
  const body = `# Murmelbahn, an unofficial GraviTrax course index.
# Public site, AI and LLM crawlers are welcome.
# See also: ${url.origin}/llms.txt and ${url.origin}/sitemap.xml

User-agent: *
Allow: /

${aiBlocks}

Sitemap: ${url.origin}/sitemap.xml
`;
  return new Response(body, { headers: CACHE_HEADERS });
};
```

- [ ] **Step 2: llms-content.ts (TDD) - write the failing test**

Create `frontend/src/lib/llms-content.spec.ts`:

```typescript
import { describe, it, expect } from 'vitest';
import { buildLlmsTxt } from './llms-content';

describe('buildLlmsTxt', () => {
  it('includes the site name, the key pages, and the API base', () => {
    const txt = buildLlmsTxt('https://example.test');
    expect(txt).toContain('# Murmelbahn');
    expect(txt).toContain('https://example.test/build');
    expect(txt).toContain('https://example.test/api/course/');
  });
});
```

- [ ] **Step 3: Run it, expect failure**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run test:unit -- --run llms
```
Expected: FAIL (cannot find `buildLlmsTxt`).

- [ ] **Step 4: Implement llms-content.ts**

```typescript
import { SITE_NAME, DEFAULT_DESCRIPTION } from './seo';

/// Markdown summary served at /llms.txt (llmstxt.org convention) so agents can
/// understand the site and use the JSON/CSV API.
export function buildLlmsTxt(origin: string): string {
  return `# ${SITE_NAME}

> ${DEFAULT_DESCRIPTION}

## Pages

- [Home and course lookup](${origin}/)
- [What can I build?](${origin}/build): enter the sets you own to find buildable tracks.
- [Sets](${origin}/sets): every set and its pieces.
- [About](${origin}/about)

## API

A course is identified by its app code. For a code CODE:

- ${origin}/api/course/CODE/bom : bill of materials as JSON.
- ${origin}/api/course/CODE/bom?format=csv : bill of materials as GraviSheet CSV.
- ${origin}/api/course/CODE/dump : the full parsed course as JSON.
- ${origin}/api/set/list : every known set and its contents.
`;
}
```

- [ ] **Step 5: Run the test (pass), then the llms.txt endpoint**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run test:unit -- --run llms
```
Create `frontend/src/routes/llms.txt/+server.ts`:

```typescript
import type { RequestHandler } from './$types';
import { buildLlmsTxt } from '$lib/llms-content';

const CACHE_HEADERS = {
  'Content-Type': 'text/plain; charset=utf-8',
  'Cache-Control': 'public, max-age=3600'
};

export const GET: RequestHandler = ({ url }) => {
  return new Response(buildLlmsTxt(url.origin), { headers: CACHE_HEADERS });
};
```

- [ ] **Step 6: sitemap.xml (+server.ts)**

For v1 the sitemap lists the static pages only (course pages are dynamic and numerous; listing them needs a course-index endpoint that does not exist yet, so they are intentionally omitted, see the spec). Adapt the hub structure:

```typescript
import type { RequestHandler } from './$types';

const CACHE_HEADERS = {
  'Content-Type': 'application/xml; charset=utf-8',
  'Cache-Control': 'public, max-age=3600'
};

const PATHS = ['/', '/build', '/sets', '/about'];

export const GET: RequestHandler = ({ url }) => {
  const entries = PATHS.map(
    (p) => `  <url>\n    <loc>${url.origin}${p}</loc>\n    <changefreq>weekly</changefreq>\n  </url>`
  ).join('\n');
  const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${entries}
</urlset>
`;
  return new Response(body, { headers: CACHE_HEADERS });
};
```

- [ ] **Step 7: Build + commit**

```bash
cd frontend && TMPDIR=$PWD/../.build-tmp npm run test:unit -- --run && TMPDIR=$PWD/../.build-tmp npm run build
TMPDIR=$PWD/.build-tmp git add frontend/src
TMPDIR=$PWD/.build-tmp git commit -m "Add robots.txt, llms.txt, and sitemap.xml"
```

### Task 13: Frontend CI workflow

**Files:**
- Create: `.github/workflows/frontend-ci.yml`

- [ ] **Step 1: Create the workflow**

```yaml
name: Frontend CI

on:
  push:
    branches: [main]
    paths:
      - 'frontend/**'
      - '.github/workflows/frontend-ci.yml'
  pull_request:
    paths:
      - 'frontend/**'
      - '.github/workflows/frontend-ci.yml'

concurrency:
  group: frontend-ci-${{ github.ref }}
  cancel-in-progress: true

jobs:
  check:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: frontend
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: npm
          cache-dependency-path: frontend/package-lock.json
      - run: npm ci
      - run: npm run lint
      - run: npm run check
      - run: npm run test:unit -- --run
      - run: npm run build
```

- [ ] **Step 2: Commit**

```bash
TMPDIR=$PWD/.build-tmp git add .github/workflows/frontend-ci.yml
TMPDIR=$PWD/.build-tmp git commit -m "Add frontend CI workflow"
```

### Task 14: Full local verification

- [ ] **Step 1: Lint, check, unit tests, build all green**

```bash
cd frontend
TMPDIR=$PWD/../.build-tmp npm run format
TMPDIR=$PWD/../.build-tmp npm run lint
TMPDIR=$PWD/../.build-tmp npm run check
TMPDIR=$PWD/../.build-tmp npm run test:unit -- --run
TMPDIR=$PWD/../.build-tmp npm run build
```
All must pass.

- [ ] **Step 2: Manual dev smoke (optional, needs the Rust API + DB)**

This needs the Rust API running on `:8080` against the proxied production DB, which is a separate decision (the controller runs read-only smoke tests). If approved: `cargo run -p murmelbahn-web` in one shell (binds `127.0.0.1:8080`), `npm run dev` in `frontend/` in another, then load `/`, `/sets`, `/about`, `/build`, `/course/BRXU8VN4QW` (a cached code), `/robots.txt`, `/llms.txt`, `/sitemap.xml`, and confirm SSR HTML (view source shows content) and that `/api/course/BRXU8VN4QW/bom` passes through. Use only known-cached codes so no write path runs.

- [ ] **Step 3: Commit any format-only changes**

```bash
TMPDIR=$PWD/.build-tmp git add -A frontend
TMPDIR=$PWD/.build-tmp git commit -m "Apply formatting" || echo "nothing to commit"
```

---

## Out of scope (later)

- The one-container Dockerfile + s6-overlay + fly.toml + deploy: that is step 4, a separate plan.
- The 3D viewer, browse/search, server-side i18n, a Playwright e2e suite, and a `/llms` HTML preview page: later specs.
- Listing course pages in the sitemap (needs a course-index API endpoint that does not exist yet).
- Showing the approximate marble count on the course page (it is a Rust-only computation in `BillOfMaterials::marbles()`; reimplementing it in TS would duplicate logic, so it is omitted unless a small API field is added later).
