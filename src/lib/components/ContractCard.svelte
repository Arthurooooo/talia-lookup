<script lang="ts">
  import { Hash, Euro, Clock, Users, OctagonAlert, Calendar, ArrowRight, CalendarCheck } from '@lucide/svelte';
  import PillBadge from './PillBadge.svelte';
  import Chip from './Chip.svelte';
  import type { BubbleContract } from '$lib/types';
  import { longDate, durationString, currency, isContractActive } from '$lib/format';

  let { contract }: { contract: BubbleContract } = $props();

  const isCancelled = $derived(contract.isCancelled === true);
  const isRuptured = $derived(contract.isTerminated === true || contract.isTerminationOccurred === true);
  const active = $derived(isContractActive(contract));

  const primaryLabel = $derived.by(() => {
    if (isCancelled) return 'ANNULÉ';
    if (contract.isTerminationOccurred) return 'ROMPU';
    if (contract.isTerminated) return 'RUPTURE';
    if (contract.isTerminationSigned) return 'RUPT. SIGNÉE';
    if (contract.isTerminationAccepted) return 'RUPT. ACCEPTÉE';
    if (active) return 'ACTIF';
    if (contract.startDate && new Date(contract.startDate) > new Date()) return 'À VENIR';
    if (contract.endDate && new Date(contract.endDate) < new Date()) return 'TERMINÉ';
    return contract.status?.toUpperCase() ?? '—';
  });

  const accent = $derived.by(() => {
    if (isCancelled) return 'gray';
    if (contract.isTerminationOccurred) return 'red';
    if (contract.isTerminated) return 'orange';
    if (contract.isTerminationSigned || contract.isTerminationAccepted) return 'yellow';
    if (active) return 'green';
    if (contract.startDate && new Date(contract.startDate) > new Date()) return 'blue';
    return 'gray';
  });

  const accentVar = $derived(`var(--${accent})`);
  const totalHours = $derived((contract.numberOfHoursInElearning ?? 0) + (contract.numberOfHoursInVirtualClassroom ?? 0));
</script>

<div class="card-accent" class:dimmed={isCancelled}>
  <div class="accent-bar" style="background: {accentVar};"></div>
  <div class="accent-body">
    <div class="flex items-baseline gap-6">
      <PillBadge text={primaryLabel} tint={accent} strong />
      {#if contract.status && contract.status.toUpperCase() !== primaryLabel}
        <PillBadge text={contract.status} tint="gray" />
      {/if}
      <div class="flex-1"></div>
      <Chip text={'…' + contract.id.slice(-6)} Icon={Hash} tint="gray" clipboardText={contract.id} copyable mono />
    </div>
    {#if contract.startDate || contract.endDate}
      <div class="flex items-center gap-8 text-sm">
        {#if contract.startDate}
          <span class="flex items-center gap-4"><Calendar size={11} color={accentVar} />{longDate(contract.startDate)}</span>
        {/if}
        {#if contract.startDate && contract.endDate}
          <ArrowRight size={11} class="text-tertiary" />
        {/if}
        {#if contract.endDate}
          <span class="flex items-center gap-4"><CalendarCheck size={11} color={accentVar} />{longDate(contract.endDate)}</span>
        {/if}
        {#if contract.startDate && contract.endDate}
          <span class="text-2xs text-tertiary">{durationString(contract.startDate, contract.endDate)}</span>
        {/if}
      </div>
    {/if}
    {#if contract.contractAmount || totalHours || contract.classId}
      <div class="flex items-center gap-12 text-sm text-secondary">
        {#if contract.contractAmount}<span class="flex items-center gap-4"><Euro size={11} />{currency(contract.contractAmount)}</span>{/if}
        {#if totalHours > 0}<span class="flex items-center gap-4"><Clock size={11} />{Math.round(totalHours)} h</span>{/if}
        {#if contract.classId}<span class="flex items-center gap-4"><Users size={11} />{contract.classId}</span>{/if}
      </div>
    {/if}
    {#if contract.terminationDate}
      <div class="flex items-center gap-4 text-xs" style="color: var(--orange);">
        <OctagonAlert size={12} />
        <span class="fw-semibold">Rompu le {longDate(contract.terminationDate)}</span>
        {#if contract.terminationReason}<span style="opacity: 0.8;">· {contract.terminationReason}</span>{/if}
      </div>
    {/if}
  </div>
</div>
