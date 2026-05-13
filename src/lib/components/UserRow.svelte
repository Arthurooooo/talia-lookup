<script lang="ts">
  import { Mail, Phone, ChevronRight } from '@lucide/svelte';
  import Avatar from './Avatar.svelte';
  import PillBadge from './PillBadge.svelte';
  import type { BubbleUser } from '$lib/types';
  import { fullName, bestEmail, initials } from '$lib/types';

  let { user, onSelect }: { user: BubbleUser; onSelect: (u: BubbleUser) => void } = $props();
</script>

<button type="button" class="row-card" onclick={() => onSelect(user)}>
  <Avatar url={user.profilePicture} initials={initials(user)} size={32} />
  <div class="flex-1" style="overflow: hidden;">
    <div class="flex items-center gap-6">
      <span class="fw-semibold">{fullName(user)}</span>
      {#if user.studentStatus}<PillBadge text={user.studentStatus} tint="blue" />
      {:else if user.leadStatus}<PillBadge text={user.leadStatus} tint="gray" />{/if}
    </div>
    <div class="flex items-center gap-8 text-2xs text-secondary truncate" style="margin-top: 2px;">
      {#if bestEmail(user)}<span class="flex items-center gap-4"><Mail size={10} />{bestEmail(user)}</span>{/if}
      {#if user.phoneNumber}<span class="flex items-center gap-4"><Phone size={10} />{user.phoneNumber}</span>{/if}
    </div>
  </div>
  <ChevronRight size={12} class="chevron" />
</button>
