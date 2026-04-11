<script lang="ts">
  import { listRecipients, addRecipient, removeRecipient } from "$lib/commands";
  import { passwords } from "$lib/stores/passwords.svelte";
  import { settings } from "$lib/stores/settings.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import { onMount } from "svelte";

  let recipients = $state<string[]>([]);
  let expanded = $state(false);
  let addId = $state("");
  let busy = $state(false);

  // Reload recipients when the active vault changes
  let activeId = $derived(settings.config.active_vault_id);
  $effect(() => {
    // Subscribe to activeId changes
    void activeId;
    refresh();
  });

  async function refresh() {
    try {
      recipients = await listRecipients();
    } catch {
      recipients = [];
    }
  }

  async function handleAdd(e: Event) {
    e.preventDefault();
    if (!addId.trim()) return;
    busy = true;
    try {
      recipients = await addRecipient(addId.trim());
      addId = "";
      ui.notify("Recipient added, store re-encrypted");
      await passwords.refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      busy = false;
    }
  }

  async function handleRemove(gpgId: string) {
    if (!confirm(`Remove ${gpgId}? The store will be re-encrypted.`)) return;
    busy = true;
    try {
      recipients = await removeRecipient(gpgId);
      ui.notify("Recipient removed, store re-encrypted");
      await passwords.refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      busy = false;
    }
  }
</script>

{#if recipients.length > 0}
  <div class="space-y-1">
    <button
      class="w-full px-3 py-1 text-xs text-zinc-500 uppercase tracking-wide flex items-center justify-between hover:text-zinc-300 transition-colors"
      onclick={() => (expanded = !expanded)}
    >
      <span>Recipients ({recipients.length})</span>
      <span class="text-[10px]">{expanded ? "▲" : "▼"}</span>
    </button>

    {#if expanded}
      <div class="space-y-1">
        {#each recipients as gpgId}
          <div class="flex items-center group px-1">
            <code class="flex-1 text-xs text-zinc-400 truncate" title={gpgId}>{gpgId}</code>
            <button
              class="px-1 text-[10px] text-zinc-600 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity"
              onclick={() => handleRemove(gpgId)}
              disabled={recipients.length <= 1 || busy}
              title={recipients.length <= 1 ? "Cannot remove the last recipient" : "Remove"}
            >
              ×
            </button>
          </div>
        {/each}
        <form onsubmit={handleAdd} class="flex gap-1 px-1">
          <input
            type="text"
            bind:value={addId}
            placeholder="Add key ID..."
            class="flex-1 min-w-0 bg-zinc-800 border border-zinc-700 rounded px-1.5 py-0.5 text-[11px] focus:outline-none focus:border-zinc-500"
          />
          <button
            type="submit"
            disabled={!addId.trim() || busy}
            class="px-1.5 py-0.5 text-[11px] bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded transition-colors"
          >
            {busy ? "..." : "+"}
          </button>
        </form>
      </div>
    {/if}
  </div>
{/if}
