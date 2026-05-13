<script lang="ts">
  import { copyToClipboard } from '$lib/toast';
  import type { Component } from 'svelte';

  type Tint = 'blue' | 'green' | 'red' | 'orange' | 'yellow' | 'purple' | 'indigo' | 'pink' | 'teal' | 'gray';

  let {
    text,
    Icon,
    tint = 'gray',
    clipboardText,
    mono = false,
    copyable = false
  }: {
    text: string;
    Icon?: Component;
    tint?: Tint;
    clipboardText?: string;
    mono?: boolean;
    copyable?: boolean;
  } = $props();

  function handle() {
    if (copyable) copyToClipboard(clipboardText ?? text);
  }
</script>

{#if copyable}
  <button type="button" class="chip chip-{tint} copyable" class:mono onclick={handle} title="Copier : {clipboardText ?? text}">
    {#if Icon}<Icon size={11} />{/if}
    <span>{text}</span>
  </button>
{:else}
  <span class="chip chip-{tint}" class:mono>
    {#if Icon}<Icon size={11} />{/if}
    <span>{text}</span>
  </span>
{/if}
