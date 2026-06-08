<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import type { RemoteBlobRow } from "./types";
  import { formatBytes } from "./utils";

  interface Progress { done: number; total: number }

  let peerId = $state("");
  let blobs: RemoteBlobRow[] = $state([]);
  let loading = $state(false);
  let error: string | null = $state(null);
  let fetched = $state(false);

  // Per-hash download state — allows concurrent downloads.
  let inFlight: Record<string, boolean> = $state({});
  let progresses: Record<string, Progress> = $state({});
  let downloadedAt: Record<string, number> = $state({});

  async function fetchCatalog() {
    const peer = peerId.trim();
    if (!peer) return;
    loading = true;
    blobs = [];
    error = null;
    fetched = false;
    try {
      blobs = await invoke<RemoteBlobRow[]>("remote_blob_list", { peer });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
      fetched = true;
    }
  }

  async function download(row: RemoteBlobRow) {
    const dest = await openDialog({ directory: true, multiple: false });
    if (!dest) return;
    inFlight[row.hash] = true;
    progresses[row.hash] = { done: 0, total: 0 };
    error = null;

    const unlisten: UnlistenFn = await listen<Progress>(
      `transfer_progress/${row.hash}`,
      (ev) => { progresses[row.hash] = ev.payload; },
    );

    try {
      await invoke("receive", { ticket: row.ticket, dest, hash: row.hash });
      downloadedAt[row.hash] = Date.now();
    } catch (e) {
      error = String(e);
    } finally {
      unlisten();
      inFlight[row.hash] = false;
    }
  }

  function progressPct(hash: string): number {
    const p = progresses[hash];
    return p && p.total > 0 ? Math.round((p.done / p.total) * 100) : 0;
  }

  function formatDownloadTime(ts: number): string {
    const diff = (Date.now() - ts) / 1000;
    if (diff < 60) return "just now";
    return new Date(ts).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
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
      <table class="table-fixed w-full text-sm">
        <thead>
          <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
            <th class="w-28 pb-2 text-left font-medium">Name</th>
            <th class="pb-2 text-left font-medium">Hash</th>
            <th class="w-24 pb-2 text-right font-medium">Downloaded</th>
            <th class="w-32 pb-2 text-right font-medium"></th>
          </tr>
        </thead>
        <tbody>
          {#each blobs as row (row.hash)}
            <tr class="border-b border-neutral-900 hover:bg-neutral-900/50">
              <td class="overflow-hidden break-words py-2.5 pr-4 text-neutral-100">{row.name}</td>
              <td class="max-w-0 py-2.5 pr-4">
                <span class="block truncate font-mono text-xs text-neutral-500" title={row.hash}>{row.hash}</span>
              </td>
              <td class="py-2.5 pr-4 text-right font-mono text-xs text-neutral-600">
                {downloadedAt[row.hash] ? formatDownloadTime(downloadedAt[row.hash]) : "—"}
              </td>
              <td class="py-2.5 text-right">
                <div class="flex flex-col items-end gap-1">
                  <button
                    onclick={() => download(row)}
                    disabled={!!inFlight[row.hash]}
                    title={inFlight[row.hash] ? "Download in progress" : `Download ${row.name}`}
                    class="rounded border border-neutral-700 bg-neutral-800 px-2.5 py-1 text-xs text-neutral-300 transition-colors hover:border-amber-700 hover:text-amber-300 disabled:cursor-not-allowed disabled:opacity-40"
                    aria-label="Download {row.name}"
                  >Download</button>
                  {#if inFlight[row.hash]}
                    <div class="w-full min-w-24">
                      <div class="h-1 w-full overflow-hidden rounded-full bg-neutral-800">
                        <div class="h-full rounded-full bg-amber-500 transition-all duration-150" style="width:{progressPct(row.hash)}%"></div>
                      </div>
                      <span class="text-xs text-neutral-600">
                        {progresses[row.hash]?.total > 0
                          ? `${formatBytes(progresses[row.hash].done)} / ${formatBytes(progresses[row.hash].total)}`
                          : "Connecting…"}
                      </span>
                    </div>
                  {/if}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else if fetched && !loading && !error}
    <p class="text-sm italic text-neutral-600">No accessible blobs on remote node.</p>
  {/if}
</div>
