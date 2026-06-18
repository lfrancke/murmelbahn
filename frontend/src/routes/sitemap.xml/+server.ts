import type { RequestHandler } from './$types';

const CACHE_HEADERS = {
  'Content-Type': 'application/xml; charset=utf-8',
  'Cache-Control': 'public, max-age=3600'
};

const PATHS = ['/', '/build', '/sets', '/about'];

export const GET: RequestHandler = ({ url }) => {
  const entries = PATHS.map(
    (p) =>
      `  <url>\n    <loc>${url.origin}${p}</loc>\n    <changefreq>weekly</changefreq>\n  </url>`
  ).join('\n');
  const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${entries}
</urlset>
`;
  return new Response(body, { headers: CACHE_HEADERS });
};
