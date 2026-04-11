<script lang="ts">
  import { settings } from "$lib/stores/settings.svelte";
  import { passwords } from "$lib/stores/passwords.svelte";
  import {
    setActiveVault,
    addVault,
    removeVault,
    initPasswordStore,
    gitClone,
    listGpgSecretKeys,
  } from "$lib/commands";
  import { ui } from "$lib/stores/ui.svelte";
  import type { Vault, GpgKey } from "$lib/types";

  type AddMode = null | "choose" | "init" | "clone";

  let addMode = $state<AddMode>(null);
  let busy = $state(false);

  // Shared fields
  let newName = $state("");
  let newPath = $state("");

  // Init-specific
  let secretKeys = $state<GpgKey[]>([]);
  let selectedKeyId = $state<string | null>(null);
  let keysLoading = $state(false);

  // Clone-specific
  let cloneUrl = $state("");

  function resetForm() {
    addMode = null;
    newName = "";
    newPath = "";
    selectedKeyId = null;
    cloneUrl = "";
    secretKeys = [];
  }

  async function openAddForm() {
    addMode = addMode ? null : "choose";
  }

  async function pickInit() {
    addMode = "init";
    keysLoading = true;
    try {
      secretKeys = await listGpgSecretKeys();
      if (secretKeys.length === 1) {
        selectedKeyId = secretKeys[0].id;
      }
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      keysLoading = false;
    }
  }

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

  async function handleInit(e: Event) {
    e.preventDefault();
    if (!newName.trim() || !newPath.trim() || !selectedKeyId) return;
    busy = true;
    try {
      const vault = await addVault(newName.trim(), newPath.trim());
      await settings.load();
      await switchVault(vault.id);
      await initPasswordStore([selectedKeyId]);
      await passwords.refresh();
      ui.notify(`Vault "${vault.name}" initialized`);
      resetForm();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      busy = false;
    }
  }

  async function handleClone(e: Event) {
    e.preventDefault();
    if (!newName.trim() || !newPath.trim() || !cloneUrl.trim()) return;
    busy = true;
    try {
      await gitClone(cloneUrl.trim(), newPath.trim());
      const vault = await addVault(newName.trim(), newPath.trim());
      await settings.load();
      await switchVault(vault.id);
      ui.notify(`Vault "${vault.name}" cloned`);
      resetForm();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      busy = false;
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
      onclick={openAddForm}
      title="Add vault"
    >
      {addMode ? "−" : "+"}
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

  {#if addMode === "choose"}
    <div class="px-2 pt-1 space-y-1">
      <button
        class="w-full text-left px-2 py-1.5 text-xs bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
        onclick={pickInit}
      >
        Initialize new store
      </button>
      <button
        class="w-full text-left px-2 py-1.5 text-xs bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
        onclick={() => (addMode = "clone")}
      >
        Clone from git
      </button>
    </div>
  {:else if addMode === "init"}
    <form onsubmit={handleInit} class="px-2 space-y-2 pt-1">
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
      {#if keysLoading}
        <div class="text-zinc-500 text-xs py-1">Loading keys...</div>
      {:else if secretKeys.length === 0}
        <div class="text-zinc-500 text-xs py-1">No GPG secret keys found.</div>
      {:else}
        <select
          bind:value={selectedKeyId}
          class="w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs focus:outline-none focus:border-zinc-500"
        >
          <option value={null} disabled selected>Select GPG key...</option>
          {#each secretKeys as key}
            <option value={key.id}>{key.uid} ({key.id})</option>
          {/each}
        </select>
      {/if}
      <div class="flex gap-1">
        <button
          type="submit"
          disabled={!newName.trim() || !newPath.trim() || !selectedKeyId || busy}
          class="flex-1 px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded transition-colors"
        >
          {busy ? "Initializing..." : "Initialize"}
        </button>
        <button
          type="button"
          class="px-2 py-1 text-xs bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
          onclick={() => (addMode = "choose")}
        >
          Back
        </button>
      </div>
    </form>
  {:else if addMode === "clone"}
    <form onsubmit={handleClone} class="px-2 space-y-2 pt-1">
      <input
        type="text"
        bind:value={newName}
        placeholder="Vault name"
        class="w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs focus:outline-none focus:border-zinc-500"
        required
      />
      <input
        type="text"
        bind:value={cloneUrl}
        placeholder="git@github.com:user/pass-store.git"
        class="w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs font-mono focus:outline-none focus:border-zinc-500"
        required
      />
      <input
        type="text"
        bind:value={newPath}
        placeholder="Clone to path"
        class="w-full bg-zinc-800 border border-zinc-700 rounded px-2 py-1 text-xs font-mono focus:outline-none focus:border-zinc-500"
        required
      />
      <div class="flex gap-1">
        <button
          type="submit"
          disabled={!newName.trim() || !newPath.trim() || !cloneUrl.trim() || busy}
          class="flex-1 px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 disabled:opacity-50 rounded transition-colors"
        >
          {busy ? "Cloning..." : "Clone"}
        </button>
        <button
          type="button"
          class="px-2 py-1 text-xs bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
          onclick={() => (addMode = "choose")}
        >
          Back
        </button>
      </div>
    </form>
  {/if}
</div>
