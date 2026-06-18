import type { PageServerLoad } from './$types';
import { DEFAULT_DESCRIPTION } from '$lib/seo';

export const load: PageServerLoad = () => {
  return { seo: { title: undefined, description: DEFAULT_DESCRIPTION } };
};
