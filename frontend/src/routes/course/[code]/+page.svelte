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

<!-- eslint-disable-next-line svelte/no-at-html-tags -- jsonLdScript sanitizes its input -->
<svelte:head>{@html jsonLdScript(jsonLd)}</svelte:head>

<article>
  <p class="text-base-content/60 font-mono text-sm">Course {data.code}</p>
  <h1 class="text-2xl font-bold">{data.title}</h1>
  <p class="text-base-content/70 mt-1 text-sm">
    Format: {data.version}{#if data.created} · Created: {data.created}{/if}
  </p>
  <p class="mt-3 flex flex-wrap gap-4 text-sm">
    <a
      class="link"
      href={`https://gravitrax.link.ravensburger.com/code/${data.code}`}
      rel="noopener">Open in the GraviTrax app</a
    >
    <a class="link" href={`/api/course/${data.code}/bom?format=csv`}>CSV</a>
    <a class="link" href={`/api/course/${data.code}/bom`}>BOM JSON</a>
    <a class="link" href={`/api/course/${data.code}/dump`}>Full dump</a>
    <a class="link" href={`/api/course/${data.code}/raw`}>Raw bytes</a>
  </p>

  <h2 class="mt-8 text-xl font-semibold">Bill of materials</h2>
  {#if data.sections.length === 0}
    <p>No pieces were counted for this course.</p>
  {:else}
    {#each data.sections as section (section.title)}
      <h3 class="mt-5 font-semibold">{section.title}</h3>
      <table class="table-zebra table w-full max-w-md">
        <thead><tr><th>Piece</th><th>Count</th></tr></thead>
        <tbody>
          {#each section.rows as row (row.label)}
            <tr><td>{row.label}</td><td>{row.count}</td></tr>
          {/each}
        </tbody>
      </table>
    {/each}
  {/if}
</article>
