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

  <div class="w-4/5 mx-auto flex flex-col">

    <div class="bg-white py-4 px-2 mb-2">
      <h1>Murmelbahn</h1>
    </div>


    <div class="mx-1 p-1">
      <h2>{$_('title_about')}</h2>
      <ul>
        <li>This site was <a href="https://www.linkedin.com/in/larsfrancke/">built by Lars Francke</a>.
        It is in no way affiliated with Ravensburger, please do not contact them if you have issues or questions about this site.</li>
        <li>It is <a href="https://github.com/lfrancke/murmelbahn">open-source</a>, please create an issue on GitHub if you are looking for improvements or find a bug.</li>
        <li>See the <a href="https://github.com/lfrancke/murmelbahn/blob/main/CHANGELOG.md">Changelog</a> for what's new (there is not much happening due to a lack of time)</li>
        <li>You can contact me via e-mail at murmelbahn(at)lars-francke.de</li>
        <li>Also check out the <a href="https://docs.google.com/spreadsheets/d/1T-hLIBz05q4QMlt7xQ63Y1SrnG_3HYWchF_nLXvYkJg/template/preview">GraviSheet</a>: A Google Sheets version of this site (it started long before this site) which is nicer to use in general</li>
      </ul>
    </div>

    <div class="mx-1 p-2">
      <h2>{$_('title_course_information')}</h2>
      <CourseInformation/>
    </div>

    <div class="mx-1 p-2">
      <h2>{$_('title_buildable')}</h2>
      <div class="pt-4">
        Please enter how many you have of each set and press `Submit`.
        If there are any tracks in our database that can be built with the parts you have, they will be shown below.
      </div>
      <InventoryEditor/>
    </div>

  </div>

</main>
<footer class="footer p-10 bg-neutral text-neutral-content">
  <div>
    <p><a href="https://github.com/lfrancke/murmelbahn">Built by Lars Francke | Source and contact information on
      Github</a> - <a href="https://github.com/lfrancke/murmelbahn/blob/main/CHANGELOG.md">Changelog</a></p>
  </div>
</footer>
