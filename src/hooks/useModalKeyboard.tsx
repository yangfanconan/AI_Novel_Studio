import { useEffect, useCallback } from "react";

interface UseModalKeyboardOptions {
  isOpen: boolean;
  onClose: () => void;
  onSubmit?: () => void;
  enableEnterSubmit?: boolean;
  enableEscapeClose?: boolean;
}

export function useModalKeyboard({
  isOpen,
  onClose,
  onSubmit,
  enableEnterSubmit = false,
  enableEscapeClose = true,
}: UseModalKeyboardOptions) {
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!isOpen) return;

      if (event.key === "Escape" && enableEscapeClose) {
        event.preventDefault();
        event.stopPropagation();
        onClose();
      }

      if (event.key === "Enter" && enableEnterSubmit && onSubmit) {
        if (
          event.target instanceof HTMLInputElement ||
          event.target instanceof HTMLTextAreaElement
        ) {
          return;
        }
        event.preventDefault();
        onSubmit();
      }
    },
    [isOpen, onClose, onSubmit, enableEnterSubmit, enableEscapeClose]
  );

  useEffect(() => {
    if (isOpen) {
      document.addEventListener("keydown", handleKeyDown);
      document.body.style.overflow = "hidden";
    }

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.body.style.overflow = "";
    };
  }, [isOpen, handleKeyDown]);
}

export function ModalOverlay({
  isOpen,
  onClose,
  children,
  className = "",
}: {
  isOpen: boolean;
  onClose: () => void;
  children: React.ReactNode;
  className?: string;
}) {
  useModalKeyboard({ isOpen, onClose });

  if (!isOpen) return null;

  return (
    <div
      className={`fixed inset-0 z-50 flex items-center justify-center ${className}`}
      onClick={(e) => {
        if (e.target === e.currentTarget) {
          onClose();
        }
      }}
    >
      <div className="absolute inset-0 bg-black/50" onClick={onClose} />
      <div className="relative z-10">{children}</div>
    </div>
  );
}
