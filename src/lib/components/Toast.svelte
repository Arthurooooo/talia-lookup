<script lang="ts">
  import { toast } from '$lib/toast';
  import { CheckCircle2 } from '@lucide/svelte';
  function trunc(t: string) { return t.length > 32 ? t.slice(0, 28) + '…' : t; }
</script>

{#if $toast}
  <div class="toast-wrap">
    <div class="toast">
      <CheckCircle2 class="check" />
      <span class="fw-semibold">Copié</span>
      <span class="mono text-2xs text-secondary truncate" style="max-width: 240px;">{trunc($toast.text)}</span>
    </div>
  </div>
{/if}
