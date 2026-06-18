import type { RequestHandler } from './$types';
import { buildLlmsTxt } from '$lib/llms-content';

const CACHE_HEADERS = {
  'Content-Type': 'text/plain; charset=utf-8',
  'Cache-Control': 'public, max-age=3600'
};

export const GET: RequestHandler = ({ url }) => {
  return new Response(buildLlmsTxt(url.origin), { headers: CACHE_HEADERS });
};
