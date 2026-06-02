<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { PeerEntry } from "./types";
  import { truncateHash } from "./utils";
  import ConfirmButton from "./ConfirmButton.svelte";

  let peers: PeerEntry[] = $state([]);
  let error: string | null = $state(null);
  let adding = $state(false);
  let newPeerId = $state("");
  let newNickname = $state("");

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

  $effect(() => { load(); });
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Peers</h2>
    <button
      onclick={() => { adding = true; newPeerId = ""; newNickname = ""; }}
      class="rounded border border-amber-700/60 bg-amber-950/40 px-2.5 py-1 text-xs font-medium text-amber-300 transition-colors hover:border-amber-500 hover:bg-amber-900/40"
    >Add peer</button>
  </div>

  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">{error}</p>
  {/if}

  {#if adding}
    <div class="flex flex-col gap-2 rounded border border-neutral-800 bg-neutral-900/50 p-3">
      <div class="flex gap-2">
        <input
          type="text"
          placeholder="Base32 peer ID"
          bind:value={newPeerId}
          class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-1.5 text-xs text-neutral-100 outline-none focus:border-amber-700"
          aria-label="Peer ID"
        />
        <input
          type="text"
          placeholder="Nickname (optional)"
          bind:value={newNickname}
          onkeydown={(e) => e.key === "Enter" && addPeer()}
          class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-1.5 text-xs text-neutral-100 outline-none focus:border-amber-700"
          aria-label="Nickname"
        />
      </div>
      <div class="flex gap-2">
        <button
          onclick={addPeer}
          class="rounded border border-amber-700/60 bg-amber-950/40 px-3 py-1.5 text-xs text-amber-300 transition-colors hover:border-amber-500"
        >Add</button>
        <button onclick={() => (adding = false)} class="text-xs text-neutral-600 hover:text-neutral-400">Cancel</button>
      </div>
    </div>
  {/if}

  <div class="overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
          <th class="pb-2 text-left font-medium">Nickname</th>
          <th class="pb-2 text-left font-medium">Peer ID</th>
          <th class="pb-2 text-right font-medium"></th>
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
            <td class="py-2.5 pr-4 text-neutral-200">
              {#if p.nickname}{p.nickname}{:else}<span class="italic text-neutral-600">—</span>{/if}
            </td>
            <td class="py-2.5 pr-4 font-mono text-xs text-neutral-500">
              {truncateHash(p.peer_id, 16)}
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
