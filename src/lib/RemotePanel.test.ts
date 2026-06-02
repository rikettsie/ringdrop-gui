import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
import RemotePanel from "./RemotePanel.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/event", () => ({ listen: vi.fn().mockResolvedValue(() => {}) }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn().mockResolvedValue("/tmp") }));

const { invoke } = await import("@tauri-apps/api/core");
const mockInvoke = vi.mocked(invoke);

const TEST_BLOBS = [
  { hash: "aaaa1111bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222", name: "movie.mp4", ticket: "rdrop://aaaa" },
  { hash: "bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222cccc3333", name: "photo.jpg", ticket: "rdrop://bbbb" },
];


describe("RemotePanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "remote_blob_list") return TEST_BLOBS;
      return undefined;
    });
  });

  it("renders peer ID input and Browse button", () => {
    const { getByLabelText, getByText } = render(RemotePanel);
    expect(getByLabelText("Remote peer ID")).toBeTruthy();
    expect(getByText("Browse")).toBeTruthy();
  });

  it("Browse button is disabled when peer ID is empty", () => {
    const { getByText } = render(RemotePanel);
    expect((getByText("Browse") as HTMLButtonElement).disabled).toBe(true);
  });

  it("calls remote_blob_list with the peer ID", async () => {
    const { getByLabelText, getByText } = render(RemotePanel);
    await fireEvent.input(getByLabelText("Remote peer ID"), { target: { value: "remotepeer" } });
    await fireEvent.click(getByText("Browse"));
    expect(mockInvoke).toHaveBeenCalledWith("remote_blob_list", { peer: "remotepeer" });
  });

  it("renders blob rows after fetch", async () => {
    const { getByLabelText, getByText, findByText } = render(RemotePanel);
    await fireEvent.input(getByLabelText("Remote peer ID"), { target: { value: "remotepeer" } });
    await fireEvent.click(getByText("Browse"));
    expect(await findByText("movie.mp4")).toBeTruthy();
    expect(await findByText("photo.jpg")).toBeTruthy();
  });

  it("calls receive with the correct ticket on download", async () => {
    const { getByLabelText, getByText, findAllByLabelText } = render(RemotePanel);
    await fireEvent.input(getByLabelText("Remote peer ID"), { target: { value: "remotepeer" } });
    await fireEvent.click(getByText("Browse"));
    const btns = await findAllByLabelText(/^Download /);
    await fireEvent.click(btns[0]);
    expect(mockInvoke).toHaveBeenCalledWith("receive", {
      ticket: TEST_BLOBS[0].ticket,
      dest: "/tmp",
    });
  });
});
