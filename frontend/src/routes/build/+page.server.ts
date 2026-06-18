import type { PageServerLoad, Actions } from './$types';
import { fetchSets, fetchBuildable, englishName, type Inventory } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const raw = await fetchSets(fetch);
  const sets = Object.values(raw)
    .map((s) => ({ id: s.id, name: englishName(s) }))
    .sort((a, b) => a.name.localeCompare(b.name));
  return {
    sets,
    seo: {
      title: 'What can I build?',
      description:
        'Enter the GraviTrax sets you own and find every track in the database you can build.'
    }
  };
};

export const actions: Actions = {
  default: async ({ request, fetch }) => {
    const form = await request.formData();
    const sets: Record<string, number> = {};
    for (const [key, value] of form.entries()) {
      const n = parseInt(String(value), 10);
      if (Number.isFinite(n) && n > 0) sets[key] = n;
    }
    const inventory: Inventory = { sets, extra_elements: {} };
    const courses = await fetchBuildable(inventory, fetch);
    return { courses };
  }
};
