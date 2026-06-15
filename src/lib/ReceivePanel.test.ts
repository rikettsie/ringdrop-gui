import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import ReceivePanel from "./ReceivePanel.svelte";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn().mockResolvedValue("/home/user/downloads"),
}));

const { invoke } = await import("@tauri-apps/api/core");
const { listen } = await import("@tauri-apps/api/event");
const { open: openDialog } = await import("@tauri-apps/plugin-dialog");
const mockInvoke = vi.mocked(invoke);
const mockListen = vi.mocked(listen);
const mockOpen = vi.mocked(openDialog);


describe("ReceivePanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockResolvedValue(undefined);
    mockOpen.mockResolvedValue("/home/user/downloads");
  });

  it("renders ticket input and destination picker", () => {
    const { getByPlaceholderText, getByText } = render(ReceivePanel);
    expect(getByPlaceholderText("rdrop://…")).toBeTruthy();
    expect(getByText("Browse")).toBeTruthy();
  });

  it("download button is disabled when ticket is empty", () => {
    const { getByText } = render(ReceivePanel);
    const btn = getByText("Download") as HTMLButtonElement;
    expect(btn.disabled).toBe(true);
  });

  it("download button is disabled when only ticket is filled", async () => {
    const { getByPlaceholderText, getByText } = render(ReceivePanel);
    await fireEvent.input(getByPlaceholderText("rdrop://…"), {
      target: { value: "rdrop://abc" },
    });
    const btn = getByText("Download") as HTMLButtonElement;
    expect(btn.disabled).toBe(true);
  });

  it("populates destination after browse", async () => {
    const { getByText, findByDisplayValue } = render(ReceivePanel);
    await fireEvent.click(getByText("Browse"));
    expect(await findByDisplayValue("/home/user/downloads")).toBeTruthy();
  });

  it("shows success message after download completes", async () => {
    const { getByPlaceholderText, getByText, findByText } = render(ReceivePanel);
    await fireEvent.input(getByPlaceholderText("rdrop://…"), {
      target: { value: "rdrop://abc" },
    });
    await fireEvent.click(getByText("Browse"));
    await fireEvent.click(await findByText("Download"));
    expect(await findByText("Download complete.")).toBeTruthy();
  });

  it("shows error message when invoke rejects", async () => {
    mockInvoke.mockRejectedValueOnce(new Error("connection refused"));
    const { getByPlaceholderText, getByText, findByText } = render(ReceivePanel);
    await fireEvent.input(getByPlaceholderText("rdrop://…"), {
      target: { value: "rdrop://abc" },
    });
    await fireEvent.click(getByText("Browse"));
    await fireEvent.click(await findByText("Download"));
    expect(await findByText(/connection refused/)).toBeTruthy();
  });

  it("shows file name when a FileProgress event arrives", async () => {
    // Capture the progress callback so we can fire it manually.
    let progressCb: ((ev: { payload: Record<string, unknown> }) => void) | null = null;
    mockListen.mockImplementationOnce(async (_channel, cb) => {
      progressCb = cb as typeof progressCb;
      return () => {};
    });
    // Keep the download pending so we can inspect the in-flight state.
    let finishDownload!: () => void;
    mockInvoke.mockImplementationOnce(() => new Promise<void>(res => { finishDownload = res; }));

    const { getByPlaceholderText, getByText, findByText } = render(ReceivePanel);
    await fireEvent.input(getByPlaceholderText("rdrop://…"), { target: { value: "rdrop://abc" } });
    await fireEvent.click(getByText("Browse"));
    await fireEvent.click(await findByText("Download"));

    // Flush microtasks so `await listen(...)` has resolved and the callback is captured.
    await new Promise(r => setTimeout(r, 0));
    expect(progressCb).not.toBeNull();

    progressCb!({ payload: { done: 512, total: 1024, file_index: 2, file_total: 5, file_name: "notes.txt" } });

    expect(await findByText(/notes\.txt/)).toBeTruthy();
    expect(await findByText(/2\/5/)).toBeTruthy();

    finishDownload();
  });

  it("does not show file name line when no FileProgress has arrived", async () => {
    let finishDownload!: () => void;
    mockInvoke.mockImplementationOnce(() => new Promise<void>(res => { finishDownload = res; }));

    const { getByPlaceholderText, getByText, findByText, queryByText } = render(ReceivePanel);
    await fireEvent.input(getByPlaceholderText("rdrop://…"), { target: { value: "rdrop://abc" } });
    await fireEvent.click(getByText("Browse"));
    await fireEvent.click(await findByText("Download"));

    await new Promise(r => setTimeout(r, 0));
    expect(queryByText(/File \d+\/\d+/)).toBeNull();

    finishDownload();
  });
});
