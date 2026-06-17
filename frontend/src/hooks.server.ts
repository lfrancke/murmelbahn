import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BASE = env.INTERNAL_API ?? 'http://localhost:8080';

export const handle: Handle = async ({ event, resolve }) => {
  if (event.url.pathname.startsWith('/api/')) {
    const target = `${BASE}${event.url.pathname}${event.url.search}`;
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
