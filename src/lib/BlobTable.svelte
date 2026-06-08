<script lang="ts">
  import type { BlobRow, RingRow } from "./types";
  import ConfirmButton from "./ConfirmButton.svelte";

  interface Props {
    rows: BlobRow[];
    rings: RingRow[];
    onDelete: (hash: string) => void;
    onAttach: (hash: string, ring: RingRow) => void;
    onDetach: (hash: string, ring: RingRow) => void;
  }

  let { rows, rings, onDelete, onAttach, onDetach }: Props = $props();

  let attachingHash: string | null = $state(null);

  async function copyTicket(ticket: string) {
    await navigator.clipboard.writeText(ticket);
  }

  function findRing(name: string): RingRow {
    return rings.find((r) => r.name === name) ?? { name, open: false };
  }

  function attachableRings(row: BlobRow): RingRow[] {
    return rings.filter((r) => !row.rings.includes(r.name));
  }
</script>

<div class="overflow-x-auto">
  <table class="table-fixed w-full text-sm">
    <thead>
      <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
        <th class="w-28 pb-2 text-left font-medium">Name</th>
        <th class="pb-2 text-left font-medium">Hash</th>
        <th class="w-44 pb-2 text-left font-medium">Rings</th>
        <th class="w-16 pb-2 text-right font-medium"></th>
      </tr>
    </thead>
    <tbody>
      {#if rows.length === 0}
        <tr>
          <td colspan="4" class="py-10 text-center text-neutral-600 italic">
            No blobs in local store.
          </td>
        </tr>
      {/if}
      {#each rows as row (row.hash)}
        <tr class="group border-b border-neutral-900 transition-colors hover:bg-neutral-900/60">
          <td class="overflow-hidden break-words py-2.5 pr-4 text-neutral-100">{row.name}</td>
          <td class="max-w-0 py-2.5 pr-4">
            <span class="block truncate font-mono text-xs text-neutral-500" title={row.hash}>{row.hash}</span>
          </td>
          <td class="py-2.5 pr-4">
            {#if row.rings.length === 0 && attachingHash !== row.hash}
              <span class="text-xs italic text-neutral-700">untagged</span>
            {/if}
            {#each row.rings as ringName}
              <span class="mr-1 inline-flex items-center gap-0.5 rounded border border-amber-900/50 bg-amber-950/30 px-1.5 py-0.5 text-xs text-amber-400">
                {ringName}
                <button
                  onclick={() => onDetach(row.hash, findRing(ringName))}
                  class="leading-none hover:text-red-400"
                  aria-label="Detach from {ringName}"
                >×</button>
              </span>
            {/each}
            {#if attachingHash === row.hash}
              <select
                class="rounded border border-amber-900/50 bg-neutral-900 px-1 py-0.5 text-xs text-amber-400"
                onchange={(e) => {
                  const ring = attachableRings(row).find((r) => r.name === e.currentTarget.value);
                  if (ring) { onAttach(row.hash, ring); }
                  attachingHash = null;
                }}
              >
                <option value="">pick ring…</option>
                {#each attachableRings(row) as ring}
                  <option value={ring.name}>{ring.name}</option>
                {/each}
              </select>
              <button
                onclick={() => (attachingHash = null)}
                class="ml-1 text-xs text-neutral-600 hover:text-neutral-400"
                aria-label="Cancel"
              >×</button>
            {:else if attachableRings(row).length > 0}
              <button
                onclick={() => (attachingHash = row.hash)}
                class="text-xs text-neutral-600 hover:text-amber-400"
                aria-label="Attach to ring"
                title="Attach to ring"
              >+</button>
            {/if}
          </td>
          <td class="py-2.5 text-right">
            <button
              onclick={() => copyTicket(row.ticket)}
              title="Copy share ticket"
              class="mr-2 text-neutral-600 transition-colors hover:text-amber-400"
              aria-label="Copy ticket"
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="inline h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-4 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
            </button>
            <ConfirmButton label="Delete?" onConfirm={() => onDelete(row.hash)}>
              <button
                title="Delete blob"
                class="text-neutral-600 transition-colors hover:text-red-500"
                aria-label="Delete blob"
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
