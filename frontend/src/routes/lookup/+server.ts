import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = ({ url }) => {
  const raw = url.searchParams.get('code') ?? '';
  const code = raw.replace(/[^a-zA-Z0-9]/g, '').toUpperCase();
  if (!code) throw redirect(303, '/');
  throw redirect(303, `/course/${code}`);
};
