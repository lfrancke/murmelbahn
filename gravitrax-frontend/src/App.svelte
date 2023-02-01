<script lang="ts">
    import {onMount} from 'svelte';
    import {Set} from "./lib/models/Set";
    import type {Inventory} from "./lib/models/Inventory";

    let data: Record<string, Set> | null = null;
    let buildable = null;

    onMount(async () => {
        const res = await fetch('http://localhost:3000/set/list');
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
        const res = await fetch('http://localhost:3000/buildable', {
            method: 'POST',
            body: JSON.stringify(data),
            headers: {
                'Content-Type': 'application/json'
            },
        })

        buildable = await res.json();
    }
</script>

<main class="prose">
    <h1>Murmelbahn</h1>

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
</main>

<style>
    /*
    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
        Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }

    main {
        text-align: center;
        padding: 1em;
        margin: 0 auto;
    }

    img {
        height: 16rem;
        width: 16rem;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4rem;
        font-weight: 100;
        line-height: 1.1;
        margin: 2rem auto;
        max-width: 14rem;
    }

    p {
        max-width: 14rem;
        margin: 1rem auto;
        line-height: 1.35;
    }

    @media (min-width: 480px) {
        h1 {
            max-width: none;
        }

        p {
            max-width: none;
        }
    }

     */
</style>
