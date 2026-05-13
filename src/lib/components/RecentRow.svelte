<script lang="ts">
  import { ChevronRight, X } from '@lucide/svelte';
  import Avatar from './Avatar.svelte';
  import type { RecentUserStub } from '$lib/types';
  import { initialsFromName } from '$lib/types';
  import { relativeDate } from '$lib/format';

  let { stub, onTap, onRemove }: {
    stub: RecentUserStub;
    onTap: (s: RecentUserStub) => void;
    onRemove: (s: RecentUserStub) => void;
  } = $props();
</script>

<button type="button" class="recents-row" onclick={() => onTap(stub)}>
  <Avatar url={stub.avatarUrl} initials={initialsFromName(stub.fullName)} size={26} />
  <div class="flex-1" style="overflow: hidden;">
    <div class="text-sm fw-semibold">{stub.fullName}</div>
    <div class="flex items-center gap-6 text-2xs text-tertiary truncate">
      {#if stub.subtitle}<span class="text-secondary">{stub.subtitle}</span>{/if}
      <span>{relativeDate(stub.visitedAt)}</span>
    </div>
  </div>
  <span class="x-btn icon-btn" onclick={(e) => { e.stopPropagation(); onRemove(stub); }} role="button" tabindex="0" title="Retirer">
    <X size={14} />
  </span>
  <ChevronRight size={12} class="chevron" />
</button>
