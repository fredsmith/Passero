<script lang="ts">
  import { passwords } from "$lib/stores/passwords.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let { onNewEntry }: { onNewEntry: () => void } = $props();
</script>

<div class="flex items-center justify-between px-4 py-2 pt-8 border-b border-zinc-800" data-tauri-drag-region>
  <div class="flex items-center gap-2 no-drag">
    <button
      class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
      onclick={() => passwords.refresh()}
    >
      Refresh
    </button>
    <button
      class="px-3 py-1.5 text-sm bg-zinc-100 text-zinc-900 hover:bg-white rounded transition-colors"
      onclick={onNewEntry}
    >
      New
    </button>
  </div>
</div>
