<script lang="ts">
  let courseCode: string | null = null;
  let apiUrl = import.meta.env.VITE_API_URL;
</script>

<div class="form-control w-full max-w-xs">
  <label class="label" for="courseCode">
    <span class="label-text">What is the course code?</span>
  </label>
  <input id="courseCode" type="text" placeholder="Enter the course code here"
         class="input input-bordered input-primary w-full max-w-xs" bind:value={courseCode}/>
</div>

{#if courseCode === null}
  <p>Please enter a course code to get links to detailed information</p>
  <p>This is mostly useless unless you are interested in technical details, BUT it is currently the only way to add a new course to the database.</p>
  <p>So, if you want to add a course, enter the code above and click any of the links that appear. You can close the tab immediately. At some point this will be made nicer...</p>
{:else}
  <ul>
    <li><a href="{apiUrl}/course/{courseCode}/dump">Course dump (JSON)</a></li>
    <li><a href="{apiUrl}/course/{courseCode}/bom">Bill of Materials (JSON)</a></li>
    <li><a href="{apiUrl}/course/{courseCode}/bom?format=csv" target="_blank">Bill of Materials
      (Gravisheet CSV)</a></li>
    <li><a href="https://gravitrax.link.ravensburger.com/code/{courseCode}">Ravensburger App Link</a></li>
  </ul>
{/if}
