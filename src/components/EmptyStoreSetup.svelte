<script lang="ts">
  import { listGpgSecretKeys, initPasswordStore } from "$lib/commands";
  import { passwords } from "$lib/stores/passwords.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import type { GpgKey } from "$lib/types";

  let started = $state(false);
  let busy = $state(false);
  let secretKeys = $state<GpgKey[]>([]);
  let selectedKeyId = $state<string | null>(null);
  let keysLoading = $state(false);

  async function start() {
    started = true;
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

  async function handleInit() {
    if (!selectedKeyId) return;
    busy = true;
    try {
      await initPasswordStore([selectedKeyId]);
      ui.notify("Password store initialized");
      await passwords.refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      busy = false;
    }
  }
</script>

<div class="p-4 space-y-3">
  {#if !started}
    <p class="text-zinc-500 text-sm">This password store is empty.</p>
    <button
      class="w-full text-left px-4 py-3 bg-zinc-800/50 border border-zinc-700 hover:border-zinc-500 rounded-lg transition-colors"
      onclick={start}
    >
      <div class="text-sm font-medium">Initialize store</div>
      <div class="text-xs text-zinc-500 mt-0.5">Set up this directory as a new password store with a GPG key</div>
    </button>
  {:else}
    <h3 class="text-sm font-medium">Select a GPG key</h3>
    {#if keysLoading}
      <div class="text-zinc-500 text-sm">Loading keys...</div>
    {:else if secretKeys.length === 0}
      <div class="text-zinc-500 text-sm">
        No secret keys found. Generate or import a GPG key first (GPG tab in the sidebar).
      </div>
    {:else}
      <div class="space-y-1">
        {#each secretKeys as key}
          <button
            class="w-full text-left px-3 py-2 rounded transition-colors {selectedKeyId === key.id
              ? 'bg-zinc-700 border border-zinc-500'
              : 'bg-zinc-800/50 border border-zinc-700 hover:border-zinc-500'}"
            onclick={() => (selectedKeyId = key.id)}
          >
            <div class="text-sm truncate">{key.uid}</div>
            <div class="text-xs text-zinc-500 font-mono">{key.id}</div>
          </button>
        {/each}
      </div>
      <button
        disabled={!selectedKeyId || busy}
        class="w-full px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 disabled:opacity-50 rounded transition-colors"
        onclick={handleInit}
      >
        {busy ? "Initializing..." : "Initialize Store"}
      </button>
    {/if}
  {/if}
</div>
