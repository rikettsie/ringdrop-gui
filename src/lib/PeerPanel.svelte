<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import jsQR from "jsqr";
  import type { PeerEntry } from "./types";
  import ConfirmButton from "./ConfirmButton.svelte";

  let peers: PeerEntry[] = $state([]);
  let error: string | null = $state(null);
  let adding = $state(false);
  let newPeerId = $state("");
  let newNickname = $state("");
  let scanError: string | null = $state(null);
  let fileInput: HTMLInputElement | undefined = $state();

  async function load() {
    error = null;
    try {
      peers = await invoke<PeerEntry[]>("peer_list");
    } catch (e) {
      error = String(e);
    }
  }

  async function addPeer() {
    const peer = newPeerId.trim();
    if (!peer) return;
    error = null;
    try {
      await invoke("peer_add", { peer, nickname: newNickname.trim() || null });
      newPeerId = "";
      newNickname = "";
      adding = false;
      await load();
    } catch (e) {
      error = String(e);
    }
  }

  async function removePeer(peerId: string) {
    error = null;
    try {
      await invoke("peer_remove", { peer: peerId });
      peers = peers.filter((p) => p.peer_id !== peerId);
    } catch (e) {
      error = String(e);
    }
  }

  async function handleQrScan(e: Event) {
    scanError = null;
    const file = (e.target as HTMLInputElement).files?.[0];
    if (!file) return;
    (e.target as HTMLInputElement).value = "";

    try {
      const url = URL.createObjectURL(file);
      const img = await new Promise<HTMLImageElement>((resolve, reject) => {
        const el = new Image();
        el.onload = () => resolve(el);
        el.onerror = reject;
        el.src = url;
      });
      URL.revokeObjectURL(url);

      const canvas = document.createElement("canvas");
      canvas.width = img.naturalWidth;
      canvas.height = img.naturalHeight;
      const ctx = canvas.getContext("2d")!;
      ctx.drawImage(img, 0, 0);
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);

      const code = jsQR(imageData.data, imageData.width, imageData.height);
      if (code) {
        newPeerId = code.data;
      } else {
        scanError = "No QR code found in the image.";
      }
    } catch (err) {
      scanError = `Failed to read image: ${err}`;
    }
  }

  $effect(() => { load(); });
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Peers</h2>
    <button
      onclick={() => { adding = true; newPeerId = ""; newNickname = ""; scanError = null; }}
      class="rounded border border-amber-700/60 bg-amber-950/40 px-2.5 py-1 text-xs font-medium text-amber-300 transition-colors hover:border-amber-500 hover:bg-amber-900/40"
    >Add peer</button>
  </div>

  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">{error}</p>
  {/if}

  {#if adding}
    <div class="flex flex-col gap-2 rounded border border-neutral-800 bg-neutral-900/50 p-3">
      <div class="flex gap-2">
        <div class="flex min-w-0 flex-1 gap-1">
          <input
            type="text"
            placeholder="Base32 peer ID"
            bind:value={newPeerId}
            class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-1.5 text-xs text-neutral-100 outline-none focus:border-amber-700"
            aria-label="Peer ID"
          />
          <button
            type="button"
            onclick={() => fileInput?.click()}
            title="Scan QR code from image file"
            aria-label="Scan QR code from image"
            class="shrink-0 rounded border border-neutral-800 bg-neutral-900 px-2 py-1.5 text-neutral-500 transition-colors hover:border-amber-700 hover:text-amber-400"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
              <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 013.75 9.375v-4.5zM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 01-1.125-1.125v-4.5zM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0113.5 9.375v-4.5z"/>
              <path d="M6.75 6.75h.75v.75h-.75v-.75zM6.75 16.5h.75v.75h-.75V16.5zM16.5 6.75h.75v.75h-.75v-.75zM13.5 13.5h.75v.75h-.75v-.75zM13.5 19.5h.75v.75h-.75v-.75zM19.5 13.5h.75v.75h-.75v-.75zM19.5 19.5h.75v.75h-.75v-.75zM16.5 16.5h.75v.75h-.75v-.75z"/>
            </svg>
          </button>
        </div>
        <input
          type="text"
          placeholder="Nickname (optional)"
          bind:value={newNickname}
          onkeydown={(e) => e.key === "Enter" && addPeer()}
          class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-1.5 text-xs text-neutral-100 outline-none focus:border-amber-700"
          aria-label="Nickname"
        />
      </div>
      <input
        type="file"
        accept="image/*"
        bind:this={fileInput}
        class="hidden"
        onchange={handleQrScan}
      />
      {#if scanError}
        <p class="text-xs text-red-400">{scanError}</p>
      {/if}
      <div class="flex gap-2">
        <button
          onclick={addPeer}
          class="rounded border border-amber-700/60 bg-amber-950/40 px-3 py-1.5 text-xs text-amber-300 transition-colors hover:border-amber-500"
        >Add</button>
        <button onclick={() => { adding = false; scanError = null; }} class="text-xs text-neutral-600 hover:text-neutral-400">Cancel</button>
      </div>
    </div>
  {/if}

  <div class="overflow-x-auto">
    <table class="table-fixed w-full text-sm">
      <thead>
        <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
          <th class="w-28 pb-2 text-left font-medium">Nickname</th>
          <th class="pb-2 text-left font-medium">Peer ID</th>
          <th class="w-8 pb-2 text-right font-medium"></th>
        </tr>
      </thead>
      <tbody>
        {#if peers.length === 0}
          <tr>
            <td colspan="3" class="py-10 text-center text-sm italic text-neutral-700">
              No peers in address book.
            </td>
          </tr>
        {/if}
        {#each peers as p (p.peer_id)}
          <tr class="border-b border-neutral-900 hover:bg-neutral-900/50">
            <td class="overflow-hidden break-words py-2.5 pr-4 text-neutral-200">
              {#if p.nickname}{p.nickname}{:else}<span class="italic text-neutral-600">—</span>{/if}
            </td>
            <td class="max-w-0 py-2.5 pr-4">
              <span class="block truncate font-mono text-xs text-neutral-500" title={p.peer_id}>{p.peer_id}</span>
            </td>
            <td class="py-2.5 text-right">
              <ConfirmButton label="Remove?" onConfirm={() => removePeer(p.peer_id)}>
                <button
                  class="text-neutral-600 transition-colors hover:text-red-500"
                  aria-label="Remove {p.nickname ?? p.peer_id}"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="inline h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </ConfirmButton>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
