import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
import PeerPanel from "./PeerPanel.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const { invoke } = await import("@tauri-apps/api/core");
const mockInvoke = vi.mocked(invoke);

const TEST_PEERS = [
  { peer_id: "aaaa1111bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222", nickname: "alice" },
  { peer_id: "bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222cccc3333", nickname: null  },
];


describe("PeerPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "peer_list") return TEST_PEERS;
      return undefined;
    });
  });

  it("renders peer list", async () => {
    const { findByText } = render(PeerPanel);
    expect(await findByText("alice")).toBeTruthy();
  });

  it("shows empty state when no peers", async () => {
    mockInvoke.mockResolvedValue([]);
    const { findByText } = render(PeerPanel);
    expect(await findByText("No peers in address book.")).toBeTruthy();
  });

  it("opens add-peer form on button click", async () => {
    const { findByText, findByLabelText } = render(PeerPanel);
    await fireEvent.click(await findByText("Add peer"));
    expect(await findByLabelText("Peer ID")).toBeTruthy();
    expect(await findByLabelText("Nickname")).toBeTruthy();
  });

  it("calls peer_add with id and nickname", async () => {
    const { findByText, findByLabelText } = render(PeerPanel);
    await fireEvent.click(await findByText("Add peer"));
    await fireEvent.input(await findByLabelText("Peer ID"),   { target: { value: "newpeer" } });
    await fireEvent.input(await findByLabelText("Nickname"),  { target: { value: "bob" } });
    await fireEvent.click(await findByText("Add"));
    expect(mockInvoke).toHaveBeenCalledWith("peer_add", { peer: "newpeer", nickname: "bob" });
  });

  it("calls peer_add with null nickname when omitted", async () => {
    const { findByText, findByLabelText } = render(PeerPanel);
    await fireEvent.click(await findByText("Add peer"));
    await fireEvent.input(await findByLabelText("Peer ID"), { target: { value: "newpeer" } });
    await fireEvent.click(await findByText("Add"));
    expect(mockInvoke).toHaveBeenCalledWith("peer_add", { peer: "newpeer", nickname: null });
  });

  it("shows remove confirmation on delete click", async () => {
    const { findByText, findByLabelText } = render(PeerPanel);
    await fireEvent.click(await findByLabelText("Remove alice"));
    expect(await findByText("Remove?")).toBeTruthy();
  });

  it("calls peer_remove on confirm", async () => {
    const { findByText, findByLabelText } = render(PeerPanel);
    await fireEvent.click(await findByLabelText("Remove alice"));
    await fireEvent.click(await findByText("Yes"));
    expect(mockInvoke).toHaveBeenCalledWith("peer_remove", { peer: TEST_PEERS[0].peer_id });
  });

  it("cancels remove without calling peer_remove", async () => {
    const { findByText, findByLabelText } = render(PeerPanel);
    await fireEvent.click(await findByLabelText("Remove alice"));
    await fireEvent.click(await findByText("No"));
    expect(mockInvoke).not.toHaveBeenCalledWith("peer_remove", expect.anything());
  });
});
