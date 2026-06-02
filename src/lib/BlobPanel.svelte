<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open as openDialog } from "@tauri-apps/plugin-dialog";
  import BlobTable from "./BlobTable.svelte";

  interface BlobRow {
    hash: string;
    name: string;
    rings: string[];
    ticket: string;
  }

  let rows: BlobRow[] = $state([]);
  let loading = $state(false);
  let error: string | null = $state(null);

  async function loadBlobs() {
    loading = true;
    error = null;
    try {
      rows = await invoke<BlobRow[]>("blob_list");
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleImport() {
    const path = await openDialog({ multiple: false, directory: false });
    if (!path) return;
    error = null;
    try {
      await invoke("blob_import", { path, rings: [] });
      await loadBlobs();
    } catch (e) {
      error = String(e);
    }
  }

  async function handleDelete(hash: string) {
    error = null;
    try {
      await invoke("blob_remove", { target: hash });
      rows = rows.filter((r) => r.hash !== hash);
    } catch (e) {
      error = String(e);
    }
  }

  $effect(() => {
    loadBlobs();
  });
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Blobs</h2>
    <button
      onclick={handleImport}
      class="rounded border border-amber-700/60 bg-amber-950/40 px-3 py-1.5 text-xs font-medium text-amber-300 transition-colors hover:border-amber-500 hover:bg-amber-900/40 hover:text-amber-200"
    >
      Import
    </button>
  </div>

  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">
      {error}
    </p>
  {/if}

  {#if loading}
    <p class="text-sm text-neutral-600">Loading…</p>
  {:else}
    <BlobTable {rows} onDelete={handleDelete} />
  {/if}
</div>
