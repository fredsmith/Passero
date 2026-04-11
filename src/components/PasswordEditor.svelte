<script lang="ts">
  import { passwords } from "$lib/stores/passwords.svelte";
  import { insertPassword } from "$lib/commands";
  import { ui } from "$lib/stores/ui.svelte";

  let {
    editPath = null,
    onclose,
  }: { editPath: string | null; onclose: () => void } = $props();

  let path = $state(editPath ?? "");
  let content = $state(editPath ? (passwords.selectedContent ?? "") : "");
  let saving = $state(false);

  // Generator
  let genLength = $state(24);
  let genSymbols = $state(true);
  let generating = $state(false);

  function randomPassword(length: number, symbols: boolean): string {
    const alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const chars = symbols ? alpha + "!@#$%^&*()-_=+[]{}|;:,.<>?" : alpha;
    const array = new Uint8Array(length);
    crypto.getRandomValues(array);
    return Array.from(array, (b) => chars[b % chars.length]).join("");
  }

  function handleGenerate() {
    const pw = randomPassword(genLength, genSymbols);
    // Replace first line, keep the rest (metadata lines)
    const lines = content.split("\n");
    if (lines.length > 1 && lines.slice(1).some((l) => l.trim())) {
      lines[0] = pw;
      content = lines.join("\n");
    } else {
      content = pw;
    }
  }

  async function handleSave() {
    if (!path.trim()) return;
    saving = true;
    try {
      await insertPassword(path, content);
      await passwords.refresh();
      if (editPath) {
        await passwords.select(path);
      }
      ui.notify(editPath ? "Updated" : "Created");
      onclose();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      saving = false;
    }
  }
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-zinc-900 border border-zinc-700 rounded-lg w-full max-w-md p-6 space-y-4">
    <h3 class="text-lg font-medium">{editPath ? "Edit" : "New"} Password</h3>

    <div>
      <label class="text-xs text-zinc-500 uppercase tracking-wide" for="entry-path">Path</label>
      <input
        id="entry-path"
        type="text"
        bind:value={path}
        placeholder="folder/entry-name"
        disabled={!!editPath}
        class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500 disabled:opacity-50"
      />
    </div>

    <div>
      <div class="flex items-center justify-between">
        <label class="text-xs text-zinc-500 uppercase tracking-wide" for="entry-content">Content</label>
        <div class="flex items-center gap-2">
          <label class="flex items-center gap-1 text-[11px] text-zinc-500">
            <input type="number" bind:value={genLength} min={8} max={128}
              class="w-12 bg-zinc-800 border border-zinc-700 rounded px-1 py-0.5 text-[11px] text-zinc-300 focus:outline-none focus:border-zinc-500"
            />
            chars
          </label>
          <label class="flex items-center gap-1 text-[11px] text-zinc-500">
            <input type="checkbox" bind:checked={genSymbols} class="rounded" />
            symbols
          </label>
          <button
            class="px-2 py-0.5 text-[11px] bg-zinc-700 hover:bg-zinc-600 rounded transition-colors"
            onclick={handleGenerate}
          >
            Generate
          </button>
        </div>
      </div>
      <textarea
        id="entry-content"
        bind:value={content}
        rows={8}
        placeholder={"password\nusername: user\nurl: https://example.com"}
        class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 font-mono placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500 resize-none"
      ></textarea>
    </div>

    <div class="flex justify-end gap-2">
      <button
        class="px-4 py-2 text-sm text-zinc-400 hover:text-white transition-colors"
        onclick={onclose}
      >
        Cancel
      </button>
      <button
        class="px-4 py-2 text-sm bg-zinc-100 text-zinc-900 rounded hover:bg-white transition-colors disabled:opacity-50"
        onclick={handleSave}
        disabled={saving || !path.trim()}
      >
        {saving ? "Saving..." : "Save"}
      </button>
    </div>
  </div>
</div>
