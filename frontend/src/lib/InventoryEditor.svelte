<script lang="ts">
  import {onMount} from 'svelte';
  import {type Set} from "./models/Set";
  import {InventoryData} from "./models/Inventory";
  import {_, addMessages, getLocaleFromNavigator, init} from 'svelte-i18n';

  const apiUrl = import.meta.env.VITE_API_URL;
  const storageKey = "inventory";

  let sets: Record<string, Set> | null = null;
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

    init({
      fallbackLocale: 'en',
      initialLocale: getLocaleFromNavigator(),
    });
  });

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
      {#each Object.entries(sets) as [key, value]}
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
          <label for="set-{key}">{$_(value.id)}</label>
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
      <th>Code</th>
      <th>Ravensburger</th>
      <th>Details</th>
      <th>BOM</th>
    </tr>
    </thead>
    <tbody>
    {#each Object.entries(buildable) as [key, value]}
      <tr>
        <td>{value}</td>
        <td><a href="https://gravitrax.link.ravensburger.com/code/{value}">Link</a></td>
        <td><a href="{apiUrl}/course/{value}/dump">Link</a></td>
        <td><a href="{apiUrl}/course/{value}/bom">Link</a></td>
      </tr>
    {/each}
    </tbody>
  </table>
{/if}
