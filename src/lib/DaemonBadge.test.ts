import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup } from "@testing-library/svelte";
import DaemonBadge from "./DaemonBadge.svelte";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// Import after mock so we get the mocked version.
const { invoke } = await import("@tauri-apps/api/core");
const mockInvoke = vi.mocked(invoke);

describe("DaemonBadge", () => {
  afterEach(() => cleanup());

  it("shows connected when daemon is running", async () => {
    mockInvoke.mockResolvedValue({ running: true, port: 7070 });
    const { findByText } = render(DaemonBadge);
    expect(await findByText("connected")).toBeTruthy();
  });

  it("shows no daemon when configured but not running", async () => {
    mockInvoke.mockResolvedValue({ running: false, port: 7070 });
    const { findByText } = render(DaemonBadge);
    expect(await findByText("no daemon")).toBeTruthy();
  });

  it("shows not configured when port is null", async () => {
    mockInvoke.mockResolvedValue({ running: false, port: null });
    const { findByText } = render(DaemonBadge);
    expect(await findByText("not configured")).toBeTruthy();
  });

  it("falls back to not configured on invoke error", async () => {
    mockInvoke.mockRejectedValue(new Error("IPC error"));
    const { findByText } = render(DaemonBadge);
    expect(await findByText("not configured")).toBeTruthy();
  });
});
