<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import QRCode from "qrcode";

  let peerId: string | null = $state(null);
  let qrSvg: string | null = $state(null);
  let copied = $state(false);
  let error: string | null = $state(null);

  async function load() {
    error = null;
    try {
      peerId = await invoke<string>("node_id");
      qrSvg = await QRCode.toString(peerId, {
        type: "svg",
        width: 160,
        margin: 1,
        errorCorrectionLevel: "H",
        color: { dark: "#f59e0b", light: "#0a0a0a" },
      });
    } catch (e) {
      error = String(e);
    }
  }

  async function copy() {
    if (!peerId) return;
    await navigator.clipboard.writeText(peerId);
    copied = true;
    setTimeout(() => (copied = false), 1800);
  }

  $effect(() => {
    load();
  });
</script>

<div class="flex flex-col gap-6">
  <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Your ID</h2>

  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">
      {error}
    </p>
  {:else if peerId}
    <!-- Peer ID display -->
    <div class="flex flex-col gap-2">
      <span class="text-xs text-neutral-500">Peer ID — share this so others can add you to their rings</span>
      <div class="flex items-start gap-2">
        <code
          class="flex-1 break-all rounded border border-neutral-800 bg-neutral-900 px-3 py-2 text-xs leading-relaxed text-amber-300 font-mono"
        >{peerId}</code>
        <button
          onclick={copy}
          title="Copy peer ID"
          class="shrink-0 rounded border border-neutral-700 bg-neutral-800 px-3 py-2 text-xs transition-colors hover:border-amber-700 hover:text-amber-300
            {copied ? 'border-emerald-700 text-emerald-400' : 'text-neutral-400'}"
        >{copied ? "Copied!" : "Copy"}</button>
      </div>
    </div>

    <!-- QR code -->
    {#if qrSvg}
      <div class="flex flex-col gap-2">
        <span class="text-xs text-neutral-500">They can also scan to import your ID</span>
        <div
          class="relative w-fit rounded border border-neutral-800 bg-[#0a0a0a] p-3"
          aria-label="QR code for peer ID"
        >
          {@html qrSvg}
          <img
            src="/mascot.png"
            alt=""
            class="absolute left-1/2 top-1/2 h-9 w-9 -translate-x-1/2 -translate-y-1/2 rounded-full bg-[#0a0a0a] object-contain p-0.5"
          />
        </div>
      </div>
    {/if}
  {:else}
    <p class="text-sm text-neutral-600">Loading…</p>
  {/if}
</div>
