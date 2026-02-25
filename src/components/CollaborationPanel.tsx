import React, { useState, useEffect } from "react";
import { Users, Wifi, WifiOff, Copy, Settings, UserPlus } from "lucide-react";
import {
  collaborationService,
  User,
  CursorPosition,
  CollaborationSession,
} from "../services/collaboration.service";

interface CollaborationPanelProps {
  projectId?: string;
  onShare?: () => void;
}

export const CollaborationPanel: React.FC<CollaborationPanelProps> = ({ projectId, onShare }) => {
  const [session, setSession] = useState<CollaborationSession | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [showUsers, setShowUsers] = useState(true);
  const [currentUser, setCurrentUser] = useState<User | null>(null);

  useEffect(() => {
    initializeCollaboration();
  }, [projectId]);

  const initializeCollaboration = async () => {
    try {
      const userId = await collaborationService.generateUserId();
      const color = await collaborationService.generateColor();

      const user: User = {
        id: userId,
        name: `User ${userId.slice(-4)}`,
        color,
      };

      setCurrentUser(user);

      if (projectId) {
        const newSessionId = await collaborationService.createSession(projectId);
        setSessionId(newSessionId);

        await collaborationService.joinSession(newSessionId, user);
        await loadSession(newSessionId);
        setIsConnected(true);
      }
    } catch (error) {
      console.error("Failed to initialize collaboration:", error);
    }
  };

  const loadSession = async (sessionId: string) => {
    try {
      const sessionData = await collaborationService.getSession(sessionId);
      setSession(sessionData);
    } catch (error) {
      console.error("Failed to load session:", error);
    }
  };

  const handleCopySessionId = () => {
    if (sessionId) {
      navigator.clipboard.writeText(sessionId);
    }
  };

  const handleInviteUser = () => {
    if (sessionId && onShare) {
      onShare();
    }
  };

  const refreshSession = async () => {
    if (sessionId) {
      await loadSession(sessionId);
    }
  };

  return (
    <div className="border-b border-border bg-card">
      <div className="px-4 py-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            {isConnected ? (
              <Wifi className="w-4 h-4 text-green-500" />
            ) : (
              <WifiOff className="w-4 h-4 text-gray-400" />
            )}
            <span className="text-sm font-medium">协作编辑</span>
          </div>

          <div className="flex items-center gap-2">
            {sessionId && (
              <button
                onClick={handleCopySessionId}
                className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
                title="复制会话 ID"
              >
                <Copy className="w-4 h-4" />
              </button>
            )}
            <button
              onClick={handleInviteUser}
              className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
              title="邀请协作者"
            >
              <UserPlus className="w-4 h-4" />
            </button>
            <button
              onClick={() => setShowUsers(!showUsers)}
              className={`p-1.5 rounded-md transition-colors ${
                showUsers
                  ? "text-foreground bg-muted"
                  : "text-muted-foreground hover:text-foreground hover:bg-muted"
              }`}
              title={showUsers ? "隐藏用户" : "显示用户"}
            >
              <Users className="w-4 h-4" />
            </button>
            <button
              onClick={() => {}}
              className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
              title="协作设置"
            >
              <Settings className="w-4 h-4" />
            </button>
          </div>
        </div>

        {showUsers && session && session.users.length > 0 && (
          <div className="mt-3 flex flex-wrap gap-2">
            {session.users.map((user) => (
              <div
                key={user.id}
                className="flex items-center gap-2 px-3 py-1.5 rounded-full text-sm"
                style={{
                  backgroundColor: `${user.color}20`,
                  border: `1px solid ${user.color}40`,
                }}
              >
                <div className="w-2 h-2 rounded-full" style={{ backgroundColor: user.color }} />
                <span className="font-medium">{user.name}</span>
                {user.id === currentUser?.id && (
                  <span className="text-xs text-muted-foreground">(你)</span>
                )}
              </div>
            ))}
          </div>
        )}

        {session && session.users.length === 0 && (
          <div className="mt-3 text-sm text-muted-foreground">等待其他用户加入...</div>
        )}
      </div>
    </div>
  );
};
