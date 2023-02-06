<script lang="ts">
  import InventoryEditor from "./lib/InventoryEditor.svelte";
  import CourseInformation from "./lib/CourseInformation.svelte";
</script>

<main class="prose container mx-auto py-4">

  <div class="grid grid-cols-2 gap-4">
    <div class="col-span-2">
      <h1>Murmelbahn</h1>
    </div>

    <div>
      <h2>What can I build?</h2>
      <div class="alert alert-info shadow-lg">
        <div>
          This is experimental!<br/>
          Some of these courses cannot be built
        </div>
      </div>
      <InventoryEditor/>
    </div>
    <div>
      <h2>Course Information</h2>
      <CourseInformation/>
    </div>
  </div>

</main>
<footer class="footer p-10 bg-neutral text-neutral-content">
  <div>
    <p><a href="https://github.com/lfrancke/murmelbahn">Built by Lars Francke | Source and contact information on Github</a></p>
  </div>
</footer>
