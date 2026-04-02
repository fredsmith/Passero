<script lang="ts">
  import { passwords } from "$lib/stores/passwords.svelte";
  import { copyPassword, deletePassword } from "$lib/commands";
  import { ui } from "$lib/stores/ui.svelte";
  import TotpDisplay from "./TotpDisplay.svelte";
  import TotpImport from "./TotpImport.svelte";

  let showPassword = $state(false);
  let showTotpImport = $state(false);

  let otpauthUri = $derived(passwords.parsedContent?.otpauthUri ?? null);

  async function handleCopy() {
    if (!passwords.selectedPath) return;
    try {
      await copyPassword(passwords.selectedPath);
      ui.notify("Copied to clipboard");
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handleDelete() {
    if (!passwords.selectedPath) return;
    if (!confirm(`Delete ${passwords.selectedPath}?`)) return;
    try {
      await deletePassword(passwords.selectedPath);
      passwords.deselect();
      await passwords.refresh();
      ui.notify("Deleted");
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }
</script>

{#if passwords.selectedPath && passwords.parsedContent}
  <div class="flex-1 p-6 overflow-y-auto">
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-lg font-medium">{passwords.selectedPath}</h2>
      <div class="flex gap-2">
        <button
          class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
          onclick={handleCopy}
        >
          Copy
        </button>
        <button
          class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
          onclick={() => (ui.showEditor = true)}
        >
          Edit
        </button>
        <button
          class="px-3 py-1.5 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
          onclick={() => (showTotpImport = true)}
          title="Add TOTP secret"
        >
          + TOTP
        </button>
        <button
          class="px-3 py-1.5 text-sm bg-red-900/50 hover:bg-red-900 text-red-200 rounded transition-colors"
          onclick={handleDelete}
        >
          Delete
        </button>
      </div>
    </div>

    <div class="space-y-4">
      <div>
        <label class="text-xs text-zinc-500 uppercase tracking-wide">Password</label>
        <div class="mt-1 flex items-center gap-2">
          <code class="flex-1 bg-zinc-800 rounded px-3 py-2 text-sm font-mono">
            {#if showPassword}
              {passwords.parsedContent.password}
            {:else}
              ••••••••••••
            {/if}
          </code>
          <button
            class="px-3 py-2 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
            onclick={() => (showPassword = !showPassword)}
          >
            {showPassword ? "Hide" : "Show"}
          </button>
        </div>
      </div>

      {#if otpauthUri}
        <TotpDisplay uri={otpauthUri} />
      {/if}

      {#if Object.keys(passwords.parsedContent.metadata).length > 0}
        <div class="space-y-2">
          {#each Object.entries(passwords.parsedContent.metadata) as [key, value]}
            <div>
              <label class="text-xs text-zinc-500 uppercase tracking-wide">{key}</label>
              <pre class="mt-1 bg-zinc-800 rounded px-3 py-2 text-sm whitespace-pre-wrap">{value}</pre>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  {#if showTotpImport && passwords.selectedPath}
    <TotpImport
      path={passwords.selectedPath}
      onclose={() => {
        showTotpImport = false;
        if (passwords.selectedPath) {
          passwords.select(passwords.selectedPath);
        }
      }}
    />
  {/if}
{:else if passwords.selectedPath}
  <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
    Loading...
  </div>
{:else}
  <div class="flex-1 flex items-center justify-center text-zinc-500 text-sm">
    Select a password to view
  </div>
{/if}
