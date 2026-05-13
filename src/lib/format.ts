// Formatters de dates / monnaie / durées.

export function longDate(iso?: string): string {
  if (!iso) return '';
  const d = new Date(iso);
  if (isNaN(d.getTime())) return iso;
  return d.toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' });
}

export function relativeDate(iso?: string): string {
  if (!iso) return '';
  const d = new Date(iso);
  if (isNaN(d.getTime())) return iso;
  const diffSec = (Date.now() - d.getTime()) / 1000;
  const abs = Math.abs(diffSec);
  if (abs < 60) return 'à l\'instant';
  if (abs < 3600) return `il y a ${Math.floor(abs/60)} min`;
  if (abs < 86400) return `il y a ${Math.floor(abs/3600)} h`;
  if (abs < 86400*7) return `il y a ${Math.floor(abs/86400)} j`;
  return longDate(iso);
}

export function durationString(start?: string, end?: string): string {
  if (!start || !end) return '';
  const a = new Date(start), b = new Date(end);
  if (isNaN(a.getTime()) || isNaN(b.getTime())) return '';
  const days = Math.round((b.getTime() - a.getTime()) / 86400000);
  if (days < 60) return `${days} j`;
  const months = Math.floor(days / 30);
  if (months < 24) return `${months} mois`;
  return `${(days/365).toFixed(1)} ans`;
}

export function currency(n?: number): string {
  if (n == null) return '';
  return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'EUR', maximumFractionDigits: 0 }).format(n);
}

export function ageFromBirth(iso?: string): number | null {
  if (!iso) return null;
  const d = new Date(iso);
  if (isNaN(d.getTime())) return null;
  const ms = Date.now() - d.getTime();
  return Math.floor(ms / (365.25 * 24 * 3600 * 1000));
}

export function isContractActive(c: { isCancelled?: boolean; isTerminated?: boolean; isTerminationOccurred?: boolean; startDate?: string; endDate?: string }): boolean {
  if (c.isCancelled) return false;
  if (c.isTerminated || c.isTerminationOccurred) return false;
  const now = Date.now();
  if (c.startDate && new Date(c.startDate).getTime() > now) return false;
  if (c.endDate && new Date(c.endDate).getTime() < now) return false;
  return true;
}

export function isSessionActive(s: { disabledDate?: string; endDate?: string }): boolean {
  if (s.disabledDate) return false;
  const now = Date.now();
  if (s.endDate && new Date(s.endDate).getTime() < now) return false;
  return true;
}
