<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import type { RemoteBlobRow } from "./types";
  import { truncateHash, formatBytes } from "./utils";

  interface Progress { done: number; total: number }

  let peerId = $state("");
  let blobs: RemoteBlobRow[] = $state([]);
  let loading = $state(false);
  let error: string | null = $state(null);

  let downloading: string | null = $state(null);
  let progress: Progress = $state({ done: 0, total: 0 });
  let downloadDone: string | null = $state(null);

  let progressPct = $derived(
    progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0,
  );

  async function fetchCatalog() {
    const peer = peerId.trim();
    if (!peer) return;
    loading = true;
    blobs = [];
    error = null;
    try {
      blobs = await invoke<RemoteBlobRow[]>("remote_blob_list", { peer });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function download(row: RemoteBlobRow) {
    const dest = await openDialog({ directory: true, multiple: false });
    if (!dest) return;
    downloading = row.hash;
    downloadDone = null;
    progress = { done: 0, total: 0 };
    error = null;

    const unlisten: UnlistenFn = await listen<Progress>("transfer_progress", (ev) => {
      progress = ev.payload;
    });

    try {
      await invoke("receive", { ticket: row.ticket, dest });
      downloadDone = row.hash;
    } catch (e) {
      error = String(e);
    } finally {
      unlisten();
      downloading = null;
    }
  }
</script>

<div class="flex flex-col gap-4">
  <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Remote catalog</h2>

  <div class="flex gap-2">
    <input
      type="text"
      placeholder="Remote peer ID (base32)"
      bind:value={peerId}
      onkeydown={(e) => e.key === "Enter" && fetchCatalog()}
      class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-2 text-sm text-neutral-100 placeholder-neutral-700 outline-none transition-colors focus:border-amber-700"
      aria-label="Remote peer ID"
    />
    <button
      onclick={fetchCatalog}
      disabled={loading || !peerId.trim()}
      class="shrink-0 rounded border border-amber-700/60 bg-amber-950/40 px-4 py-2 text-sm font-medium text-amber-300 transition-colors hover:border-amber-500 hover:bg-amber-900/40 disabled:cursor-not-allowed disabled:opacity-40"
    >{loading ? "Fetching…" : "Browse"}</button>
  </div>

  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">{error}</p>
  {/if}

  {#if blobs.length > 0}
    <div class="overflow-x-auto">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
            <th class="pb-2 text-left font-medium">Name</th>
            <th class="pb-2 text-left font-medium">Hash</th>
            <th class="pb-2 text-right font-medium"></th>
          </tr>
        </thead>
        <tbody>
          {#each blobs as row (row.hash)}
            <tr class="border-b border-neutral-900 hover:bg-neutral-900/50">
              <td class="py-2.5 pr-4 text-neutral-100">{row.name}</td>
              <td class="py-2.5 pr-4 font-mono text-xs text-neutral-500">{truncateHash(row.hash, 12)}</td>
              <td class="py-2.5 text-right">
                {#if downloading === row.hash}
                  <div class="flex min-w-32 flex-col items-end gap-1">
                    <div class="h-1 w-full overflow-hidden rounded-full bg-neutral-800">
                      <div class="h-full rounded-full bg-amber-500 transition-all duration-150" style="width:{progressPct}%"></div>
                    </div>
                    <span class="text-xs text-neutral-600">
                      {progress.total > 0 ? `${formatBytes(progress.done)} / ${formatBytes(progress.total)}` : "Connecting…"}
                    </span>
                  </div>
                {:else if downloadDone === row.hash}
                  <span class="text-xs text-emerald-500">Done</span>
                {:else}
                  <button
                    onclick={() => download(row)}
                    class="rounded border border-neutral-700 bg-neutral-800 px-2.5 py-1 text-xs text-neutral-300 transition-colors hover:border-amber-700 hover:text-amber-300"
                    aria-label="Download {row.name}"
                  >Download</button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else if !loading && peerId.trim()}
    <p class="text-sm italic text-neutral-700">No accessible blobs from this peer.</p>
  {/if}
</div>
