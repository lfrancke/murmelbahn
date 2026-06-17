import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { fetchSavedCourse, fetchBom } from '$lib/api';
import { bomSections } from '$lib/bom';

export const load: PageServerLoad = async ({ params, fetch }) => {
  const code = params.code.toUpperCase();
  const [saved, bom] = await Promise.all([fetchSavedCourse(code, fetch), fetchBom(code, fetch)]);
  if (!saved || !bom) throw error(404, `Course ${code} could not be found.`);

  const created = new Date(saved.course.meta_data.creation_timestamp).toISOString().slice(0, 10);
  const title = saved.course.meta_data.title;
  const sections = bomSections(bom);

  return {
    code,
    title,
    version: saved.header.version,
    created,
    sections,
    seo: { title, description: `Bill of materials for the GraviTrax course ${title} (${code}).` }
  };
};
