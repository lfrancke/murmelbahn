import { describe, it, expect } from 'vitest';
import { buildLlmsTxt } from './llms-content';

describe('buildLlmsTxt', () => {
  it('includes the site name, the key pages, and the API base', () => {
    const txt = buildLlmsTxt('https://example.test');
    expect(txt).toContain('# Murmelbahn');
    expect(txt).toContain('https://example.test/build');
    expect(txt).toContain('https://example.test/api/course/');
  });
});
