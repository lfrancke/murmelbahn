import type { Bom } from './api';

export interface BomRow {
  label: string;
  count: number;
}
export interface BomSection {
  title: string;
  rows: BomRow[];
}

function rows(map: Record<string, number>): BomRow[] {
  return Object.entries(map)
    .filter(([, count]) => count > 0)
    .map(([label, count]) => ({ label, count }))
    .sort((a, b) => b.count - a.count || a.label.localeCompare(b.label));
}

export function bomSections(bom: Bom): BomSection[] {
  const sections: BomSection[] = [];
  for (const [title, map] of [
    ['Tiles', bom.tiles],
    ['Rails', bom.rails],
    ['Walls', bom.walls],
    ['Layers', bom.layers]
  ] as const) {
    const r = rows(map);
    if (r.length) sections.push({ title, rows: r });
  }
  if (bom.connectors > 0) {
    sections.push({ title: 'Connectors', rows: [{ label: 'Connector', count: bom.connectors }] });
  }
  return sections;
}
