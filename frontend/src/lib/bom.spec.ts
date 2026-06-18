import { describe, it, expect } from 'vitest';
import { bomSections } from './bom';
import type { Bom } from './api';

const empty: Bom = {
  layers: {},
  tiles: {},
  rails: {},
  walls: {},
  balconies: 0,
  rails_small: 0,
  rails_medium: 0,
  rails_large: 0,
  connectors: 0
};

describe('bomSections', () => {
  it('groups tiles sorted by count descending then label', () => {
    const bom: Bom = { ...empty, tiles: { Cannon: 2, Starter: 5 }, connectors: 3 };
    const sections = bomSections(bom);
    const tiles = sections.find((s) => s.title === 'Tiles');
    expect(tiles).toBeDefined();
    expect(tiles!.rows[0]).toEqual({ label: 'Starter', count: 5 });
    expect(tiles!.rows[1]).toEqual({ label: 'Cannon', count: 2 });
    expect(sections.find((s) => s.title === 'Connectors')).toBeDefined();
  });

  it('omits empty groups', () => {
    const sections = bomSections(empty);
    expect(sections).toHaveLength(0);
  });
});
