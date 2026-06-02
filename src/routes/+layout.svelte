<script lang="ts">
  import "../app.css";
  import DaemonBadge from "$lib/DaemonBadge.svelte";

  let { children } = $props();
</script>

<div class="flex h-screen flex-col bg-neutral-950 text-neutral-100">
  <header class="flex items-center justify-between border-b border-neutral-800 px-4 py-2">
    <span class="text-sm font-semibold tracking-widest text-neutral-400 uppercase">ringdrop</span>
    <DaemonBadge />
  </header>

  <main class="flex-1 overflow-auto p-6">
    {@render children()}
  </main>
</div>
