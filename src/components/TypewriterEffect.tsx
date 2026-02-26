import React, { useState, useEffect, useCallback } from "react";

interface TypewriterEffectProps {
  text: string;
  speed?: number;
  onComplete?: () => void;
  className?: string;
  showCursor?: boolean;
  cursorChar?: string;
}

export const TypewriterEffect: React.FC<TypewriterEffectProps> = ({
  text,
  speed = 30,
  onComplete,
  className = "",
  showCursor = true,
  cursorChar = "|",
}) => {
  const [displayedText, setDisplayedText] = useState("");
  const [isComplete, setIsComplete] = useState(false);

  useEffect(() => {
    setDisplayedText("");
    setIsComplete(false);
  }, [text]);

  useEffect(() => {
    if (displayedText.length < text.length) {
      const timer = setTimeout(() => {
        setDisplayedText(text.slice(0, displayedText.length + 1));
      }, speed);
      return () => clearTimeout(timer);
    } else if (!isComplete && displayedText.length === text.length && text.length > 0) {
      setIsComplete(true);
      onComplete?.();
    }
  }, [displayedText, text, speed, isComplete, onComplete]);

  return (
    <span className={className}>
      {displayedText}
      {showCursor && (
        <span
          className={`inline-block w-[2px] h-[1em] bg-current ml-0.5 ${
            isComplete ? "animate-pulse" : "animate-blink"
          }`}
        >
          {cursorChar}
        </span>
      )}
    </span>
  );
};

interface StreamingTextProps {
  chunks: string[];
  speed?: number;
  onChunkComplete?: (index: number) => void;
  onComplete?: () => void;
  className?: string;
}

export const StreamingText: React.FC<StreamingTextProps> = ({
  chunks,
  speed = 30,
  onChunkComplete,
  onComplete,
  className = "",
}) => {
  const [currentChunkIndex, setCurrentChunkIndex] = useState(0);
  const [displayedChunks, setDisplayedChunks] = useState<string[]>([]);

  const handleChunkComplete = useCallback(() => {
    onChunkComplete?.(currentChunkIndex);
    if (currentChunkIndex < chunks.length - 1) {
      setDisplayedChunks((prev) => [...prev, chunks[currentChunkIndex]]);
      setCurrentChunkIndex((prev) => prev + 1);
    } else {
      onComplete?.();
    }
  }, [currentChunkIndex, chunks, onChunkComplete, onComplete]);

  useEffect(() => {
    setCurrentChunkIndex(0);
    setDisplayedChunks([]);
  }, [chunks]);

  if (chunks.length === 0) return null;

  return (
    <div className={className}>
      {displayedChunks.map((chunk, index) => (
        <span key={index}>{chunk}</span>
      ))}
      {currentChunkIndex < chunks.length && (
        <TypewriterEffect
          text={chunks[currentChunkIndex]}
          speed={speed}
          onComplete={handleChunkComplete}
          showCursor={currentChunkIndex === chunks.length - 1}
        />
      )}
    </div>
  );
};

export default TypewriterEffect;
