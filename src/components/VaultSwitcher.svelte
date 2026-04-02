<script lang="ts">
  import { settings } from "$lib/stores/settings.svelte";
  import { passwords } from "$lib/stores/passwords.svelte";
  import { setActiveVault, addVault, removeVault } from "$lib/commands";
  import { ui } from "$lib/stores/ui.svelte";
  import type { Vault } from "$lib/types";

  let showAddForm = $state(false);
  let newName = $state("");
  let newPath = $state("");

  async function switchVault(id: string | null) {
    try {
      await setActiveVault(id);
      await settings.load();
      passwords.deselect();
      await passwords.refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handleAdd(e: Event) {
    e.preventDefault();
    if (!newName.trim() || !newPath.trim()) return;
    try {
      const vault = await addVault(newName.trim(), newPath.trim());
      await settings.load();
      await switchVault(vault.id);
      newName = "";
      newPath = "";
      showAddForm = false;
      ui.notify(`Vault "${vault.name}" added`);
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handleRemove(vault: Vault) {
    if (!confirm(`Remove vault "${vault.name}"? (This does not delete the password store)`)) return;
    try {
      await removeVault(vault.id);
      await settings.load();
      await passwords.refresh();
      ui.notify(`Vault "${vault.name}" removed`);
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  let vaults = $derived(settings.config.vaults);
  let activeId = $derived(settings.config.active_vault_id);
</script>

<div class="space-y-1">
  <div class="px-3 py-1 text-xs text-zinc-500 uppercase tracking-wide flex items-center justify-between">
    <span>Vaults</span>
    <button
      class="text-zinc-500 hover:text-zinc-300 transition-colors"
      onclick={() => (showAddForm = !showAddForm)}
      title="Add vault"
    >
      {showAddForm ? "−" : "+"}
    </button>
  </div>

  <!-- Default vault (no vault selected) -->
  <button
    class="w-full text-left px-3 py-1.5 rounded text-sm transition-colors {!activeId || vaults.length === 0
      ? 'bg-zinc-800 text-white'
      : 'text-zinc-400 hover:text-white hover:bg-zinc-800/50'}"
    onclick={() => switchVault(null)}
  >
    Default
  </button>

  {#each vaults as vault}
    <div class="flex items-center group">
      <button
        class="flex-1 text-left px-3 py-1.5 rounded text-sm transition-colors truncate {activeId === vault.id
          ? 'bg-zinc-800 text-white'
          : 'text-zinc-400 hover:text-white hover:bg-zinc-800/50'}"
        onclick={() => switchVault(vault.id)}
        title={vault.path}
      >
        {vault.name}
      </button>
      <button
        class="px-1 text-xs text-zinc-600 hover:text-red-400 opacity-0 group-hover:opacity-100 transition-opacity"
        onclick={() => handleRemove(vault)}
        title="Remove vault"
      >
        ×
      </button>
    </div>
  {/each}

  {#if showAddForm}
    <form onsubmit={handleAdd} class="px-2 space-y-2 pt-1">
      <input
        type="text"
        bind:value={newName}
        placeholder="Vault name"
        class="w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs focus:outline-none focus:border-zinc-500"
        required
      />
      <input
        type="text"
        bind:value={newPath}
        placeholder="Path to store"
        class="w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs font-mono focus:outline-none focus:border-zinc-500"
        required
      />
      <button
        type="submit"
        disabled={!newName.trim() || !newPath.trim()}
        class="w-full px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded transition-colors"
      >
        Add Vault
      </button>
    </form>
  {/if}
</div>
