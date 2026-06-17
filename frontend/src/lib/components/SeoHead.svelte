<!--
  Centralised <svelte:head> for SEO/GEO. Renders title, description, canonical,
  Open Graph and Twitter Card meta tags. Per-page values come from the page's
  `seo` data; absolute URLs come from the layout's `origin` + `path` data.
-->
<script lang="ts">
  import { page } from '$app/state';
  import { SITE_NAME, DEFAULT_TITLE, DEFAULT_DESCRIPTION, buildPageTitle } from '$lib/seo';

  const pageSeo = $derived(page.data.seo as { title?: string; description?: string } | undefined);
  const origin = $derived((page.data.origin as string | undefined) ?? '');
  const path = $derived((page.data.path as string | undefined) ?? '/');

  const title = $derived(buildPageTitle(pageSeo?.title ?? DEFAULT_TITLE));
  const description = $derived(pageSeo?.description ?? DEFAULT_DESCRIPTION);
  const canonical = $derived(origin ? `${origin}${path}` : path);
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <link rel="canonical" href={canonical} />

  <meta property="og:type" content="website" />
  <meta property="og:site_name" content={SITE_NAME} />
  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={canonical} />
  <meta property="og:locale" content="en" />

  <meta name="twitter:card" content="summary" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />

  <!-- Generous robots directives, the hub is intentionally public. -->
  <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1" />
</svelte:head>
