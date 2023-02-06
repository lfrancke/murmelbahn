<script lang="ts">
    import {onMount} from 'svelte';
    import {Set} from "./models/Set";
    import {Inventory} from "./models/Inventory";

    let data: Record<string, Set> | null = null;
    let buildable = null;

    onMount(async () => {
        const res = await fetch('/set/list');
        data = await res.json();
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
        const res = await fetch('/buildable', {
            method: 'POST',
            body: JSON.stringify(data),
            headers: {
                'Content-Type': 'application/json'
            },
        })

        buildable = await res.json();
    }
</script>

{#if data === null}
    <p>Loading...</p>
{:else}
    <form on:submit|preventDefault={onSubmit}>
        <ul>
            {#each Object.entries(data) as [key, value]}
                <li>
                    <input class="input input-sm input-bordered w-24" type="number" value="0" name="{key}" id="set-{key}"/>
                    <label for="set-{key}">{value.id}</label>
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
            <li>{value}</li>
        {/each}
    </ul>
{/if}
