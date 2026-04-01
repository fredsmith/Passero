<script lang="ts">
  import SearchBar from "../components/SearchBar.svelte";
  import Toolbar from "../components/Toolbar.svelte";
  import PasswordTree from "../components/PasswordTree.svelte";
  import PasswordViewer from "../components/PasswordViewer.svelte";
  import PasswordEditor from "../components/PasswordEditor.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import { passwords } from "$lib/stores/passwords.svelte";

  let showNewEditor = $state(false);
</script>

<div class="flex flex-col h-full">
  <Toolbar onNewEntry={() => (showNewEditor = true)} />
  <div class="flex flex-1 overflow-hidden">
    <div class="w-72 border-r border-zinc-800 flex flex-col">
      <SearchBar />
      <PasswordTree />
    </div>
    <PasswordViewer />
  </div>
</div>

{#if showNewEditor}
  <PasswordEditor editPath={null} onclose={() => (showNewEditor = false)} />
{/if}

{#if ui.showEditor && passwords.selectedPath}
  <PasswordEditor
    editPath={passwords.selectedPath}
    onclose={() => (ui.showEditor = false)}
  />
{/if}
