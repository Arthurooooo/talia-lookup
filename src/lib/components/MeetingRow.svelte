<script lang="ts">
  import PillBadge from './PillBadge.svelte';
  import type { BubbleMeeting } from '$lib/types';
  import { longDate } from '$lib/format';

  let { group }: { group: { type: string; latest: BubbleMeeting; count: number } } = $props();

  const isPast = $derived(new Date(group.latest.startDate ?? '').getTime() < Date.now());

  const typeColor = $derived.by(() => {
    const t = group.type.toLowerCase();
    if (t.includes('r1')) return 'blue';
    if (t.includes('r2')) return 'purple';
    if (t.includes('r3')) return 'pink';
    if (t.includes('demo')) return 'orange';
    return 'gray';
  });

  const statusColor = $derived.by(() => {
    const s = (group.latest.meetingStatus ?? '').toLowerCase();
    if (s.includes('no') || s.includes('absent')) return 'red';
    if (s.includes('show') || s.includes('done') || s.includes('complet')) return 'green';
    if (s.includes('cancel') || s.includes('annul')) return 'gray';
    if (s.includes('schedul') || s.includes('prév')) return 'blue';
    return 'gray';
  });
</script>

<div class="flex items-center gap-8">
  <PillBadge text={group.type} tint={typeColor} strong />
  {#if group.latest.startDate}
    <span class="text-sm fw-semibold" class:text-secondary={isPast}>{longDate(group.latest.startDate)}</span>
  {/if}
  <div class="flex-1"></div>
  {#if group.latest.meetingStatus}
    <PillBadge text={group.latest.meetingStatus} tint={statusColor} />
  {:else if isPast}
    <PillBadge text="PASSÉ" tint="gray" />
  {:else}
    <PillBadge text="PRÉVU" tint="blue" />
  {/if}
  {#if group.count > 1}
    <span class="text-2xs fw-bold text-tertiary">×{group.count}</span>
  {/if}
</div>
