import { afterEach, beforeEach, vi } from "vitest";
import { cleanup } from "@testing-library/svelte";

afterEach(() => cleanup());
beforeEach(() => vi.clearAllMocks());
