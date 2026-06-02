<script lang="ts">
  import { truncateHash } from "./utils";

  interface BlobRow {
    hash: string;
    name: string;
    rings: string[];
    ticket: string;
  }

  interface Props {
    rows: BlobRow[];
    onDelete: (hash: string) => void;
  }

  let { rows, onDelete }: Props = $props();

  let confirming: string | null = $state(null);

  async function copyTicket(ticket: string) {
    await navigator.clipboard.writeText(ticket);
  }
</script>

<div class="overflow-x-auto">
  <table class="w-full text-sm">
    <thead>
      <tr class="border-b border-amber-900/40 text-xs uppercase tracking-wider text-neutral-500">
        <th class="pb-2 text-left font-medium">Name</th>
        <th class="pb-2 text-left font-medium">Hash</th>
        <th class="pb-2 text-left font-medium">Rings</th>
        <th class="pb-2 text-right font-medium"></th>
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
          <td class="py-2.5 pr-4 text-neutral-100">{row.name}</td>
          <td class="py-2.5 pr-4 font-mono text-xs text-neutral-500">
            {truncateHash(row.hash, 12)}
          </td>
          <td class="py-2.5 pr-4">
            {#if row.rings.length === 0}
              <span class="text-xs italic text-neutral-700">untagged</span>
            {:else}
              {#each row.rings as ring}
                <span
                  class="mr-1 rounded border border-amber-900/50 bg-amber-950/30 px-1.5 py-0.5 text-xs text-amber-400"
                >{ring}</span>
              {/each}
            {/if}
          </td>
          <td class="py-2.5 text-right">
            {#if confirming === row.hash}
              <span class="mr-2 text-xs text-neutral-400">Delete?</span>
              <button
                onclick={() => { onDelete(row.hash); confirming = null; }}
                class="mr-2 text-xs text-red-400 hover:text-red-300"
              >Yes</button>
              <button
                onclick={() => (confirming = null)}
                class="text-xs text-neutral-600 hover:text-neutral-300"
              >No</button>
            {:else}
              <!-- Copy ticket -->
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
              <!-- Delete -->
              <button
                onclick={() => (confirming = row.hash)}
                title="Delete blob"
                class="text-neutral-600 transition-colors hover:text-red-500"
                aria-label="Delete blob"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="inline h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
