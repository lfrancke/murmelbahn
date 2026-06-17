import type { PageServerLoad } from './$types';
import { fetchSets, englishName } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const raw = await fetchSets(fetch);
  const sets = Object.values(raw)
    .map((s) => ({
      id: s.id,
      name: englishName(s),
      pieces: Object.entries(s.content)
        .map(([label, count]) => ({ label, count }))
        .sort((a, b) => b.count - a.count || a.label.localeCompare(b.label))
    }))
    .sort((a, b) => a.name.localeCompare(b.name));
  return {
    sets,
    seo: {
      title: 'Sets',
      description: 'Every GraviTrax set known to the database and the pieces it contains.'
    }
  };
};
