import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
import IdPanel from "./IdPanel.svelte";

const TEST_PEER_ID = "5pbfvz7tqd4k7nqvxuwfjpxhcb4nvn2m7kmpqbymjjbfklimzjrq";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// qrcode uses canvas internally; stub it so tests don't need a real canvas.
vi.mock("qrcode", () => ({
  default: {
    toString: vi.fn().mockResolvedValue("<svg><rect/></svg>"),
  },
}));

const { invoke } = await import("@tauri-apps/api/core");
const mockInvoke = vi.mocked(invoke);

afterEach(() => cleanup());

describe("IdPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockInvoke.mockResolvedValue(TEST_PEER_ID);
    Object.assign(navigator, {
      clipboard: { writeText: vi.fn().mockResolvedValue(undefined) },
    });
  });

  it("displays the peer ID", async () => {
    const { findByText } = render(IdPanel);
    expect(await findByText(TEST_PEER_ID)).toBeTruthy();
  });

  it("renders a QR code element", async () => {
    const { findByLabelText } = render(IdPanel);
    expect(await findByLabelText("QR code for peer ID")).toBeTruthy();
  });

  it("copies peer ID on copy button click", async () => {
    const { findByText } = render(IdPanel);
    await fireEvent.click(await findByText("Copy"));
    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(TEST_PEER_ID);
  });

  it("shows Copied! feedback after copying", async () => {
    const { findByText } = render(IdPanel);
    await fireEvent.click(await findByText("Copy"));
    expect(await findByText("Copied!")).toBeTruthy();
  });

  it("shows error when invoke fails", async () => {
    mockInvoke.mockRejectedValue(new Error("daemon offline"));
    const { findByText } = render(IdPanel);
    expect(await findByText(/daemon offline/)).toBeTruthy();
  });
});
