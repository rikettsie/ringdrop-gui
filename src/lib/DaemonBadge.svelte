<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type Status = "connected" | "disconnected" | "unconfigured";

  const label: Record<Status, string> = {
    connected: "connected",
    disconnected: "no daemon",
    unconfigured: "not configured",
  };

  const dot: Record<Status, string> = {
    connected: "bg-emerald-400",
    disconnected: "bg-red-500",
    unconfigured: "bg-neutral-500",
  };

  let status: Status = $state("unconfigured");

  async function poll() {
    try {
      const res = await invoke<{ running: boolean; port: number | null }>(
        "daemon_status",
      );
      status = res.running ? "connected" : res.port !== null ? "disconnected" : "unconfigured";
    } catch {
      status = "unconfigured";
    }
  }

  $effect(() => {
    poll();
    const id = setInterval(poll, 3000);
    return () => clearInterval(id);
  });
</script>

<div class="flex items-center gap-2 text-xs text-neutral-400">
  <span class="h-2 w-2 rounded-full {dot[status]}" aria-hidden="true"></span>
  <span>{label[status]}</span>
</div>
