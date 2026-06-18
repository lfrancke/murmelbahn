<script lang="ts">
  import { enhance } from '$app/forms';
  let { data, form } = $props();
  const STORAGE_KEY = 'mb_inventory';

  let formEl: HTMLFormElement;

  function restore() {
    if (typeof localStorage === 'undefined') return;
    let saved: Record<string, number>;
    try {
      saved = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? '{}') as Record<string, number>;
    } catch {
      saved = {};
    }
    for (const input of formEl.querySelectorAll<HTMLInputElement>('input[type="number"]')) {
      if (saved[input.name] != null) input.value = String(saved[input.name]);
    }
  }

  function save() {
    if (typeof localStorage === 'undefined') return;
    const sets: Record<string, number> = {};
    for (const input of formEl.querySelectorAll<HTMLInputElement>('input[type="number"]')) {
      const n = parseInt(input.value, 10);
      if (Number.isFinite(n) && n > 0) sets[input.name] = n;
    }
    localStorage.setItem(STORAGE_KEY, JSON.stringify(sets));
  }

  $effect(() => {
    restore();
  });
</script>

<h1 class="text-2xl font-bold">What can I build?</h1>
<p class="text-base-content/70 mt-1">
  Enter how many of each set you own, then submit. Tracks you can build are listed below.
</p>

<form
  bind:this={formEl}
  method="post"
  use:enhance
  oninput={save}
  onreset={() => localStorage.removeItem(STORAGE_KEY)}
  class="mt-6"
>
  <div class="grid gap-x-8 gap-y-1 sm:grid-cols-2">
    {#each data.sets as set (set.id)}
      <label class="flex items-center gap-3 py-1">
        <span class="flex-1 text-sm">{set.name}</span>
        <input
          type="number"
          name={set.id}
          min="0"
          value="0"
          inputmode="numeric"
          class="input input-bordered input-sm w-20 font-mono"
        />
      </label>
    {/each}
  </div>
  <div class="mt-4 flex gap-2">
    <button type="submit" class="btn btn-primary">Show buildable courses</button>
    <button type="reset" class="btn btn-ghost">Reset</button>
  </div>
</form>

{#if form?.courses}
  <section class="mt-8">
    <h2 class="text-xl font-semibold">Buildable courses ({form.courses.length})</h2>
    {#if form.courses.length === 0}
      <p>No courses in the database can be built with that inventory yet.</p>
    {:else}
      <ul class="mt-2 columns-1 sm:columns-2">
        {#each form.courses as course (course.course_code)}
          <li class="py-1">
            <a class="link" href={`/course/${course.course_code}`}>
              {course.title.trim() || course.course_code}
            </a>
            {#if course.title.trim()}
              <span class="text-base-content/50 font-mono text-xs">{course.course_code}</span>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  </section>
{/if}
