import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, cleanup, fireEvent } from "@testing-library/svelte";
import BlobTable from "./BlobTable.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const rows = [
  { hash: "aaaa1111bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222", name: "photo.jpg", rings: ["friends"], ticket: "rdrop://aaaa" },
  { hash: "bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222cccc3333", name: "doc.pdf",   rings: [],          ticket: "rdrop://bbbb" },
];


describe("BlobTable", () => {
  beforeEach(() => {
    Object.assign(navigator, { clipboard: { writeText: vi.fn().mockResolvedValue(undefined) } });
  });

  it("renders a row per blob", () => {
    const { getAllByRole } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    // header row + 2 data rows
    expect(getAllByRole("row").length).toBe(3);
  });

  it("shows blob name", () => {
    const { getByText } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    expect(getByText("photo.jpg")).toBeTruthy();
  });

  it("shows truncated hash", () => {
    const { getByText } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    expect(getByText("aaaa1111bbbb…")).toBeTruthy();
  });

  it("shows ring badge", () => {
    const { getByText } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    expect(getByText("friends")).toBeTruthy();
  });

  it("shows untagged label when rings is empty", () => {
    const { getByText } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    expect(getByText("untagged")).toBeTruthy();
  });

  it("shows empty state when rows is empty", () => {
    const { getByText } = render(BlobTable, { props: { rows: [], onDelete: vi.fn() } });
    expect(getByText("No blobs in local store.")).toBeTruthy();
  });

  it("copies ticket to clipboard on copy button click", async () => {
    const { getAllByLabelText } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    await fireEvent.click(getAllByLabelText("Copy ticket")[0]);
    expect(navigator.clipboard.writeText).toHaveBeenCalledWith("rdrop://aaaa");
  });

  it("shows delete confirmation on delete button click", async () => {
    const { getAllByLabelText, findByText } = render(BlobTable, { props: { rows, onDelete: vi.fn() } });
    await fireEvent.click(getAllByLabelText("Delete blob")[0]);
    expect(await findByText("Delete?")).toBeTruthy();
  });

  it("calls onDelete with the correct hash on confirm", async () => {
    const onDelete = vi.fn();
    const { getAllByLabelText, findByText } = render(BlobTable, { props: { rows, onDelete } });
    await fireEvent.click(getAllByLabelText("Delete blob")[0]);
    await fireEvent.click(await findByText("Yes"));
    expect(onDelete).toHaveBeenCalledWith(rows[0].hash);
  });

  it("cancels delete without calling onDelete", async () => {
    const onDelete = vi.fn();
    const { getAllByLabelText, findByText } = render(BlobTable, { props: { rows, onDelete } });
    await fireEvent.click(getAllByLabelText("Delete blob")[0]);
    await fireEvent.click(await findByText("No"));
    expect(onDelete).not.toHaveBeenCalled();
  });
});
