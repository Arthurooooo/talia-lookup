// Toast global pour les copies dans le presse-papier.
import { writable } from 'svelte/store';

export const toast = writable<{ text: string; t: number } | null>(null);

let timer: number | null = null;

export function showToast(text: string) {
  if (timer) { clearTimeout(timer); timer = null; }
  toast.set({ text, t: Date.now() });
  timer = setTimeout(() => toast.set(null), 1400) as unknown as number;
}

export async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    showToast(text);
  } catch (e) {
    console.error('clipboard write failed', e);
  }
}
