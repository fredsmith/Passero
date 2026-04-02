<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import MainView from "./views/MainView.svelte";
  import SettingsView from "./views/SettingsView.svelte";
  import GeneratorView from "./views/GeneratorView.svelte";
  import GpgView from "./views/GpgView.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import { passwords } from "$lib/stores/passwords.svelte";
  import { settings } from "$lib/stores/settings.svelte";
  import { onMount } from "svelte";

  onMount(() => {
    passwords.refresh();
    settings.load();
  });
</script>

<div class="flex h-screen bg-zinc-900 text-zinc-100">
  <Sidebar />
  <main class="flex-1 overflow-hidden flex flex-col">
    {#if ui.notification}
      <div
        class="px-4 py-2 text-sm {ui.notification.type === 'error'
          ? 'bg-red-900/50 text-red-200'
          : 'bg-green-900/50 text-green-200'}"
      >
        {ui.notification.message}
      </div>
    {/if}
    {#if ui.currentView === "main"}
      <MainView />
    {:else if ui.currentView === "settings"}
      <SettingsView />
    {:else if ui.currentView === "generator"}
      <GeneratorView />
    {:else if ui.currentView === "gpg"}
      <GpgView />
    {/if}
  </main>
</div>
