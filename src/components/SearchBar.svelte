<script lang="ts">
  import { ui } from "$lib/stores/ui.svelte";
</script>

<div class="px-4 py-3 border-b border-zinc-800">
  <input
    type="text"
    placeholder="Search passwords..."
    bind:value={ui.searchQuery}
    class="w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-1.5 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500"
  />
</div>
