// Build a JSON-LD <script> tag string for injection into <svelte:head> via
// {@html}. The closing tag is split so the Svelte template parser doesn't
// see `</script>` literally, otherwise it treats the substring as a real
// closing tag and fails to parse the surrounding template.
//
// The payload is a typed object we build, but several fields are DB-derived
// (component names, release versions, CRD kinds/groups). `JSON.stringify` does
// not escape `<`, `>` or `&`, so a value containing `</script>` would break out
// of the script element into HTML, the canonical JSON-LD XSS. `escapeForScript`
// neutralises that: the escapes are valid inside JSON string literals, so the
// JSON-LD still parses to exactly the same object, it just can no longer
// terminate the surrounding `<script>` context.
function escapeForScript(json: string): string {
  return json.replace(/</g, '\\u003c').replace(/>/g, '\\u003e').replace(/&/g, '\\u0026');
}

export function jsonLdScript(value: unknown): string {
  const json = escapeForScript(JSON.stringify(value));
  return `<script type="application/ld+json">${json}</` + `script>`;
}
