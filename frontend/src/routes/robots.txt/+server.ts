import type { RequestHandler } from './$types';

const CACHE_HEADERS = {
  'Content-Type': 'text/plain; charset=utf-8',
  'Cache-Control': 'public, max-age=3600'
};

const AI_USER_AGENTS = [
  'GPTBot',
  'ChatGPT-User',
  'OAI-SearchBot',
  'ClaudeBot',
  'Claude-Web',
  'anthropic-ai',
  'PerplexityBot',
  'Perplexity-User',
  'Google-Extended',
  'Applebot-Extended',
  'CCBot',
  'cohere-ai',
  'Bytespider'
];

export const GET: RequestHandler = ({ url }) => {
  const aiBlocks = AI_USER_AGENTS.map((ua) => `User-agent: ${ua}\nAllow: /`).join('\n\n');
  const body = `# Murmelbahn, an unofficial GraviTrax course index.
# Public site, AI and LLM crawlers are welcome.
# See also: ${url.origin}/llms.txt and ${url.origin}/sitemap.xml

User-agent: *
Allow: /

${aiBlocks}

Sitemap: ${url.origin}/sitemap.xml
`;
  return new Response(body, { headers: CACHE_HEADERS });
};
