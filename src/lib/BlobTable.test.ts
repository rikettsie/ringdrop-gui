import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, fireEvent } from "@testing-library/svelte";
import BlobTable from "./BlobTable.svelte";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn() }));

const rows = [
  { hash: "aaaa1111bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222", name: "photo.jpg", rings: ["friends"], ticket: "rdrop://aaaa" },
  { hash: "bbbb2222cccc3333dddd4444eeee5555ffff6666aaaa1111bbbb2222cccc3333", name: "doc.pdf",   rings: [],          ticket: "rdrop://bbbb" },
];

const rings = [
  { name: "friends", open: false },
  { name: "work",    open: false },
];

const defaultProps = {
  rows,
  rings,
  onDelete: vi.fn(),
  onAttach: vi.fn(),
  onDetach: vi.fn(),
};

describe("BlobTable", () => {
  beforeEach(() => {
    Object.assign(navigator, { clipboard: { writeText: vi.fn().mockResolvedValue(undefined) } });
  });

  it("renders a row per blob", () => {
    const { getAllByRole } = render(BlobTable, { props: defaultProps });
    // header row + 2 data rows
    expect(getAllByRole("row").length).toBe(3);
  });

  it("shows blob name", () => {
    const { getByText } = render(BlobTable, { props: defaultProps });
    expect(getByText("photo.jpg")).toBeTruthy();
  });

  it("shows truncated hash", () => {
    const { getByText } = render(BlobTable, { props: defaultProps });
    expect(getByText("aaaa1111bbbb…")).toBeTruthy();
  });

  it("shows ring badge", () => {
    const { getByLabelText } = render(BlobTable, { props: defaultProps });
    // ring badge renders a detach button labelled "Detach from friends"
    expect(getByLabelText("Detach from friends")).toBeTruthy();
  });

  it("shows untagged label when rings is empty", () => {
    const { getByText } = render(BlobTable, { props: defaultProps });
    expect(getByText("untagged")).toBeTruthy();
  });

  it("shows empty state when rows is empty", () => {
    const { getByText } = render(BlobTable, { props: { ...defaultProps, rows: [] } });
    expect(getByText("No blobs in local store.")).toBeTruthy();
  });

  it("copies ticket to clipboard on copy button click", async () => {
    const { getAllByLabelText } = render(BlobTable, { props: defaultProps });
    await fireEvent.click(getAllByLabelText("Copy ticket")[0]);
    expect(navigator.clipboard.writeText).toHaveBeenCalledWith("rdrop://aaaa");
  });

  it("shows delete confirmation on delete button click", async () => {
    const { getAllByLabelText, findByText } = render(BlobTable, { props: defaultProps });
    await fireEvent.click(getAllByLabelText("Delete blob")[0]);
    expect(await findByText("Delete?")).toBeTruthy();
  });

  it("calls onDelete with the correct hash on confirm", async () => {
    const onDelete = vi.fn();
    const { getAllByLabelText, findByText } = render(BlobTable, { props: { ...defaultProps, onDelete } });
    await fireEvent.click(getAllByLabelText("Delete blob")[0]);
    await fireEvent.click(await findByText("Yes"));
    expect(onDelete).toHaveBeenCalledWith(rows[0].hash);
  });

  it("cancels delete without calling onDelete", async () => {
    const onDelete = vi.fn();
    const { getAllByLabelText, findByText } = render(BlobTable, { props: { ...defaultProps, onDelete } });
    await fireEvent.click(getAllByLabelText("Delete blob")[0]);
    await fireEvent.click(await findByText("No"));
    expect(onDelete).not.toHaveBeenCalled();
  });

  it("calls onDetach with the correct hash and ring when detach × is clicked", async () => {
    const onDetach = vi.fn();
    const { getByLabelText } = render(BlobTable, { props: { ...defaultProps, onDetach } });
    await fireEvent.click(getByLabelText("Detach from friends"));
    expect(onDetach).toHaveBeenCalledWith(rows[0].hash, rings[0]);
  });

  it("shows attach ring picker when + is clicked", async () => {
    const { getAllByLabelText, findByText } = render(BlobTable, { props: defaultProps });
    await fireEvent.click(getAllByLabelText("Attach to ring")[0]);
    expect(await findByText("work")).toBeTruthy();
  });

  it("calls onAttach with the correct hash and ring when a ring is selected", async () => {
    const onAttach = vi.fn();
    const { getAllByLabelText, findByRole } = render(BlobTable, { props: { ...defaultProps, onAttach } });
    await fireEvent.click(getAllByLabelText("Attach to ring")[0]);
    const select = await findByRole("combobox");
    await fireEvent.change(select, { target: { value: "work" } });
    expect(onAttach).toHaveBeenCalledWith(rows[0].hash, rings[1]);
  });
});
