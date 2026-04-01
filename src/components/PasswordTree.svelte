<script lang="ts">
  import { passwords } from "$lib/stores/passwords.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import { filterTree } from "$lib/utils";
  import PasswordTreeNode from "./PasswordTreeNode.svelte";

  const filteredTree = $derived(filterTree(passwords.tree, ui.searchQuery));
</script>

<div class="flex-1 overflow-y-auto p-2">
  {#if passwords.loading}
    <div class="text-zinc-500 text-sm p-4">Loading...</div>
  {:else if passwords.error}
    <div class="text-red-400 text-sm p-4">{passwords.error}</div>
  {:else if filteredTree.length === 0}
    <div class="text-zinc-500 text-sm p-4">
      {#if ui.searchQuery}
        No matches for "{ui.searchQuery}"
      {:else}
        No passwords found. Is your password store initialized?
      {/if}
    </div>
  {:else}
    {#each filteredTree as entry (entry.path)}
      <PasswordTreeNode {entry} depth={0} />
    {/each}
  {/if}
</div>
