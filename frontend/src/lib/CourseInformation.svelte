<script lang="ts">
  let courseCode: string | null = null;
  let apiUrl = import.meta.env.VITE_API_URL;
</script>

<div class="form-control w-full max-w-xs">
  <label class="label" for="courseCode">
    <span class="label-text">What is the course code?</span>
  </label>
  <input id="courseCode" type="text" placeholder="Enter the course code here" class="input input-bordered input-primary w-full max-w-xs" bind:value={courseCode} />
</div>

{#if courseCode === null}
  <p>Please enter a course code to get links to detailed information</p>
{:else}
  <ul>
    <li><a href="{apiUrl}/course/{courseCode}/dump">Course dump (JSON)</a></li>
    <li><a href="{apiUrl}/course/{courseCode}/bom">Bill of Materials (JSON)</a></li>
    <li><a href="{apiUrl}/course/{courseCode}/bom?format=csv" target="_blank">Bill of Materials
      (Gravisheet CSV)</a></li>
  </ul>
{/if}
