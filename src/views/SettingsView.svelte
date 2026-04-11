<script lang="ts">
  import { settings } from "$lib/stores/settings.svelte";
  import { ui } from "$lib/stores/ui.svelte";

  async function handleSave() {
    await settings.save();
    ui.notify("Settings saved");
  }
</script>

<div class="h-8 w-full shrink-0" data-tauri-drag-region></div>
<div class="p-6 max-w-2xl overflow-y-auto">
  <h2 class="text-lg font-medium mb-6">Settings</h2>

  {#if settings.error}
    <div class="text-red-400 text-sm mb-4">{settings.error}</div>
  {/if}

  <div class="space-y-6">
    <section class="space-y-3">
      <h3 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">Tool Paths</h3>
      <div>
        <label class="text-xs text-zinc-500" for="pass-binary">pass binary</label>
        <input
          id="pass-binary"
          type="text"
          bind:value={settings.config.pass_binary}
          placeholder="pass (default)"
          class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500"
        />
      </div>
      <div>
        <label class="text-xs text-zinc-500" for="gpg-binary">gpg binary</label>
        <input
          id="gpg-binary"
          type="text"
          bind:value={settings.config.gpg_binary}
          placeholder="gpg (default)"
          class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500"
        />
      </div>
      <div>
        <label class="text-xs text-zinc-500" for="git-binary">git binary</label>
        <input
          id="git-binary"
          type="text"
          bind:value={settings.config.git_binary}
          placeholder="git (default)"
          class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500"
        />
      </div>
    </section>

    <section class="space-y-3">
      <h3 class="text-sm font-medium text-zinc-400 uppercase tracking-wide">Password Store</h3>
      <div>
        <label class="text-xs text-zinc-500" for="store-dir">Default store directory</label>
        <input
          id="store-dir"
          type="text"
          bind:value={settings.config.password_store_dir}
          placeholder="~/.password-store (default)"
          class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-zinc-500"
        />
      </div>
      <div>
        <label class="text-xs text-zinc-500" for="clipboard-timeout">Clipboard timeout (seconds)</label>
        <input
          id="clipboard-timeout"
          type="number"
          bind:value={settings.config.clipboard_timeout}
          min={0}
          max={300}
          class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm text-zinc-100 focus:outline-none focus:border-zinc-500"
        />
      </div>
    </section>

    <button
      class="px-4 py-2 text-sm bg-zinc-100 text-zinc-900 rounded hover:bg-white transition-colors"
      onclick={handleSave}
    >
      Save Settings
    </button>
  </div>
</div>
