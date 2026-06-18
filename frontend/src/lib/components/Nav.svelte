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
    <a href="/" class="text-lg font-semibold"
      >Murmelbahn <span class="text-base-content/60 text-sm font-normal"
        >GraviTrax course index</span
      ></a
    >
    <ul class="menu menu-horizontal ml-auto gap-1">
      {#each links as link (link.href)}
        <li>
          <a href={link.href} aria-current={path === link.href ? 'page' : undefined}>{link.label}</a
          >
        </li>
      {/each}
    </ul>
  </nav>
</header>
