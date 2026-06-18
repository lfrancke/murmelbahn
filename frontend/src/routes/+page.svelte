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
  <!-- eslint-disable-next-line svelte/no-at-html-tags -- jsonLdScript sanitizes its input -->
  {@html jsonLdScript(websiteJsonLd)}
  <!-- eslint-disable-next-line svelte/no-at-html-tags -- jsonLdScript sanitizes its input -->
  {@html jsonLdScript(orgJsonLd)}
</svelte:head>

<section class="prose max-w-none">
  <p class="text-base-content/60 text-sm tracking-wide uppercase">GraviTrax course database</p>
  <h1>Look up any course, and see what you can build.</h1>
  <p class="lead">
    Enter a course code from the GraviTrax app to inspect its construction and full bill of
    materials, or tell us which sets you own and find every track you can build.
  </p>
</section>

<form action="/lookup" method="get" role="search" class="join mt-6">
  <input
    name="code"
    placeholder="COURSE CODE"
    maxlength="12"
    autocapitalize="characters"
    autocomplete="off"
    aria-label="Course code"
    class="input input-bordered join-item"
  />
  <button type="submit" class="btn btn-primary join-item">Look up</button>
</form>

<div class="mt-10 grid gap-4 sm:grid-cols-2">
  <a href="/build" class="card bg-base-200 p-5">
    <h2 class="text-lg font-semibold">What can I build? &rarr;</h2>
    <p class="text-base-content/70 text-sm">
      Enter the sets you own; see which tracks are buildable.
    </p>
  </a>
  <a href="/sets" class="card bg-base-200 p-5">
    <h2 class="text-lg font-semibold">Sets &rarr;</h2>
    <p class="text-base-content/70 text-sm">Every set and the pieces it contains.</p>
  </a>
</div>
