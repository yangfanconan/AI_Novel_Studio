import "@testing-library/jest-dom";
import { expect, afterEach, vi } from "vitest";
import { cleanup } from "@testing-library/react";

afterEach(() => {
  cleanup();
});

expect.extend({
  toHaveBeenCalledWithLogs: (received, logs: string[]) => {
    const pass = logs.some((log) => received.toString().includes(log));

    return {
      pass,
      message: () =>
        pass
          ? `Expected function to have been called with logs containing: ${logs}`
          : `Expected function to have been called with logs containing: ${logs}, but it was called with: ${received}`,
    };
  },
});

global.console = {
  ...console,
  error: vi.fn(),
  warn: vi.fn(),
  log: vi.fn(),
  info: vi.fn(),
  debug: vi.fn(),
};
