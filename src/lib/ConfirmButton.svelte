<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Text shown in the confirmation prompt, e.g. `"Delete?"`. */
    label?: string;
    /** Called when the user clicks Yes. */
    onConfirm: () => void;
    children: Snippet;
  }

  let { label = "Sure?", onConfirm, children }: Props = $props();

  let confirming = $state(false);
</script>

{#if confirming}
  <span class="mr-2 text-xs text-neutral-400">{label}</span>
  <button
    onclick={() => { onConfirm(); confirming = false; }}
    class="mr-2 text-xs text-red-400 hover:text-red-300"
  >Yes</button>
  <button
    onclick={() => (confirming = false)}
    class="text-xs text-neutral-600 hover:text-neutral-400"
  >No</button>
{:else}
  <!-- display:contents removes this span from the layout flow so callers don't
       need to wire a trigger themselves — any click on the child enters confirm mode. -->
  <span
    role="button"
    tabindex="0"
    onclick={() => (confirming = true)}
    onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); confirming = true; } }}
    style="display:contents"
  >
    {@render children()}
  </span>
{/if}
