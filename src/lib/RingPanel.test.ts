import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
import RingPanel from "./RingPanel.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const { invoke } = await import("@tauri-apps/api/core");
const mockInvoke = vi.mocked(invoke);

const TEST_RINGS = [
  { name: "friends", open: false },
  { name: "open",    open: true  },
];
const TEST_MEMBERS = [
  { peer_id: "aaaa1111bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222", nickname: "alice" },
  { peer_id: "bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222cccc3333", nickname: null  },
];

afterEach(() => cleanup());

describe("RingPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "ring_list")    return TEST_RINGS;
      if (cmd === "ring_members") return TEST_MEMBERS;
      return undefined;
    });
  });

  it("renders ring names from ring_list", async () => {
    const { findByText } = render(RingPanel);
    expect(await findByText("friends")).toBeTruthy();
    expect(await findByText("open")).toBeTruthy();
  });

  it("marks the open ring with a pub badge", async () => {
    const { findByText } = render(RingPanel);
    expect(await findByText("pub")).toBeTruthy();
  });

  it("shows placeholder when no ring is selected", async () => {
    const { findByText } = render(RingPanel);
    expect(await findByText("Select a ring to manage its members.")).toBeTruthy();
  });

  it("loads members when a ring is clicked", async () => {
    const { findByText } = render(RingPanel);
    await fireEvent.click(await findByText("friends"));
    expect(await findByText("alice")).toBeTruthy();
  });

  it("shows inline create form on + click", async () => {
    const { findByLabelText, findByRole } = render(RingPanel);
    await fireEvent.click(await findByLabelText("New ring"));
    expect(await findByLabelText("New ring name")).toBeTruthy();
  });

  it("calls ring_create and reloads on confirm", async () => {
    const { findByLabelText } = render(RingPanel);
    await fireEvent.click(await findByLabelText("New ring"));
    const input = await findByLabelText("New ring name") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "work" } });
    await fireEvent.keyDown(input, { key: "Enter" });
    expect(mockInvoke).toHaveBeenCalledWith("ring_create", { name: "work" });
  });

  it("shows add-peer form on Add peer click", async () => {
    const { findByText, findByLabelText } = render(RingPanel);
    await fireEvent.click(await findByText("friends"));
    await fireEvent.click(await findByText("Add peer"));
    expect(await findByLabelText("Peer ID to add")).toBeTruthy();
  });

  it("calls ring_add when peer is submitted", async () => {
    const { findByText, findByLabelText } = render(RingPanel);
    await fireEvent.click(await findByText("friends"));
    await fireEvent.click(await findByText("Add peer"));
    const input = await findByLabelText("Peer ID to add") as HTMLInputElement;
    await fireEvent.input(input, { target: { value: "newpeer123" } });
    await fireEvent.click(await findByText("Add"));
    expect(mockInvoke).toHaveBeenCalledWith("ring_add", { ring: "friends", peer: "newpeer123" });
  });

  it("shows remove confirmation on remove button click", async () => {
    const { findByText, findByLabelText } = render(RingPanel);
    await fireEvent.click(await findByText("friends"));
    await fireEvent.click(await findByLabelText("Remove alice from ring"));
    expect(await findByText("Remove?")).toBeTruthy();
  });

  it("calls ring_remove on confirm", async () => {
    const { findByText, findByLabelText } = render(RingPanel);
    await fireEvent.click(await findByText("friends"));
    await fireEvent.click(await findByLabelText("Remove alice from ring"));
    await fireEvent.click(await findByText("Yes"));
    expect(mockInvoke).toHaveBeenCalledWith("ring_remove", {
      ring: "friends",
      peer: TEST_MEMBERS[0].peer_id,
    });
  });
});
