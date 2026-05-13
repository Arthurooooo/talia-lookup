import type { BubbleUser, RecentUserStub } from './types';
import { fullName } from './types';

const KEY = 'talia.recents.v1';
const LIMIT = 12;

export function loadRecents(): RecentUserStub[] {
  try {
    const raw = localStorage.getItem(KEY);
    return raw ? (JSON.parse(raw) as RecentUserStub[]) : [];
  } catch { return []; }
}

export function saveRecents(items: RecentUserStub[]) {
  localStorage.setItem(KEY, JSON.stringify(items));
}

export function addRecent(u: BubbleUser): RecentUserStub[] {
  const items = loadRecents().filter(r => r.id !== u.id);
  items.unshift({
    id: u.id,
    fullName: fullName(u),
    subtitle: u.studentStatus ?? u.leadStatus,
    avatarUrl: u.profilePicture,
    visitedAt: new Date().toISOString(),
  });
  const trimmed = items.slice(0, LIMIT);
  saveRecents(trimmed);
  return trimmed;
}

export function removeRecent(id: string): RecentUserStub[] {
  const items = loadRecents().filter(r => r.id !== id);
  saveRecents(items);
  return items;
}

export function clearRecents() {
  localStorage.removeItem(KEY);
}
