import React, { useState } from "react";
import { Sparkles } from "lucide-react";
import type { Character } from "../types";
import { AIGenerateDialog } from "./AIGenerateDialog";

interface CharacterListProps {
  characters: Character[];
  projectId?: string;
  onCreateCharacter: () => void;
  onEditCharacter: (character: Character) => void;
  onDeleteCharacter: (characterId: string) => void;
  onAIGenerateCharacter?: (data: any) => Promise<void>;
}

export function CharacterList({
  characters,
  projectId,
  onCreateCharacter,
  onEditCharacter,
  onDeleteCharacter,
  onAIGenerateCharacter,
}: CharacterListProps) {
  const [isAIDialogOpen, setIsAIDialogOpen] = useState(false);

  const handleAIConfirm = async (data: any) => {
    if (onAIGenerateCharacter) {
      await onAIGenerateCharacter(data);
    }
    setIsAIDialogOpen(false);
  };

  return (
    <>
      <div className="flex flex-col h-full">
        <div className="p-4 border-b border-border">
          <div className="flex items-center justify-between">
            <h2 className="text-sm font-semibold">角色管理</h2>
            <div className="flex items-center gap-2">
              {projectId && onAIGenerateCharacter && (
                <button
                  onClick={() => setIsAIDialogOpen(true)}
                  className="flex items-center gap-1 text-sm text-blue-500 hover:text-blue-600 transition-colors"
                  title="AI 生成角色"
                >
                  <Sparkles className="w-4 h-4" />
                  AI生成
                </button>
              )}
              <button
                onClick={onCreateCharacter}
                className="text-sm text-primary hover:text-primary/80 transition-colors"
              >
                新建
              </button>
            </div>
          </div>
          <p className="text-xs text-muted-foreground mt-1">{characters.length} 个角色</p>
        </div>

        <div className="flex-1 overflow-y-auto">
          {characters.length === 0 ? (
            <div className="flex items-center justify-center h-full text-muted-foreground text-sm">
              暂无角色
            </div>
          ) : (
            <div className="p-2 space-y-2">
              {characters.map((character) => (
                <div
                  key={character.id}
                  className="p-3 border border-border rounded-lg hover:bg-accent transition-colors cursor-pointer"
                  onClick={() => onEditCharacter(character)}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1 min-w-0">
                      <h3 className="text-sm font-medium truncate">{character.name}</h3>
                      {character.age && (
                        <p className="text-xs text-muted-foreground mt-1">{character.age}岁</p>
                      )}
                      {character.gender && (
                        <p className="text-xs text-muted-foreground">{character.gender}</p>
                      )}
                    </div>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        onDeleteCharacter(character.id);
                      }}
                      className="text-xs text-destructive hover:text-destructive/80 transition-colors ml-2"
                    >
                      删除
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {projectId && (
        <AIGenerateDialog
          isOpen={isAIDialogOpen}
          onClose={() => setIsAIDialogOpen(false)}
          type="character"
          projectId={projectId}
          onConfirm={handleAIConfirm}
        />
      )}
    </>
  );
}
