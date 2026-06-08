<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { RingRow, PeerEntry } from "./types";
  import ConfirmButton from "./ConfirmButton.svelte";

  let rings: RingRow[] = $state([]);
  let selected: RingRow | null = $state(null);
  let members: PeerEntry[] = $state([]);
  let error: string | null = $state(null);

  let creating = $state(false);
  let newRingName = $state("");

  let addingPeer = $state(false);
  let newPeerId = $state("");

  async function loadRings() {
    error = null;
    try {
      rings = await invoke<RingRow[]>("ring_list");
    } catch (e) {
      error = String(e);
    }
  }

  async function selectRing(ring: RingRow) {
    selected = ring;
    addingPeer = false;
    newPeerId = "";
    error = null;
    try {
      members = await invoke<PeerEntry[]>("ring_members", { ring: ring.name });
    } catch (e) {
      error = String(e);
    }
  }

  async function createRing() {
    const name = newRingName.trim();
    if (!name) return;
    error = null;
    try {
      await invoke("ring_create", { name });
      newRingName = "";
      creating = false;
      await loadRings();
      const created = rings.find((r) => r.name === name);
      if (created) await selectRing(created);
    } catch (e) {
      error = String(e);
    }
  }

  async function addPeer() {
    const peer = newPeerId.trim();
    if (!peer || !selected) return;
    error = null;
    try {
      await invoke("ring_add", { ring: selected.name, peer });
      newPeerId = "";
      addingPeer = false;
      members = await invoke<PeerEntry[]>("ring_members", { ring: selected.name });
    } catch (e) {
      error = String(e);
    }
  }

  async function removePeer(peerId: string) {
    if (!selected) return;
    error = null;
    try {
      await invoke("ring_remove", { ring: selected.name, peer: peerId });
      members = members.filter((m) => m.peer_id !== peerId);
    } catch (e) {
      error = String(e);
    }
  }

  $effect(() => { loadRings(); });
</script>

<div class="flex h-full gap-6">

  <!-- Ring list -->
  <div class="flex w-44 shrink-0 flex-col gap-1">
    <div class="mb-1 flex items-center justify-between">
      <span class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Rings</span>
      <button
        onclick={() => { creating = true; newRingName = ""; }}
        title="New ring"
        class="text-lg leading-none text-neutral-600 transition-colors hover:text-amber-400"
        aria-label="New ring"
      >+</button>
    </div>

    {#each rings as ring (ring.name)}
      <button
        onclick={() => selectRing(ring)}
        class="flex items-center gap-1.5 rounded px-2 py-1.5 text-left text-sm transition-colors
          {selected?.name === ring.name
            ? 'bg-amber-950/60 text-amber-300 font-medium'
            : 'text-neutral-400 hover:bg-neutral-900 hover:text-neutral-200'}"
      >
        <span class="flex-1 truncate">{ring.name}</span>
        {#if ring.open}
          <span class="shrink-0 rounded bg-neutral-800 px-1 text-xs text-neutral-500">pub</span>
        {/if}
      </button>
    {/each}

    {#if rings.length === 0 && !creating}
      <p class="mt-2 text-xs italic text-neutral-700">No rings yet.</p>
    {/if}

    {#if creating}
      <div class="mt-1 flex gap-1">
        <input
          type="text"
          placeholder="ring name"
          bind:value={newRingName}
          onkeydown={(e) => e.key === "Enter" && createRing()}
          class="min-w-0 flex-1 rounded border border-amber-800/50 bg-neutral-900 px-2 py-1 text-xs text-neutral-100 outline-none focus:border-amber-600"
          aria-label="New ring name"
        />
        <button onclick={createRing} class="px-1 text-xs text-amber-400 hover:text-amber-300">✓</button>
        <button onclick={() => (creating = false)} class="px-1 text-xs text-neutral-600 hover:text-neutral-400">✕</button>
      </div>
    {/if}
  </div>

  <!-- Ring detail -->
  <div class="flex min-w-0 flex-1 flex-col gap-4">
    {#if error}
      <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">{error}</p>
    {/if}

    {#if selected}
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">
          {selected.name}
          {#if selected.open}<span class="ml-2 normal-case text-neutral-600">public ring</span>{/if}
        </h2>
        {#if !selected.open}
          <button
            onclick={() => { addingPeer = true; newPeerId = ""; }}
            class="rounded border border-amber-700/60 bg-amber-950/40 px-2.5 py-1 text-xs font-medium text-amber-300 transition-colors hover:border-amber-500"
          >Add peer</button>
        {/if}
      </div>

      {#if addingPeer}
        <div class="flex gap-2">
          <input
            type="text"
            placeholder="Base32 peer ID"
            bind:value={newPeerId}
            onkeydown={(e) => e.key === "Enter" && addPeer()}
            class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-1.5 text-xs text-neutral-100 outline-none focus:border-amber-700"
            aria-label="Peer ID to add"
          />
          <button onclick={addPeer} class="rounded border border-amber-700/60 bg-amber-950/40 px-3 py-1.5 text-xs text-amber-300 transition-colors hover:border-amber-500">Add</button>
          <button onclick={() => (addingPeer = false)} class="text-xs text-neutral-600 hover:text-neutral-400">✕</button>
        </div>
      {/if}

      {#if members.length === 0}
        <p class="text-sm italic text-neutral-700">No members yet.</p>
      {:else}
        <div class="flex flex-col gap-1">
          {#each members as m (m.peer_id)}
            <div class="flex items-center justify-between rounded border border-neutral-900 bg-neutral-900/40 px-3 py-2">
              <div class="flex min-w-0 flex-col">
                {#if m.nickname}
                  <span class="text-sm text-neutral-200">{m.nickname}</span>
                  <span class="block truncate font-mono text-xs text-neutral-600" title={m.peer_id}>{m.peer_id}</span>
                {:else}
                  <span class="block truncate font-mono text-sm text-neutral-400" title={m.peer_id}>{m.peer_id}</span>
                {/if}
              </div>
              <ConfirmButton label="Remove?" onConfirm={() => removePeer(m.peer_id)}>
                <button
                  class="text-neutral-700 transition-colors hover:text-red-500"
                  aria-label="Remove {m.nickname ?? m.peer_id} from ring"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M13 7a4 4 0 11-8 0 4 4 0 018 0zM9 14a6 6 0 00-6 6h12a6 6 0 00-6-6zM21 12h-6" />
                  </svg>
                </button>
              </ConfirmButton>
            </div>
          {/each}
        </div>
      {/if}
    {:else}
      <p class="text-sm italic text-neutral-700">Select a ring to manage its members.</p>
    {/if}
  </div>

</div>
