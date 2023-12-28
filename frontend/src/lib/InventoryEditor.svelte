<script lang="ts">
  import {onMount} from 'svelte';
  import {type Set} from "./models/Set";
  import {InventoryData} from "./models/Inventory";
  import {_, addMessages, getLocaleFromNavigator, init} from 'svelte-i18n';

  const apiUrl = import.meta.env.VITE_API_URL;
  const storageKey = "inventory";

  let sets: Record<string, Set> | null = null;
  let sortedKeys = [];
  let buildable = null;

  const jsonInventory: String = localStorage.getItem(storageKey);
  let inventory: InventoryData = jsonInventory ? JSON.parse(jsonInventory) : new InventoryData();

  // Fetch all sets
  // TODO: I don't know enough about Svelte, does it maybe make sense to pass these into the component instead?
  onMount(async () => {
    const res = await fetch(`${apiUrl}/set/list`);
    sets = await res.json();

    let acc = {};
    for (const key in sets) {
      const value = sets[key];
      value.names.forEach(({language_code, name}) => {
        if (!acc[language_code]) {
          acc[language_code] = {};
        }

        acc[language_code][value.id] = name;
      });
    }

    for (const lang in acc) {
      addMessages(lang, acc[lang]);
    }

    sortedKeys = Object.keys(sets).sort((keyA, keyB) => {
      const nameA = getNameInLanguage(sets[keyA].names, 'en');
      const nameB = getNameInLanguage(sets[keyB].names, 'en');
      return nameA.localeCompare(nameB);
    });

    init({
      fallbackLocale: 'en',
      initialLocale: getLocaleFromNavigator(),
    });
  });

  // Function to get name in a specific language
  function getNameInLanguage(namesArray, languageCode) {
    const nameEntry = namesArray.find(name => name.language_code === languageCode);
    return nameEntry ? nameEntry.name : 'Unknown'; // Fallback to 'Unknown' if name in the specific language is not found
  }


  function onSubmit() {
    localStorage.setItem(storageKey, JSON.stringify(inventory));
    buildable = null;
    doPost(inventory);
  }

  async function doPost(data) {
    const res = await fetch(`${apiUrl}/buildable`, {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      },
    })

    buildable = await res.json();
  }

  function resetInventory() {
    localStorage.removeItem(storageKey);
    inventory = new InventoryData();
  }
</script>

{#if sets === null}
  <p>Loading set data...</p>
{:else}
  <form on:submit|preventDefault={onSubmit}>
    <ul>
      {#each sortedKeys as key}
        <li>
          <input
              class="input input-sm input-bordered w-24"
              type="number"
              min="0"
              max="1000"
              placeholder="0"
              bind:value="{inventory.sets[key]}"
              name="{key}"
              id="set-{key}"/>
          <label for="set-{key}">{$_(sets[key].id)}</label>
        </li>
      {/each}
    </ul>
    <button class="btn" type="submit">Submit</button>
    <button class="btn" type="reset" on:click={resetInventory}>Reset</button>
  </form>
{/if}

{#if buildable !== null}
  <h2>{$_('buildable_results')}</h2>
  <table>
    <thead>
    <tr>
      <th>Title</th>
      <th>Code</th>
      <th>Added to DB</th>
      <th>Created</th>
      <th>Ravensburger</th>
      <th>Details</th>
      <th>BOM</th>
    </tr>
    </thead>
    <tbody>
    {#each Object.entries(buildable) as [key, value]}
      <tr>
        <td>{value.title}</td>
        <td>{value.course_code}</td>
        <td>{value.date_added_to_db}</td>
        <td>{value.creation_timestamp}</td>
        <td><a href="https://gravitrax.link.ravensburger.com/code/{value.course_code}">Link</a></td>
        <td><a href="{apiUrl}/course/{value.course_code}/dump">Link</a></td>
        <td><a href="{apiUrl}/course/{value.course_code}/bom">Link</a></td>
      </tr>
    {/each}
    </tbody>
  </table>
{/if}
