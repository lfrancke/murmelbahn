import type { PageServerLoad } from './$types';

export const load: PageServerLoad = () => {
  return { seo: { title: 'About', description: 'About the Murmelbahn GraviTrax course index.' } };
};
