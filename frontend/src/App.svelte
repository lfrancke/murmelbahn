<script lang="ts">
  import InventoryEditor from "./lib/InventoryEditor.svelte";
  import CourseInformation from "./lib/CourseInformation.svelte";
</script>

<main class="prose container mx-auto py-4">

  <div class="grid grid-cols-2">
    <div class="col-span-2">
      <h1>Murmelbahn</h1>
    </div>
    <div>
      <h2>What can I build?</h2>
      <InventoryEditor/>
    </div>
    <div>
      <h2>Course Information</h2>
      <CourseInformation/>
    </div>
  </div>


</main>
