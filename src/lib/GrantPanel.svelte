<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { truncateHash } from "./utils";

  interface GrantRow { privilege: string; peer_id: string }

  // For now the only grantable privilege is blob-list; extend when the daemon exposes more.
  const PRIVILEGES = ["blob-list"];

  let grants: GrantRow[] = $state([]);
  let error: string | null = $state(null);
  let adding = $state(false);
  let newPeerId = $state("");
  let newPrivilege = $state(PRIVILEGES[0]);
  let revokingKey: string | null = $state(null); // "<peer_id>:<privilege>"

  function rowKey(r: GrantRow) { return `${r.peer_id}:${r.privilege}`; }

  async function load() {
    error = null;
    try {
      grants = await invoke<GrantRow[]>("grant_list", { peer: null, privilege: null });
    } catch (e) {
      error = String(e);
    }
  }

  async function grantPrivilege() {
    const peer = newPeerId.trim();
    if (!peer) return;
    error = null;
    try {
      await invoke("grant_grant", { peer, privilege: newPrivilege });
      newPeerId = "";
      adding = false;
      await load();
    } catch (e) {
      error = String(e);
    }
  }

  async function revokePrivilege(peer: string, privilege: string) {
    error = null;
    try {
      await invoke("grant_revoke", { peer, privilege });
      revokingKey = null;
      grants = grants.filter((g) => !(g.peer_id === peer && g.privilege === privilege));
    } catch (e) {
      error = String(e);
    }
  }

  $effect(() => { load(); });
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h2 class="text-xs font-semibold uppercase tracking-widest text-neutral-500">Grants</h2>
    <button
      onclick={() => { adding = true; newPeerId = ""; newPrivilege = PRIVILEGES[0]; }}
      class="rounded border border-amber-700/60 bg-amber-950/40 px-2.5 py-1 text-xs font-medium text-amber-300 transition-colors hover:border-amber-500 hover:bg-amber-900/40"
    >Grant access</button>
  </div>

  {#if error}
    <p class="rounded border border-red-900/50 bg-red-950/30 px-3 py-2 text-xs text-red-400">{error}</p>
  {/if}

  <!-- Grant form -->
  {#if adding}
    <div class="flex flex-col gap-2 rounded border border-neutral-800 bg-neutral-900/50 p-3">
      <div class="flex gap-2">
        <input
          type="text"
          placeholder="Base32 peer ID"
          bind:value={newPeerId}
          class="min-w-0 flex-1 rounded border border-neutral-800 bg-neutral-900 px-3 py-1.5 text-xs text-neutral-100 outline-none focus:border-amber-700"
          aria-label="Peer ID to grant"
        />
        <select
          bind:value={newPrivilege}
          class="rounded border border-neutral-800 bg-neutral-900 px-2 py-1.5 text-xs text-neutral-300 outline-none focus:border-amber-700"
          aria-label="Privilege"
        >
          {#each PRIVILEGES as p}
            <option value={p}>{p}</option>
          {/each}
        </select>
      </div>
      <div class="flex gap-2">
        <button
          onclick={grantPrivilege}
          class="rounded border border-amber-700/60 bg-amber-950/40 px-3 py-1.5 text-xs text-amber-300 transition-colors hover:border-amber-500"
        >Grant</button>
        <button onclick={() => (adding = false)} class="text-xs text-neutral-600 hover:text-neutral-400">Cancel</button>
      </div>
    </div>
  {/if}

  <!-- Grant table -->
  <div class="overflow-x-auto">
    <table class="w-full text-sm">
      <thead>
        <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
          <th class="pb-2 text-left font-medium">Privilege</th>
          <th class="pb-2 text-left font-medium">Peer ID</th>
          <th class="pb-2 text-right font-medium"></th>
        </tr>
      </thead>
      <tbody>
        {#if grants.length === 0}
          <tr>
            <td colspan="3" class="py-10 text-center text-sm italic text-neutral-700">
              No grants. Use <span class="text-amber-600">Grant access</span> to let a peer query your blob list.
            </td>
          </tr>
        {/if}
        {#each grants as g (rowKey(g))}
          <tr class="border-b border-neutral-900 hover:bg-neutral-900/50">
            <td class="py-2.5 pr-4">
              <span class="rounded border border-amber-900/50 bg-amber-950/30 px-1.5 py-0.5 text-xs text-amber-400">{g.privilege}</span>
            </td>
            <td class="py-2.5 pr-4 font-mono text-xs text-neutral-500">
              {truncateHash(g.peer_id, 16)}
            </td>
            <td class="py-2.5 text-right">
              {#if revokingKey === rowKey(g)}
                <span class="mr-2 text-xs text-neutral-400">Revoke?</span>
                <button onclick={() => revokePrivilege(g.peer_id, g.privilege)} class="mr-2 text-xs text-red-400 hover:text-red-300">Yes</button>
                <button onclick={() => (revokingKey = null)} class="text-xs text-neutral-600 hover:text-neutral-400">No</button>
              {:else}
                <button
                  onclick={() => (revokingKey = rowKey(g))}
                  class="text-neutral-600 transition-colors hover:text-red-500"
                  aria-label="Revoke {g.privilege} from {g.peer_id}"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="inline h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
                  </svg>
                </button>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
