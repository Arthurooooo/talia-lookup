<script lang="ts">
  type Tint = 'blue' | 'green' | 'red' | 'orange' | 'yellow' | 'purple' | 'indigo' | 'pink' | 'teal' | 'gray';
  let { text, tint = 'gray', strong = false }: { text: string; tint?: Tint; strong?: boolean } = $props();
</script>

<span class="pill pill-{tint}" class:strong>{text}</span>
