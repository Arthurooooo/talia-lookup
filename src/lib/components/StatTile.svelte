<script lang="ts">
  import type { Component } from 'svelte';
  type Tint = 'blue' | 'green' | 'red' | 'orange' | 'yellow' | 'purple' | 'indigo' | 'pink' | 'teal' | 'gray';

  let {
    label,
    value,
    Icon,
    tint = 'gray'
  }: { label: string; value: string; Icon: Component; tint?: Tint } = $props();

  let bgVar = $derived(`var(--${tint})`);
</script>

<div class="stat" style="border-color: color-mix(in srgb, {bgVar} 22%, transparent); background: color-mix(in srgb, {bgVar} 10%, transparent);">
  <div class="stat-icon" style="color: {bgVar};"><Icon size={16} /></div>
  <div>
    <div class="stat-label">{label}</div>
    <div class="stat-value">{value}</div>
  </div>
</div>
