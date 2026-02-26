import { useEffect, useCallback, useRef } from "react";

export interface KeyboardShortcut {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
  action: () => void;
  description?: string;
}

export function useKeyboardShortcuts(shortcuts: KeyboardShortcut[], enabled = true) {
  const shortcutsRef = useRef(shortcuts);
  shortcutsRef.current = shortcuts;

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!enabled) return;

      const target = event.target as HTMLElement;
      const isInput =
        target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable;

      for (const shortcut of shortcutsRef.current) {
        const keyMatch = event.key.toLowerCase() === shortcut.key.toLowerCase();
        const ctrlMatch = shortcut.ctrl ? event.ctrlKey || event.metaKey : !event.ctrlKey && !event.metaKey;
        const shiftMatch = shortcut.shift ? event.shiftKey : !event.shiftKey;
        const altMatch = shortcut.alt ? event.altKey : !event.altKey;

        if (keyMatch && ctrlMatch && shiftMatch && altMatch) {
          if (isInput && !shortcut.ctrl && !shortcut.meta) {
            continue;
          }
          event.preventDefault();
          shortcut.action();
          return;
        }
      }
    },
    [enabled]
  );

  useEffect(() => {
    if (enabled) {
      window.addEventListener("keydown", handleKeyDown);
      return () => window.removeEventListener("keydown", handleKeyDown);
    }
  }, [handleKeyDown, enabled]);
}

export function formatShortcut(shortcut: KeyboardShortcut): string {
  const parts: string[] = [];
  const isMac = navigator.platform.toUpperCase().indexOf("MAC") >= 0;

  if (shortcut.ctrl) {
    parts.push(isMac ? "⌘" : "Ctrl");
  }
  if (shortcut.shift) {
    parts.push(isMac ? "⇧" : "Shift");
  }
  if (shortcut.alt) {
    parts.push(isMac ? "⌥" : "Alt");
  }
  if (shortcut.meta) {
    parts.push(isMac ? "⌘" : "Meta");
  }

  parts.push(shortcut.key.toUpperCase());

  return parts.join(isMac ? "" : "+");
}

export const COMMON_SHORTCUTS = {
  SAVE: { key: "s", ctrl: true, description: "保存" },
  NEW: { key: "n", ctrl: true, description: "新建" },
  SEARCH: { key: "k", ctrl: true, description: "搜索" },
  CLOSE: { key: "Escape", description: "关闭/取消" },
  TOGGLE_SIDEBAR: { key: "b", ctrl: true, description: "切换侧边栏" },
  TOGGLE_FULLSCREEN: { key: "F11", description: "全屏" },
} as const;
