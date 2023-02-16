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

<main class="prose mx-auto max-w-screen-lg">

  <div class="flex">
    <div class="flex flex-col">
      <div class="bg-white py-4 px-6 mb-4">
        <h1>Murmelbahn</h1>
      </div>

      <div class="flex justify-center">
        <div class="w-5/12 mx-1 p-6">
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
        <div class="w-5/12 mx-1 p-6">
          <h2>{$_('title_course_information')}</h2>
          <CourseInformation/>
        </div>
      </div>
    </div>
  </div>

</main>
<footer class="footer p-10 bg-neutral text-neutral-content">
  <div>
    <p><a href="https://github.com/lfrancke/murmelbahn">Built by Lars Francke | Source and contact information on
      Github</a> - <a href="https://github.com/lfrancke/murmelbahn/blob/main/CHANGELOG.md">Changelog</a></p>
  </div>
</footer>
