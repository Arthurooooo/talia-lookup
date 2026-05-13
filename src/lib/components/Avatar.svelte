<script lang="ts">
  let { url, initials, size = 36 }: { url?: string; initials: string; size?: number } = $props();
  function fixUrl(u?: string) {
    if (!u) return undefined;
    if (u.startsWith('//')) return 'https:' + u;
    return u;
  }
  let safeUrl = $derived(fixUrl(url));
</script>

<div class="avatar" style="width: {size}px; height: {size}px; font-size: {Math.round(size * 0.4)}px;">
  {#if safeUrl}
    <img src={safeUrl} alt="" onerror={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }} />
  {:else}
    {initials}
  {/if}
</div>
