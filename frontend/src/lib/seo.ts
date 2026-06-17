export const SITE_NAME = 'Murmelbahn';
export const DEFAULT_TITLE = 'Murmelbahn, a GraviTrax course index';
export const DEFAULT_DESCRIPTION =
  'Look up any GraviTrax course by its code to see the full bill of materials, or enter the sets you own to find every track you can build.';
export const PUBLISHER = {
  name: 'Lars Francke',
  url: 'https://github.com/lfrancke'
};

export function buildPageTitle(pageTitle: string | undefined): string {
  if (!pageTitle || pageTitle === SITE_NAME) return SITE_NAME;
  return `${pageTitle} · ${SITE_NAME}`;
}
