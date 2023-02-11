<script lang="ts">
  import InventoryEditor from "./lib/InventoryEditor.svelte";
  import CourseInformation from "./lib/CourseInformation.svelte";
  import {_, addMessages, getLocaleFromNavigator, init} from "svelte-i18n";

  // TODO: Is it possible to only load the languages we really need?
  import en from './assets/en.json';
  import de from './assets/de.json';

  addMessages('en', en);
  addMessages('de', de);

  init({
    fallbackLocale: 'en',
    initialLocale: getLocaleFromNavigator(),
  });

</script>

<main class="prose container mx-auto py-4">

  <div class="grid grid-cols-2 gap-4">
    <div class="col-span-2">
      <h1>Murmelbahn</h1>
    </div>

    <div>
      <h2>{$_('title_buildable')}</h2>
      <div class="alert alert-info shadow-lg">
        This is experimental!<br/>
        Some of these courses cannot be built
      </div>
      <div class="pt-4">
        Please enter how many you have of each set and press `Submit`.
        If there are any tracks in our database that can be built with the parts you have, they will be shown below.
      </div>
      <InventoryEditor/>
    </div>
    <div>
      <h2>{$_('title_course_information')}</h2>
      <CourseInformation/>
    </div>
  </div>

</main>
<footer class="footer p-10 bg-neutral text-neutral-content">
  <div>
    <p><a href="https://github.com/lfrancke/murmelbahn">Built by Lars Francke | Source and contact information on
      Github</a> - <a href="https://github.com/lfrancke/murmelbahn/blob/main/CHANGELOG.md">Changelog</a></p>
  </div>
</footer>
