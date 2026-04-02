<script lang="ts">
  import { insertTotp, importTotpFromQr } from "$lib/commands";
  import { passwords } from "$lib/stores/passwords.svelte";
  import { ui } from "$lib/stores/ui.svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  let { path, onclose }: { path: string; onclose: () => void } = $props();

  let mode = $state<"code" | "qr">("code");
  let secret = $state("");
  let issuer = $state("");
  let account = $state("");
  let saving = $state(false);
  let qrFilePath = $state<string | null>(null);

  async function handleSubmitCode(e: Event) {
    e.preventDefault();
    if (!secret.trim()) return;

    saving = true;
    try {
      await insertTotp(
        path,
        secret.trim(),
        issuer.trim() || undefined,
        account.trim() || undefined,
      );
      ui.notify("TOTP secret added");
      await passwords.select(path);
      onclose();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      saving = false;
    }
  }

  async function handlePickImage() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          { name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "bmp", "webp"] },
        ],
      });
      if (selected) {
        qrFilePath = selected as string;
      }
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  async function handleSubmitQr() {
    if (!qrFilePath) return;

    saving = true;
    try {
      await importTotpFromQr(path, qrFilePath);
      ui.notify("TOTP imported from QR code");
      await passwords.select(path);
      onclose();
    } catch (e) {
      ui.notify(String(e), "error");
    } finally {
      saving = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 bg-black/60 flex items-center justify-center z-50"
  onkeydown={(e) => e.key === "Escape" && onclose()}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="bg-zinc-900 border border-zinc-700 rounded-lg w-full max-w-md p-6" onclick={(e) => e.stopPropagation()}>
    <h3 class="text-lg font-medium mb-4">Add TOTP Secret</h3>

    <div class="flex gap-2 mb-4">
      <button
        class="px-3 py-1 text-sm rounded transition-colors {mode === 'code'
          ? 'bg-zinc-600 text-white'
          : 'bg-zinc-800 text-zinc-400 hover:text-white'}"
        onclick={() => (mode = "code")}
      >
        Setup Code
      </button>
      <button
        class="px-3 py-1 text-sm rounded transition-colors {mode === 'qr'
          ? 'bg-zinc-600 text-white'
          : 'bg-zinc-800 text-zinc-400 hover:text-white'}"
        onclick={() => (mode = "qr")}
      >
        QR Image
      </button>
    </div>

    {#if mode === "code"}
      <p class="text-sm text-zinc-400 mb-4">
        Enter the setup key provided by the service (the text alternative to scanning a QR code).
      </p>
      <form onsubmit={handleSubmitCode} class="space-y-4">
        <div>
          <label for="totp-secret" class="text-xs text-zinc-500 uppercase tracking-wide">
            Secret Key <span class="text-red-400">*</span>
          </label>
          <input
            id="totp-secret"
            type="text"
            bind:value={secret}
            placeholder="JBSWY3DPEHPK3PXP"
            class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm font-mono focus:outline-none focus:border-zinc-500"
            required
          />
        </div>
        <div>
          <label for="totp-issuer" class="text-xs text-zinc-500 uppercase tracking-wide">
            Issuer (optional)
          </label>
          <input
            id="totp-issuer"
            type="text"
            bind:value={issuer}
            placeholder="e.g. GitHub, Google"
            class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
          />
        </div>
        <div>
          <label for="totp-account" class="text-xs text-zinc-500 uppercase tracking-wide">
            Account (optional)
          </label>
          <input
            id="totp-account"
            type="text"
            bind:value={account}
            placeholder="e.g. user@example.com"
            class="mt-1 w-full bg-zinc-800 border border-zinc-700 rounded px-3 py-2 text-sm focus:outline-none focus:border-zinc-500"
          />
        </div>
        <div class="flex justify-end gap-2 pt-2">
          <button
            type="button"
            class="px-4 py-2 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
            onclick={onclose}
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={saving || !secret.trim()}
            class="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 disabled:opacity-50 rounded transition-colors"
          >
            {saving ? "Saving..." : "Add TOTP"}
          </button>
        </div>
      </form>
    {:else}
      <p class="text-sm text-zinc-400 mb-4">
        Select an image file containing a TOTP QR code (screenshot or saved image).
      </p>
      <div class="space-y-4">
        <div>
          <button
            class="w-full px-4 py-3 text-sm bg-zinc-800 hover:bg-zinc-700 border border-zinc-700 border-dashed rounded transition-colors"
            onclick={handlePickImage}
          >
            {qrFilePath ? qrFilePath.split("/").pop() : "Choose QR code image..."}
          </button>
        </div>
        <div class="flex justify-end gap-2 pt-2">
          <button
            type="button"
            class="px-4 py-2 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
            onclick={onclose}
          >
            Cancel
          </button>
          <button
            disabled={saving || !qrFilePath}
            class="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 disabled:opacity-50 rounded transition-colors"
            onclick={handleSubmitQr}
          >
            {saving ? "Importing..." : "Import from QR"}
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>
