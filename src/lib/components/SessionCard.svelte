<script lang="ts">
  import { Hash, Calendar, ArrowRight, CalendarCheck, Clock, Ticket, TriangleAlert, GraduationCap } from '@lucide/svelte';
  import PillBadge from './PillBadge.svelte';
  import Chip from './Chip.svelte';
  import type { BubbleSession } from '$lib/types';
  import { longDate, isSessionActive } from '$lib/format';

  let { session }: { session: BubbleSession } = $props();

  const headline = $derived.by(() => {
    const parts: string[] = [];
    if (session.year) parts.push(session.year);
    if (session.trimester) parts.push(session.trimester);
    if (session.schoolDay) parts.push(session.schoolDay);
    return parts.join(' · ') || 'Session';
  });

  const accent = $derived.by(() => {
    if (session.disabledDate) return 'gray';
    if (session.isInClass) return 'blue';
    if (session.examDate) return 'indigo';
    const now = Date.now();
    if (session.startDate && new Date(session.startDate).getTime() > now) return 'blue';
    if (session.endDate && new Date(session.endDate).getTime() < now) return 'gray';
    return 'purple';
  });
  const accentVar = $derived(`var(--${accent})`);

  const total = $derived(session.regularTickets ?? 0);
  const remaining = $derived(session.remainingRegularTickets ?? 0);
  const ticketFraction = $derived(total > 0 ? remaining / total : 0);
  const ticketColor = $derived(ticketFraction < 0.15 ? 'var(--red)' : ticketFraction < 0.35 ? 'var(--orange)' : 'var(--green)');
</script>

<div class="card-accent" class:dimmed={!!session.disabledDate}>
  <div class="accent-bar" style="background: {accentVar};"></div>
  <div class="accent-body">
    <div class="flex items-baseline gap-6 flex-wrap">
      <span class="text-sm fw-semibold">{headline}</span>
      {#if session.sessionType}<PillBadge text={session.sessionType} tint="purple" />{/if}
      {#if session.isInClass}<PillBadge text="EN CLASSE" tint="blue" strong />{/if}
      {#if session.disabledDate}<PillBadge text="DÉSACTIVÉE" tint="red" />{/if}
      {#if session.examDate}<PillBadge text="EXAMEN" tint="indigo" />{/if}
      <div class="flex-1"></div>
      <Chip text={'…' + session.id.slice(-6)} Icon={Hash} tint="gray" clipboardText={session.id} copyable mono />
    </div>
    {#if session.startDate || session.endDate}
      <div class="flex items-center gap-8 text-sm text-secondary">
        {#if session.startDate}
          <span class="flex items-center gap-4"><Calendar size={11} color={accentVar} />{longDate(session.startDate)}</span>
        {/if}
        {#if session.startDate && session.endDate}
          <ArrowRight size={11} />
        {/if}
        {#if session.endDate}
          <span class="flex items-center gap-4"><CalendarCheck size={11} color={accentVar} />{longDate(session.endDate)}</span>
        {/if}
      </div>
    {/if}
    {#if total > 0}
      <div>
        <div class="flex items-center gap-4 text-2xs">
          <Ticket size={11} color={ticketColor} />
          <span class="fw-bold" style="color: {ticketColor};">{remaining}</span>
          <span class="text-secondary">/ {total} tickets</span>
        </div>
        <div class="tickets-bar" style="margin-top: 3px;">
          <div style="width: {Math.max(0, Math.min(1, ticketFraction)) * 100}%; background: {ticketColor};"></div>
        </div>
      </div>
    {/if}
    <div class="flex items-center gap-12 text-sm text-secondary">
      {#if session.requiredHours}<span class="flex items-center gap-4"><Clock size={11} />{session.requiredHours} h</span>{/if}
      {#if session.absenceWarnings && session.absenceWarnings > 0}
        <span class="flex items-center gap-4" style="color: var(--orange);"><TriangleAlert size={11} />{Math.round(session.absenceWarnings)} absence{session.absenceWarnings > 1 ? 's' : ''}</span>
      {/if}
    </div>
    {#if session.examDate}
      <span class="flex items-center gap-4 text-xs fw-semibold" style="color: var(--indigo);">
        <GraduationCap size={12} />Examen le {longDate(session.examDate)}
      </span>
    {/if}
  </div>
</div>
