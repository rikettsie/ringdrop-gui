<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/stores";
  import DaemonBadge from "$lib/DaemonBadge.svelte";

  let { children } = $props();

  let versions: { gui: string; daemon: string } | null = $state(null);
  let cliVersion: string | null = $state(null);

  // major.minor must match — pre-1.0 minor bumps can be breaking
  function compatibleVersions(compiled: string, cli: string): boolean {
    const [cMaj, cMin] = compiled.split(".").map(Number);
    const [rMaj, rMin] = cli.split(".").map(Number);
    return cMaj === rMaj && cMin === rMin;
  }

  let versionMismatch = $derived.by(() => {
    if (!versions || !cliVersion) return false;
    return !compatibleVersions(versions.daemon, cliVersion);
  });

  $effect(() => {
    Promise.all([
      invoke<{ gui: string; daemon: string }>("app_versions"),
      invoke<string>("rdrop_cli_version").catch(() => null),
    ])
      .then(([v, cli]) => {
        versions = v;
        cliVersion = cli;
      })
      .catch(() => {});
  });

  const nav = [
    { href: "/",       label: "Blobs"   },
    { href: "/receive", label: "Receive" },
    { href: "/id",      label: "Your ID" },
    { href: "/rings",   label: "Rings"   },
    { href: "/peers",   label: "Peers"   },
    { href: "/grants",  label: "Grants"  },
    { href: "/remote",  label: "Remote"  },
  ];
</script>

<div class="flex h-screen flex-col bg-neutral-950 text-neutral-100">
  <!-- Top bar -->
  <header class="flex shrink-0 items-center justify-between border-b border-neutral-800/60 px-4 py-2">
    <div class="flex items-center gap-2.5">
      <div class="h-9 w-9 overflow-hidden">
        <img src="/mascot.png" alt="ringdrop mascot" class="h-9 w-9 translate-y-1 scale-[1.35] object-contain" />
      </div>
      <div class="flex flex-col leading-tight">
        <span class="text-sm font-semibold text-amber-400">ringdrop</span>
        <span class="text-xs text-neutral-600">gui v{versions?.gui ?? '…'}</span>
        <span class="text-xs text-neutral-600">daemon v{versions?.daemon ?? '…'}</span>
      </div>
    </div>
    <DaemonBadge />
  </header>

  {#if versionMismatch}
    <div class="shrink-0 border-b border-amber-900/60 bg-amber-950/40 px-4 py-1.5 text-xs text-amber-400">
      Version mismatch: GUI compiled against ringdrop {versions!.daemon}, but <code class="font-mono">rdrop --version</code> reports {cliVersion}. Some features may not work correctly.
    </div>
  {/if}

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
