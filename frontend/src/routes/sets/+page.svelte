<script lang="ts">
  let { data } = $props();
</script>

<h1 class="text-2xl font-bold">Sets</h1>
<p class="text-base-content/70 mt-1">Every set known to the database and the pieces it contains.</p>

{#each data.sets as set (set.id)}
  <details class="collapse-arrow bg-base-200 collapse mt-2">
    <summary class="collapse-title font-medium"
      >{set.name} <span class="text-base-content/50 font-mono text-sm">({set.id})</span></summary
    >
    <div class="collapse-content">
      <table class="table w-full max-w-md">
        <thead><tr><th>Piece</th><th>Count</th></tr></thead>
        <tbody>
          {#each set.pieces as piece (piece.label)}
            <tr><td>{piece.label}</td><td>{piece.count}</td></tr>
          {/each}
        </tbody>
      </table>
    </div>
  </details>
{/each}
