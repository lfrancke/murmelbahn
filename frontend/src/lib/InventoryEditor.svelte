<script lang="ts">
  import {onMount} from 'svelte';
  import {type Set} from "./models/Set";
  import {type Inventory} from "./models/Inventory";
  import {addMessages, getLocaleFromNavigator, init, _} from 'svelte-i18n';

  let sets: Record<string, Set> | null = null;
  let buildable = null;
  let apiUrl = import.meta.env.VITE_API_URL;

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

  function onSubmit(e) {
    const formData: FormData = new FormData(e.target);

    const data: Inventory = {
      extra_elements: undefined,
      sets: {}
    };
    for (let field of formData) {
      const [key, value] = field;
      data.sets[key] = Number(value);
    }
    doPost(data);
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
</script>

{#if sets === null}
  <p>Loading set data...</p>
{:else}
  <form on:submit|preventDefault={onSubmit}>
    <ul>
      {#each Object.entries(sets) as [key, value]}
        <li>
          <input class="input input-sm input-bordered w-24" type="number" value="0" name="{key}" id="set-{key}"/>
          <label for="set-{key}">{$_(value.id)}</label>
        </li>
      {/each}
    </ul>
    <button class="btn" type="submit">Submit</button>
  </form>
{/if}

{#if buildable !== null}
  <h2>Tracks you can build</h2>
  <ul>
    {#each Object.entries(buildable) as [key, value]}
      <li><a href="{apiUrl}/course/{value}/bom">{value}</a></li>
    {/each}
  </ul>
{/if}
