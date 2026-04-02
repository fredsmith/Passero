<script lang="ts">
  import * as OTPAuth from "otpauth";
  import { ui } from "$lib/stores/ui.svelte";
  import { untrack } from "svelte";

  let { uri }: { uri: string } = $props();

  let code = $state("");
  let remaining = $state(0);
  let period = $state(30);
  let error = $state<string | null>(null);

  // NOT reactive — avoids $effect re-triggering when we set/clear the interval
  let intervalId: ReturnType<typeof setInterval> | null = null;

  function parseTotp(otpauthUri: string): OTPAuth.TOTP | null {
    try {
      const parsed = OTPAuth.URI.parse(otpauthUri);
      if (parsed instanceof OTPAuth.TOTP) {
        return parsed;
      }
      error = "Not a TOTP URI";
      return null;
    } catch (e) {
      error = String(e);
      return null;
    }
  }

  function tick(totp: OTPAuth.TOTP) {
    const now = Math.floor(Date.now() / 1000);
    const step = totp.period;
    const secs = step - (now % step);
    if (secs === step || code === "") {
      // New period or first run — regenerate
      code = totp.generate();
    }
    period = step;
    remaining = secs;
  }

  function stopTimer() {
    if (intervalId !== null) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }

  async function handleCopy() {
    if (!code) return;
    try {
      await navigator.clipboard.writeText(code);
      ui.notify("TOTP code copied");
    } catch (e) {
      ui.notify(String(e), "error");
    }
  }

  $effect(() => {
    // Only track `uri` — everything else is untracked to prevent loops
    const currentUri = uri;
    return untrack(() => {
      stopTimer();
      const totp = parseTotp(currentUri);
      if (!totp) return stopTimer;

      tick(totp);
      intervalId = setInterval(() => tick(totp), 1000);
      return stopTimer;
    });
  });

  let progress = $derived(period ? (remaining / period) * 100 : 0);
  let formattedCode = $derived(
    code ? code.slice(0, 3) + " " + code.slice(3) : "--- ---",
  );
  let urgent = $derived(remaining <= 5);
</script>

{#if error}
  <div class="text-xs text-red-400">{error}</div>
{:else}
  <div>
    <label class="text-xs text-zinc-500 uppercase tracking-wide">TOTP Code</label>
    <div class="mt-1 flex items-center gap-2">
      <div class="flex-1 bg-zinc-800 rounded px-3 py-2 flex items-center gap-3">
        <code class="text-lg font-mono tracking-widest {urgent ? 'text-orange-400' : 'text-green-400'}">
          {formattedCode}
        </code>
        <div class="flex-1">
          <div class="h-1 bg-zinc-700 rounded-full overflow-hidden">
            <div
              class="h-full rounded-full transition-all duration-1000 linear {urgent ? 'bg-orange-400' : 'bg-green-500'}"
              style="width: {progress}%"
            ></div>
          </div>
        </div>
        <span class="text-xs text-zinc-500 tabular-nums w-4 text-right">
          {remaining}s
        </span>
      </div>
      <button
        class="px-3 py-2 text-sm bg-zinc-800 hover:bg-zinc-700 rounded transition-colors"
        onclick={handleCopy}
      >
        Copy
      </button>
    </div>
  </div>
{/if}
