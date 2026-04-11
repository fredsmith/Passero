<script lang="ts">
  import {
    listGpgKeys,
    listGpgSecretKeys,
    generateGpgKey,
    importGpgKey,
    importGpgKeyFromKeyserver,
    exportGpgKey,
    publishGpgKey,
    setGpgKeyTrust,
    deleteGpgKey,
  } from "$lib/commands";
  import type { GpgKey } from "$lib/types";
  import { ui } from "$lib/stores/ui.svelte";
  import { onMount } from "svelte";

  let publicKeys = $state<GpgKey[]>([]);
  let secretKeys = $state<GpgKey[]>([]);
  let loading = $state(true);

  // Generate key form
  let showGenerate = $state(false);
  let genName = $state("");
  let genEmail = $state("");
  let genPassphrase = $state("");
  let generating = $state(false);

  // Import form
  let showImport = $state(false);
  let importMode = $state<"paste" | "keyserver">("paste");
  let importKeyData = $state("");
  let importKeyId = $state("");
  let importKeyserver = $state("hkps://keys.openpgp.org");
  let importing = $state(false);

  // Trust levels
  const trustLabels: Record<string, string> = {
    o: "Unknown",
    n: "Never",
    m: "Marginal",
    f: "Full",
    u: "Ultimate",
    "-": "Unknown",
    q: "Undefined",
    e: "Expired",
    r: "Revoked",
  };

  async function refresh() {
    loading = true;
    try {
      [publicKeys, secretKeys] = await Promise.all([
        listGpgKeys(),
        listGpgSecretKeys(),
      ]);
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      loading = false;
    }
  }

  onMount(refresh);

  function isSecretKey(key: GpgKey): boolean {
    return secretKeys.some((sk) => sk.id === key.id);
  }

  async function handleGenerate(e: Event) {
    e.preventDefault();
    generating = true;
    try {
      const fingerprint = await generateGpgKey({
        name: genName,
        email: genEmail,
        passphrase: genPassphrase || undefined,
      });
      ui.notify(`Key generated: ${fingerprint.slice(-8)}`);
      genName = "";
      genEmail = "";
      genPassphrase = "";
      showGenerate = false;
      await refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      generating = false;
    }
  }

  async function handleImport(e: Event) {
    e.preventDefault();
    importing = true;
    try {
      let result: string;
      if (importMode === "paste") {
        result = await importGpgKey(importKeyData);
      } else {
        result = await importGpgKeyFromKeyserver(
          importKeyId,
          importKeyserver || undefined,
        );
      }
      ui.notify("Key imported successfully");
      importKeyData = "";
      importKeyId = "";
      showImport = false;
      await refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      importing = false;
    }
  }

  async function handleExport(key: GpgKey) {
    try {
      const armored = await exportGpgKey(key.id, false);
      await navigator.clipboard.writeText(armored);
      ui.notify("Public key copied to clipboard");
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handlePublish(key: GpgKey) {
    try {
      await publishGpgKey(key.id);
      ui.notify("Key published to keyserver");
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handleSetTrust(key: GpgKey, level: number) {
    if (!key.fingerprint) {
      ui.notify("No fingerprint available", "error");
      return;
    }
    try {
      await setGpgKeyTrust(key.fingerprint, level);
      ui.notify("Trust level updated");
      await refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handleDelete(key: GpgKey) {
    const hasSecret = isSecretKey(key);
    const msg = hasSecret
      ? `Delete key ${key.uid} (including secret key)? This cannot be undone.`
      : `Delete public key ${key.uid}?`;
    if (!confirm(msg)) return;
    try {
      await deleteGpgKey(key.fingerprint || key.id, hasSecret);
      ui.notify("Key deleted");
      await refresh();
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }
</script>

<div class="h-8 w-full shrink-0" data-tauri-drag-region></div>
<div class="p-6 max-w-3xl overflow-y-auto flex-1">
  <div class="flex items-center justify-between mb-6">
    <h2 class="text-lg font-medium">GPG Keys</h2>
    <div class="flex gap-2">
      <button
        class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
        onclick={() => { showGenerate = !showGenerate; showImport = false; }}
      >
        Generate
      </button>
      <button
        class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
        onclick={() => { showImport = !showImport; showGenerate = false; }}
      >
        Import
      </button>
      <button
        class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
        onclick={refresh}
      >
        Refresh
      </button>
    </div>
  </div>

  {#if showGenerate}
    <div class="mb-6 p-4 bg-zinc-800/50 border border-zinc-700 rounded-lg">
      <h3 class="text-sm font-medium mb-3">Generate New Key Pair</h3>
      <form onsubmit={handleGenerate} class="space-y-3">
        <div>
          <label for="gen-name" class="text-xs text-zinc-500">Name</label>
          <input
            id="gen-name"
            type="text"
            bind:value={genName}
            placeholder="John Doe"
            required
            class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
          />
        </div>
        <div>
          <label for="gen-email" class="text-xs text-zinc-500">Email</label>
          <input
            id="gen-email"
            type="email"
            bind:value={genEmail}
            placeholder="john@example.com"
            required
            class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
          />
        </div>
        <div>
          <label for="gen-pass" class="text-xs text-zinc-500">Passphrase (optional)</label>
          <input
            id="gen-pass"
            type="password"
            bind:value={genPassphrase}
            placeholder="Leave empty for no passphrase"
            class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
          />
        </div>
        <div class="flex gap-2">
          <button
            type="submit"
            disabled={generating}
            class="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 disabled:opacity-50 rounded transition-colors"
          >
            {generating ? "Generating..." : "Generate Key"}
          </button>
          <button
            type="button"
            class="px-4 py-2 text-sm bg-zinc-700 hover:bg-zinc-600 rounded transition-colors"
            onclick={() => (showGenerate = false)}
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  {/if}

  {#if showImport}
    <div class="mb-6 p-4 bg-zinc-800/50 border border-zinc-700 rounded-lg">
      <h3 class="text-sm font-medium mb-3">Import Key</h3>
      <div class="flex gap-2 mb-3">
        <button
          class="px-3 py-1 text-xs rounded transition-colors {importMode === 'paste'
            ? 'bg-zinc-600 text-white'
            : 'bg-zinc-800 text-zinc-400 hover:text-white'}"
          onclick={() => (importMode = "paste")}
        >
          Paste Key
        </button>
        <button
          class="px-3 py-1 text-xs rounded transition-colors {importMode === 'keyserver'
            ? 'bg-zinc-600 text-white'
            : 'bg-zinc-800 text-zinc-400 hover:text-white'}"
          onclick={() => (importMode = "keyserver")}
        >
          Keyserver
        </button>
      </div>
      <form onsubmit={handleImport} class="space-y-3">
        {#if importMode === "paste"}
          <div>
            <label for="import-key" class="text-xs text-zinc-500">Public Key (ASCII-armored)</label>
            <textarea
              id="import-key"
              bind:value={importKeyData}
              placeholder="-----BEGIN PGP PUBLIC KEY BLOCK-----"
              required
              rows={6}
              class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm font-mono focus:outline-none focus:border-zinc-500 resize-y"
            ></textarea>
          </div>
        {:else}
          <div>
            <label for="import-id" class="text-xs text-zinc-500">Key ID or Email</label>
            <input
              id="import-id"
              type="text"
              bind:value={importKeyId}
              placeholder="0xABCDEF01 or user@example.com"
              required
              class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
            />
          </div>
          <div>
            <label for="import-server" class="text-xs text-zinc-500">Keyserver</label>
            <input
              id="import-server"
              type="text"
              bind:value={importKeyserver}
              class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
            />
          </div>
        {/if}
        <div class="flex gap-2">
          <button
            type="submit"
            disabled={importing}
            class="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 disabled:opacity-50 rounded transition-colors"
          >
            {importing ? "Importing..." : "Import Key"}
          </button>
          <button
            type="button"
            class="px-4 py-2 text-sm bg-zinc-700 hover:bg-zinc-600 rounded transition-colors"
            onclick={() => (showImport = false)}
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  {/if}

  {#if loading}
    <div class="text-zinc-500 text-sm">Loading keys...</div>
  {:else if publicKeys.length === 0}
    <div class="text-zinc-500 text-sm">No GPG keys found. Generate or import one to get started.</div>
  {:else}
    <div class="space-y-2">
      {#each publicKeys as key}
        <div class="bg-zinc-800/50 border border-zinc-700 rounded-lg p-4">
          <div class="flex items-start justify-between">
            <div class="flex-1 min-w-0">
              <div class="text-sm font-medium truncate">{key.uid}</div>
              <div class="text-xs text-zinc-500 font-mono mt-0.5">{key.id}</div>
              {#if key.fingerprint}
                <div class="text-xs text-zinc-600 font-mono mt-0.5 truncate">{key.fingerprint}</div>
              {/if}
              <div class="flex items-center gap-3 mt-2 text-xs">
                <span class="text-zinc-500">
                  Trust: {trustLabels[key.trust] || key.trust}
                </span>
                {#if isSecretKey(key)}
                  <span class="text-green-500">Secret key available</span>
                {/if}
              </div>
            </div>
            <div class="flex gap-1 ml-3">
              <button
                class="px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 rounded transition-colors"
                onclick={() => handleExport(key)}
                title="Copy public key"
              >
                Export
              </button>
              <button
                class="px-2 py-1 text-xs bg-zinc-700 hover:bg-zinc-600 rounded transition-colors"
                onclick={() => handlePublish(key)}
                title="Publish to keyserver"
              >
                Publish
              </button>
              <select
                class="px-2 py-1 text-xs bg-zinc-700 rounded appearance-none cursor-pointer"
                onchange={(e) => {
                  const val = parseInt((e.target as HTMLSelectElement).value);
                  if (val) handleSetTrust(key, val);
                  (e.target as HTMLSelectElement).value = "0";
                }}
              >
                <option value="0">Trust...</option>
                <option value="2">Never</option>
                <option value="3">Marginal</option>
                <option value="4">Full</option>
                <option value="5">Ultimate</option>
              </select>
              <button
                class="px-2 py-1 text-xs bg-red-900/50 hover:bg-red-900 text-red-200 rounded transition-colors"
                onclick={() => handleDelete(key)}
                title="Delete key"
              >
                Delete
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
