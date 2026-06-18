import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { fetchSavedCourse, fetchBom } from '$lib/api';
import { bomSections } from '$lib/bom';

export const load: PageServerLoad = async ({ params, fetch }) => {
  const code = params.code.toUpperCase();
  const [saved, bom] = await Promise.all([fetchSavedCourse(code, fetch), fetchBom(code, fetch)]);
  if (!saved || !bom) throw error(404, `Course ${code} could not be found.`);

  // SkyTrax and other new-app courses arrive with a zeroed metadata header: no
  // title and a zero creation timestamp. Fall back to the code as the name, and
  // omit the date rather than showing 1970-01-01.
  const ts = saved.course.meta_data.creation_timestamp;
  const rawTitle = saved.course.meta_data.title.trim();
  const title = rawTitle.length > 0 ? rawTitle : code;
  const created = ts > 0 ? new Date(ts).toISOString().slice(0, 10) : null;
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
