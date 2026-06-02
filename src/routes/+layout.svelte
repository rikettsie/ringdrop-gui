<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import DaemonBadge from "$lib/DaemonBadge.svelte";

  let { children } = $props();

  const nav = [
    { href: "/",       label: "Blobs"   },
    { href: "/receive", label: "Receive" },
    { href: "/id",      label: "ID"      },
    { href: "/rings",   label: "Rings"   },
    { href: "/peers",   label: "Peers"   },
    { href: "/grants",  label: "Grants"  },
    { href: "/remote",  label: "Remote"  },
  ];
</script>

<div class="flex h-screen flex-col bg-neutral-950 text-neutral-100">
  <!-- Top bar -->
  <header class="flex shrink-0 items-center justify-between border-b border-neutral-800/60 px-4 py-2">
    <span class="text-xs font-semibold uppercase tracking-widest text-neutral-500">ringdrop</span>
    <DaemonBadge />
  </header>

  <div class="flex min-h-0 flex-1">
    <!-- Sidebar -->
    <nav class="flex w-40 shrink-0 flex-col gap-0.5 border-r border-neutral-800/60 py-3 px-2">
      {#each nav as item}
        <a
          href={item.href}
          class="rounded px-3 py-1.5 text-sm transition-colors
            {$page.url.pathname === item.href
              ? 'bg-amber-950/60 text-amber-300 font-medium'
              : 'text-neutral-500 hover:bg-neutral-900 hover:text-neutral-200'}"
        >{item.label}</a>
      {/each}
    </nav>

    <!-- Page content -->
    <main class="min-w-0 flex-1 overflow-auto p-6">
      {@render children()}
    </main>
  </div>
</div>
