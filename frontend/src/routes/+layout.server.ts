import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = ({ url }) => {
  return { origin: url.origin, path: url.pathname };
};
