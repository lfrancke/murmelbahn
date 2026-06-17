import { SITE_NAME, DEFAULT_DESCRIPTION } from './seo';

// Markdown summary served at /llms.txt (llmstxt.org convention) so agents can
// understand the site and use the JSON/CSV API.
export function buildLlmsTxt(origin: string): string {
  return `# ${SITE_NAME}

> ${DEFAULT_DESCRIPTION}

## Pages

- [Home and course lookup](${origin}/)
- [What can I build?](${origin}/build): enter the sets you own to find buildable tracks.
- [Sets](${origin}/sets): every set and its pieces.
- [About](${origin}/about)

## API

A course is identified by its app code. For a code CODE:

- ${origin}/api/course/CODE/bom : bill of materials as JSON.
- ${origin}/api/course/CODE/bom?format=csv : bill of materials as GraviSheet CSV.
- ${origin}/api/course/CODE/dump : the full parsed course as JSON.
- ${origin}/api/set/list : every known set and its contents.
`;
}
