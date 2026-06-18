import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BASE = env.INTERNAL_API ?? 'http://localhost:8080';

export const handle: Handle = async ({ event, resolve }) => {
  // Forward the JSON/CSV API and the Prometheus metrics to the Rust backend.
  // Everything else is served by SvelteKit.
  const path = event.url.pathname;
  if (path.startsWith('/api/') || path === '/metrics') {
    const target = `${BASE}${path}${event.url.search}`;
    const method = event.request.method;
    const body =
      method === 'GET' || method === 'HEAD' ? undefined : await event.request.arrayBuffer();
    const upstream = await fetch(target, {
      method,
      headers: { 'content-type': event.request.headers.get('content-type') ?? 'application/json' },
      body
    });
    return new Response(upstream.body, {
      status: upstream.status,
      headers: {
        'content-type': upstream.headers.get('content-type') ?? 'application/octet-stream'
      }
    });
  }
  return resolve(event);
};
