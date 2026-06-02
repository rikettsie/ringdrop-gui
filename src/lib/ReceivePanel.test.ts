import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
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
const { open: openDialog } = await import("@tauri-apps/plugin-dialog");
const mockInvoke = vi.mocked(invoke);
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
});
