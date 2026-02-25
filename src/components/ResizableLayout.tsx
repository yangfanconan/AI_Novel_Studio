import React, { useState, useRef, useCallback, useEffect } from "react";
import { ChevronLeft, ChevronRight } from "lucide-react";

interface ResizableLayoutProps {
  leftPanel: React.ReactNode;
  centerPanel: React.ReactNode;
  rightPanel: React.ReactNode;
  children?: React.ReactNode;
  initialLeftWidth?: number;
  initialRightWidth?: number;
  minLeftWidth?: number;
  minRightWidth?: number;
  maxLeftWidth?: number;
  maxRightWidth?: number;
}

export const ResizableLayout: React.FC<ResizableLayoutProps> = ({
  leftPanel,
  centerPanel,
  rightPanel,
  children,
  initialLeftWidth = 256,
  initialRightWidth = 288,
  minLeftWidth = 200,
  minRightWidth = 250,
  maxLeftWidth = 600,
  maxRightWidth = 600,
}) => {
  const [leftWidth, setLeftWidth] = useState(initialLeftWidth);
  const [rightWidth, setRightWidth] = useState(initialRightWidth);
  const [leftCollapsed, setLeftCollapsed] = useState(false);
  const [rightCollapsed, setRightCollapsed] = useState(false);
  const [isResizingLeft, setIsResizingLeft] = useState(false);
  const [isResizingRight, setIsResizingRight] = useState(false);

  const startX = useRef(0);
  const startLeftWidth = useRef(0);
  const startRightWidth = useRef(0);

  const handleLeftMouseDown = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      setIsResizingLeft(true);
      startX.current = e.clientX;
      startLeftWidth.current = leftWidth;
      document.body.style.cursor = "col-resize";
      document.body.style.userSelect = "none";
    },
    [leftWidth]
  );

  const handleRightMouseDown = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      setIsResizingRight(true);
      startX.current = e.clientX;
      startRightWidth.current = rightWidth;
      document.body.style.cursor = "col-resize";
      document.body.style.userSelect = "none";
    },
    [rightWidth]
  );

  const handleMouseMove = useCallback(
    (e: MouseEvent) => {
      if (!isResizingLeft && !isResizingRight) return;

      const deltaX = e.clientX - startX.current;

      if (isResizingLeft) {
        const newLeftWidth = Math.min(
          maxLeftWidth,
          Math.max(minLeftWidth, startLeftWidth.current + deltaX)
        );
        setLeftWidth(newLeftWidth);
      }

      if (isResizingRight) {
        const newRightWidth = Math.min(
          maxRightWidth,
          Math.max(minRightWidth, startRightWidth.current - deltaX)
        );
        setRightWidth(newRightWidth);
      }
    },
    [isResizingLeft, isResizingRight, minLeftWidth, minRightWidth, maxLeftWidth, maxRightWidth]
  );

  const toggleLeftCollapse = useCallback(() => {
    if (leftCollapsed) {
      setLeftWidth(initialLeftWidth);
      setLeftCollapsed(false);
    } else {
      setLeftWidth(0);
      setLeftCollapsed(true);
    }
  }, [leftCollapsed, initialLeftWidth]);

  const toggleRightCollapse = useCallback(() => {
    if (rightCollapsed) {
      setRightWidth(initialRightWidth);
      setRightCollapsed(false);
    } else {
      setRightWidth(0);
      setRightCollapsed(true);
    }
  }, [rightCollapsed, initialRightWidth]);

  const handleMouseUp = useCallback(() => {
    setIsResizingLeft(false);
    setIsResizingRight(false);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }, []);

  useEffect(() => {
    if (isResizingLeft || isResizingRight) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseup", handleMouseUp);
    }

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isResizingLeft, isResizingRight, handleMouseMove, handleMouseUp]);

  useEffect(() => {
    const projectId = localStorage.getItem("current-project-id") || "default";
    const savedLeftWidths = localStorage.getItem("resizable-left-widths");
    const savedRightWidths = localStorage.getItem("resizable-right-widths");

    if (savedLeftWidths) {
      try {
        const parsed = JSON.parse(savedLeftWidths);
        const savedWidth = parsed[projectId];
        if (savedWidth) setLeftWidth(savedWidth);
      } catch (e) {
        console.warn("Failed to parse saved left widths:", e);
      }
    }

    if (savedRightWidths) {
      try {
        const parsed = JSON.parse(savedRightWidths);
        const savedWidth = parsed[projectId];
        if (savedWidth) setRightWidth(savedWidth);
      } catch (e) {
        console.warn("Failed to parse saved right widths:", e);
      }
    }
  }, []);

  useEffect(() => {
    const projectId = localStorage.getItem("current-project-id") || "default";
    const savedLeftWidths = localStorage.getItem("resizable-left-widths");
    if (savedLeftWidths) {
      try {
        const parsed = JSON.parse(savedLeftWidths);
        const newWidths = { ...parsed, [projectId]: leftWidth };
        localStorage.setItem("resizable-left-widths", JSON.stringify(newWidths));
      } catch (e) {
        console.warn("Failed to save left width:", e);
      }
    }
  }, [leftWidth]);

  useEffect(() => {
    const projectId = localStorage.getItem("current-project-id") || "default";
    const savedRightWidths = localStorage.getItem("resizable-right-widths");
    if (savedRightWidths) {
      try {
        const parsed = JSON.parse(savedRightWidths);
        const newWidths = { ...parsed, [projectId]: rightWidth };
        localStorage.setItem("resizable-right-widths", JSON.stringify(newWidths));
      } catch (e) {
        console.warn("Failed to save right width:", e);
      }
    }
  }, [rightWidth]);

  const centerWidth = `calc(100% - ${leftWidth + rightWidth}px)`;

  return (
    <div className="flex h-screen bg-background overflow-hidden">
      <div
        style={{ width: leftWidth, minWidth: leftCollapsed ? 0 : minLeftWidth }}
        className="flex-shrink-0 border-r border-border flex flex-col overflow-hidden"
      >
        {leftPanel}
      </div>

      <div className="flex items-center">
        <div
          onMouseDown={handleLeftMouseDown}
          className={`flex-shrink-0 w-1 cursor-col-resize transition-colors ${
            isResizingLeft ? "bg-primary" : "hover:bg-border"
          }`}
        />
        <button
          onClick={toggleLeftCollapse}
          className="flex-shrink-0 p-1 hover:bg-border transition-colors text-muted-foreground"
          title={leftCollapsed ? "展开" : "折叠"}
        >
          {leftCollapsed ? <ChevronRight className="w-3 h-3" /> : <ChevronLeft className="w-3 h-3" />}
        </button>
      </div>

      <div
        style={{ width: centerWidth, minWidth: 0 }}
        className="flex-1 flex flex-col overflow-hidden"
      >
        {centerPanel}
      </div>

      <div className="flex items-center">
        <button
          onClick={toggleRightCollapse}
          className="flex-shrink-0 p-1 hover:bg-border transition-colors text-muted-foreground"
          title={rightCollapsed ? "展开" : "折叠"}
        >
          {rightCollapsed ? <ChevronLeft className="w-3 h-3" /> : <ChevronRight className="w-3 h-3" />}
        </button>
        <div
          onMouseDown={handleRightMouseDown}
          className={`flex-shrink-0 w-1 cursor-col-resize transition-colors ${
            isResizingRight ? "bg-primary" : "hover:bg-border"
          }`}
        />
      </div>

      <div
        style={{ width: rightWidth, minWidth: rightCollapsed ? 0 : minRightWidth }}
        className="flex-shrink-0 border-l border-border flex flex-col overflow-hidden"
      >
        {rightPanel}
      </div>
      {children}
    </div>
  );
};
