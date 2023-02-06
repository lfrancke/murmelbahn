<script lang="ts">
  let courseCode: string | null = null;
</script>

<input type="text" placeholder="Course code" class="input w-full max-w-xs" bind:value="{courseCode}"/>

{#if courseCode === null}
  <p>Please enter a course code to get detailed information</p>
{:else}
  <ul>
    <li><a href="/course/{courseCode}/dump">Course dump (JSON)</a></li>
    <li><a href="/course/{courseCode}/bom">Bill of Materials (JSON)</a></li>
    <li><a href="/course/{courseCode}/bom?format=csv" target="_blank">Bill of Materials
      (Gravisheet CSV)</a></li>
  </ul>
{/if}
