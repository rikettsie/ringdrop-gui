<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import { formatBytes } from "./utils";

  let ticket = $state("");
  let dest = $state("");
  let downloading = $state(false);
  let done = $state(0);
  let total = $state(0);
  let error: string | null = $state(null);
  let success = $state(false);

  let progress = $derived(total > 0 ? done / total : 0);
  let canDownload = $derived(ticket.trim().length > 0 && dest.length > 0 && !downloading);

  async function pickDest() {
    const path = await openDialog({ directory: true, multiple: false });
    if (path) dest = path as string;
  }

  async function download() {
    if (!canDownload) return;
    downloading = true;
    done = 0;
    total = 0;
    error = null;
    success = false;

    const unlisten: UnlistenFn = await listen<{ done: number; total: number }>(
      "transfer_progress",
      (ev) => {
        done = ev.payload.done;
        total = ev.payload.total;
      },
    );

    try {
      await invoke("receive", { ticket: ticket.trim(), dest });
      success = true;
    } catch (e) {
      error = String(e);
    } finally {
      unlisten();
      downloading = false;
    }
  }
</script>

<div class="flex flex-col gap-4">
  <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Receive</h2>

  <!-- Ticket input -->
  <div class="flex flex-col gap-1.5">
    <label for="ticket-input" class="text-xs text-neutral-500">Share ticket</label>
    <input
      id="ticket-input"
      type="text"
      placeholder="rdrop://…"
      bind:value={ticket}
      disabled={downloading}
      class="rounded border border-neutral-800 bg-neutral-900 px-3 py-2 text-sm text-neutral-100 placeholder-neutral-700 outline-none transition-colors focus:border-amber-700 disabled:opacity-50"
    />
  </div>

  <!-- Destination -->
  <div class="flex flex-col gap-1.5">
    <span class="text-xs text-neutral-500">Destination</span>
    <div class="flex gap-2">
      <input
        type="text"
        readonly
        value={dest}
        placeholder="Pick a directory…"
        class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-2 text-sm text-neutral-400 placeholder-neutral-700 outline-none"
      />
      <button
        onclick={pickDest}
        disabled={downloading}
        class="shrink-0 rounded border border-neutral-700 bg-neutral-800 px-3 py-2 text-xs text-neutral-300 transition-colors hover:border-neutral-600 hover:text-neutral-100 disabled:opacity-50"
      >Browse</button>
    </div>
  </div>

  <!-- Download button -->
  <button
    onclick={download}
    disabled={!canDownload}
    class="rounded border border-amber-700/60 bg-amber-950/40 px-4 py-2 text-sm font-medium text-amber-300 transition-colors hover:border-amber-500 hover:bg-amber-900/40 hover:text-amber-200 disabled:cursor-not-allowed disabled:opacity-40"
  >
    {downloading ? "Downloading…" : "Download"}
  </button>

  <!-- Progress bar -->
  {#if downloading || (success && total > 0)}
    <div class="flex flex-col gap-1">
      <div class="h-1.5 w-full overflow-hidden rounded-full bg-neutral-800">
        <div
          class="h-full rounded-full bg-amber-500 transition-all duration-150"
          style="width: {Math.round(progress * 100)}%"
        ></div>
      </div>
      <p class="text-right text-xs text-neutral-600">
        {#if total > 0}
          {formatBytes(done)} / {formatBytes(total)} ({Math.round(progress * 100)}%)
        {:else}
          Connecting…
        {/if}
      </p>
    </div>
  {/if}

  <!-- Status messages -->
  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">
      {error}
    </p>
  {/if}
  {#if success}
    <p class="rounded border border-emerald-900/50 bg-emerald-950/30 px-3 py-2 text-xs text-emerald-400">
      Download complete.
    </p>
  {/if}
</div>
