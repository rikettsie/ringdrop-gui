import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
import GrantPanel from "./GrantPanel.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const { invoke } = await import("@tauri-apps/api/core");
const mockInvoke = vi.mocked(invoke);

const TEST_GRANTS = [
  { privilege: "blob-list", peer_id: "aaaa1111bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222" },
];


describe("GrantPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "grant_list") return TEST_GRANTS;
      return undefined;
    });
  });

  it("renders grant list", async () => {
    const { findByText } = render(GrantPanel);
    expect(await findByText("blob-list")).toBeTruthy();
  });

  it("shows empty state when no grants", async () => {
    mockInvoke.mockResolvedValue([]);
    const { findByText } = render(GrantPanel);
    expect(await findByText(/No grants/)).toBeTruthy();
  });

  it("opens grant form on button click", async () => {
    const { findByText, findByLabelText } = render(GrantPanel);
    await fireEvent.click(await findByText("Grant access"));
    expect(await findByLabelText("Peer ID to grant")).toBeTruthy();
  });

  it("calls grant_grant with peer and privilege", async () => {
    const { findByText, findByLabelText } = render(GrantPanel);
    await fireEvent.click(await findByText("Grant access"));
    await fireEvent.input(await findByLabelText("Peer ID to grant"), { target: { value: "newpeer" } });
    await fireEvent.click(await findByText("Grant"));
    expect(mockInvoke).toHaveBeenCalledWith("grant_grant", { peer: "newpeer", privilege: "blob-list" });
  });

  it("shows revoke confirmation on revoke button click", async () => {
    const { findByText, findByLabelText } = render(GrantPanel);
    await fireEvent.click(await findByLabelText(`Revoke blob-list from ${TEST_GRANTS[0].peer_id}`));
    expect(await findByText("Revoke?")).toBeTruthy();
  });

  it("calls grant_revoke on confirm", async () => {
    const { findByText, findByLabelText } = render(GrantPanel);
    await fireEvent.click(await findByLabelText(`Revoke blob-list from ${TEST_GRANTS[0].peer_id}`));
    await fireEvent.click(await findByText("Yes"));
    expect(mockInvoke).toHaveBeenCalledWith("grant_revoke", {
      peer: TEST_GRANTS[0].peer_id,
      privilege: "blob-list",
    });
  });
});
