<script lang="ts">
  import { ui } from "$lib/stores/ui.svelte";
  import { gitPull, gitPush } from "$lib/commands";
  import { passwords } from "$lib/stores/passwords.svelte";

  async function handleSync(action: "pull" | "push") {
    try {
      if (action === "pull") {
        await gitPull();
        await passwords.refresh();
        ui.notify("Pulled from remote");
      } else {
        await gitPush();
        ui.notify("Pushed to remote");
      }
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }
</script>

<aside class="w-56 bg-zinc-950 border-r border-zinc-800 flex flex-col">
  <div class="p-4 pt-10 border-b border-zinc-800" data-tauri-drag-region>
    <h1 class="text-lg font-semibold tracking-tight">Passero</h1>
    <p class="text-xs text-zinc-500">password-store</p>
  </div>

  <nav class="flex-1 p-2 space-y-1">
    <button
      class="w-full text-left px-3 py-2 rounded text-sm transition-colors {ui.currentView ===
      'main'
        ? 'bg-zinc-800 text-white'
        : 'text-zinc-400 hover:text-white hover:bg-zinc-800/50'}"
      onclick={() => ui.navigate("main")}
    >
      Passwords
    </button>
    <button
      class="w-full text-left px-3 py-2 rounded text-sm transition-colors {ui.currentView ===
      'generator'
        ? 'bg-zinc-800 text-white'
        : 'text-zinc-400 hover:text-white hover:bg-zinc-800/50'}"
      onclick={() => ui.navigate("generator")}
    >
      Generator
    </button>
    <button
      class="w-full text-left px-3 py-2 rounded text-sm transition-colors {ui.currentView ===
      'settings'
        ? 'bg-zinc-800 text-white'
        : 'text-zinc-400 hover:text-white hover:bg-zinc-800/50'}"
      onclick={() => ui.navigate("settings")}
    >
      Settings
    </button>
  </nav>

  <div class="p-2 border-t border-zinc-800 space-y-1">
    <button
      class="w-full text-left px-3 py-2 rounded text-sm text-zinc-400 hover:text-white hover:bg-zinc-800/50 transition-colors"
      onclick={() => handleSync("pull")}
    >
      Pull
    </button>
    <button
      class="w-full text-left px-3 py-2 rounded text-sm text-zinc-400 hover:text-white hover:bg-zinc-800/50 transition-colors"
      onclick={() => handleSync("push")}
    >
      Push
    </button>
  </div>
</aside>
