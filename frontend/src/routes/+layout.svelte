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
