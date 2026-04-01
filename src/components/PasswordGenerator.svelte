<script lang="ts">
  import { generatePassword } from "$lib/commands";
  import { passwords } from "$lib/stores/passwords.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  let path = $state("");
  let length = $state(24);
  let symbols = $state(true);
  let generating = $state(false);
  let result = $state<string | null>(null);

  async function handleGenerate() {
    if (!path.trim()) return;
    generating = true;
    result = null;
    try {
      const content = await generatePassword(path, length, symbols);
      result = content.split("\n")[0] ?? "";
      await passwords.refresh();
      ui.notify(`Generated password for ${path}`);
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      generating = false;
    }
  }
</script>

<div class="space-y-4">
  <div>
    <label class="text-xs text-zinc-500 uppercase tracking-wide" for="gen-path">Path</label>
    <input
      id="gen-path"
      type="text"
      bind:value={path}
      placeholder="folder/entry-name"
      class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500"
    />
  </div>

  <div class="flex gap-4">
    <div class="flex-1">
      <label class="text-xs text-zinc-500 uppercase tracking-wide" for="gen-length">Length</label>
      <input
        id="gen-length"
        type="number"
        bind:value={length}
        min={8}
        max={128}
        class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 focus:outline-none focus:border-zinc-500"
      />
    </div>
    <div class="flex items-end">
      <label class="flex items-center gap-2 text-sm text-zinc-300 pb-2">
        <input type="checkbox" bind:checked={symbols} class="rounded" />
        Symbols
      </label>
    </div>
  </div>

  <button
    class="px-4 py-2 text-sm bg-zinc-100 text-zinc-900 rounded hover:bg-white transition-colors disabled:opacity-50"
    onclick={handleGenerate}
    disabled={generating || !path.trim()}
  >
    {generating ? "Generating..." : "Generate"}
  </button>

  {#if result}
    <div>
      <label class="text-xs text-zinc-500 uppercase tracking-wide">Generated</label>
      <code class="mt-1 block bg-zinc-800 rounded px-3 py-2 text-sm font-mono break-all">
        {result}
      </code>
    </div>
  {/if}
</div>
