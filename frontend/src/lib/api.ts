import { env } from '$env/dynamic/private';

const BASE = env.INTERNAL_API ?? 'http://localhost:8080';

export interface Name {
  language_code: string;
  name: string;
}
export interface SetInfo {
  id: string;
  names: Name[];
  content: Record<string, number>;
}
export type SetList = Record<string, SetInfo>;

export interface Bom {
  layers: Record<string, number>;
  tiles: Record<string, number>;
  rails: Record<string, number>;
  walls: Record<string, number>;
  balconies: number;
  rails_small: number;
  rails_medium: number;
  rails_large: number;
  connectors: number;
}

export interface SavedCourse {
  header: { version: string };
  course: { meta_data: { title: string; creation_timestamp: number } };
}

export interface BuildableCourse {
  course_code: string;
  title: string;
  date_added_to_db: string;
  creation_timestamp: string;
}

export interface Inventory {
  sets: Record<string, number>;
  extra_elements: Record<string, number>;
}

async function getJson<T>(path: string, fetcher: typeof fetch): Promise<T | null> {
  const res = await fetcher(`${BASE}${path}`);
  if (res.status === 404) return null;
  if (!res.ok) throw new Error(`API ${path} returned ${res.status}`);
  return (await res.json()) as T;
}

export function fetchSavedCourse(code: string, fetcher: typeof fetch) {
  return getJson<SavedCourse>(`/api/course/${encodeURIComponent(code)}/dump`, fetcher);
}

export function fetchBom(code: string, fetcher: typeof fetch) {
  return getJson<Bom>(`/api/course/${encodeURIComponent(code)}/bom`, fetcher);
}

export async function fetchSets(fetcher: typeof fetch): Promise<SetList> {
  return (await getJson<SetList>('/api/set/list', fetcher)) ?? {};
}

export async function fetchBuildable(
  inventory: Inventory,
  fetcher: typeof fetch
): Promise<BuildableCourse[]> {
  const res = await fetcher(`${BASE}/api/buildable`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(inventory)
  });
  if (!res.ok) throw new Error(`buildable returned ${res.status}`);
  return (await res.json()) as BuildableCourse[];
}

export function englishName(set: SetInfo): string {
  return set.names.find((n) => n.language_code === 'en')?.name ?? set.names[0]?.name ?? set.id;
}
