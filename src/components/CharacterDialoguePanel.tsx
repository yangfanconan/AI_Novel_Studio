import React, { useState, useEffect } from "react";
import {
  MessageSquare,
  Plus,
  Trash2,
  Settings,
  Send,
  RefreshCw,
  History,
  User,
  Edit,
} from "lucide-react";
import {
  characterDialogueService,
  DialogueSession,
  DialogueMessage,
  CreateSessionRequest,
  SendMessageRequest,
  UpdateSessionRequest,
} from "../services/characterDialogue.service";

interface CharacterDialoguePanelProps {
  characterId: string;
  characterName: string;
  onClose?: () => void;
}

export const CharacterDialoguePanel: React.FC<CharacterDialoguePanelProps> = ({
  characterId,
  characterName,
  onClose,
}) => {
  const [tabValue, setTabValue] = useState<"sessions" | "current">("sessions");
  const [sessions, setSessions] = useState<DialogueSession[]>([]);
  const [currentSession, setCurrentSession] = useState<DialogueSession | null>(null);
  const [messageInput, setMessageInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [settingsDialogOpen, setSettingsDialogOpen] = useState(false);
  const [newSessionName, setNewSessionName] = useState("");
  const [newSystemPrompt, setNewSystemPrompt] = useState("");
  const [settings, setSettings] = useState({
    session_name: "",
    system_prompt: "",
    ai_model: "default",
    temperature: 0.7,
    max_tokens: 1000,
  });

  useEffect(() => {
    loadSessions();
  }, [characterId]);

  const loadSessions = async () => {
    try {
      setLoading(true);
      const data = await characterDialogueService.getSessions(characterId);
      setSessions(data);
      setError(null);
    } catch (err) {
      setError("加载对话会话失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const loadSession = async (sessionId: string) => {
    try {
      setLoading(true);
      const session = await characterDialogueService.getSession(sessionId);
      setCurrentSession(session);
      setError(null);
    } catch (err) {
      setError("加载会话详情失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateSession = async () => {
    try {
      setLoading(true);
      const request: CreateSessionRequest = {
        character_id: characterId,
        session_name: newSessionName,
        system_prompt: newSystemPrompt || undefined,
      };
      const session = await characterDialogueService.createSession(request);
      setSessions([session, ...sessions]);
      setCurrentSession(session);
      setCreateDialogOpen(false);
      setNewSessionName("");
      setNewSystemPrompt("");
      setError(null);
    } catch (err) {
      setError("创建会话失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateQuickSession = async () => {
    try {
      setLoading(true);
      const session = await characterDialogueService.createQuickSession(characterId, characterName);
      setSessions([session, ...sessions]);
      setCurrentSession(session);
      setError(null);
    } catch (err) {
      setError("创建快速会话失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleSendMessage = async () => {
    if (!messageInput.trim() || !currentSession) return;

    try {
      setLoading(true);
      const request: SendMessageRequest = {
        session_id: currentSession.id,
        user_message: messageInput,
      };
      await characterDialogueService.sendMessage(request);
      await loadSession(currentSession.id);
      setMessageInput("");
      setError(null);
    } catch (err) {
      setError("发送消息失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteSession = async (sessionId: string) => {
    try {
      setLoading(true);
      await characterDialogueService.deleteSession(sessionId);
      setSessions(sessions.filter((s) => s.id !== sessionId));
      if (currentSession?.id === sessionId) {
        setCurrentSession(null);
      }
      setError(null);
    } catch (err) {
      setError("删除会话失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateSettings = async () => {
    if (!currentSession) return;

    try {
      setLoading(true);
      const request: UpdateSessionRequest = {
        session_id: currentSession.id,
        session_name: settings.session_name || undefined,
        system_prompt: settings.system_prompt || undefined,
        ai_model: settings.ai_model,
        temperature: settings.temperature,
        max_tokens: settings.max_tokens,
      };
      const updated = await characterDialogueService.updateSession(request);
      setCurrentSession(updated);
      setSessions(sessions.map((s) => (s.id === updated.id ? updated : s)));
      setSettingsDialogOpen(false);
      setError(null);
    } catch (err) {
      setError("更新设置失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleRegenerateResponse = async (messageId: string) => {
    try {
      setLoading(true);
      await characterDialogueService.regenerateResponse(messageId);
      if (currentSession) {
        await loadSession(currentSession.id);
      }
      setError(null);
    } catch (err) {
      setError("重新生成回复失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteMessage = async (messageId: string) => {
    try {
      setLoading(true);
      await characterDialogueService.deleteMessage(messageId);
      if (currentSession) {
        await loadSession(currentSession.id);
      }
      setError(null);
    } catch (err) {
      setError("删除消息失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const openSettingsDialog = () => {
    if (currentSession) {
      setSettings({
        session_name: currentSession.session_name,
        system_prompt: currentSession.system_prompt || "",
        ai_model: currentSession.settings.ai_model,
        temperature: currentSession.settings.temperature,
        max_tokens: currentSession.settings.max_tokens,
      });
      setSettingsDialogOpen(true);
    }
  };

  return (
    <div className="max-w-7xl mx-auto p-4">
      <div className="flex items-center gap-2 mb-4">
        <MessageSquare className="w-6 h-6" />
        <h1 className="text-3xl font-semibold">{characterName} 对话</h1>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md text-red-700">
          {error}
          <button
            onClick={() => setError(null)}
            className="float-right ml-2 text-red-500 hover:text-red-700"
          >
            ✕
          </button>
        </div>
      )}

      <div className="border-b border-border">
        <div className="flex gap-1">
          <button
            onClick={() => setTabValue("sessions")}
            className={`px-4 py-2 text-sm rounded-t-md transition-colors ${
              tabValue === "sessions"
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted"
            }`}
          >
            对话列表
          </button>
          <button
            onClick={() => setTabValue("current")}
            disabled={!currentSession}
            className={`px-4 py-2 text-sm rounded-t-md transition-colors ${
              tabValue === "current" && currentSession
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-muted"
            }`}
          >
            当前对话
          </button>
        </div>
      </div>

      {tabValue === "sessions" && (
        <div className="p-4 bg-card">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-lg font-semibold">对话会话</h2>
            <div className="flex gap-2">
              <button
                onClick={handleCreateQuickSession}
                disabled={loading}
                className="px-3 py-1.5 text-sm border border-border rounded-md hover:bg-muted"
              >
                <div className="flex items-center gap-1">
                  <Plus className="w-4 h-4" />
                  快速开始
                </div>
              </button>
              <button
                onClick={() => setCreateDialogOpen(true)}
                disabled={loading}
                className="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              >
                <div className="flex items-center gap-1">
                  <Plus className="w-4 h-4" />
                  创建会话
                </div>
              </button>
              <button
                onClick={loadSessions}
                disabled={loading}
                className="p-1.5 border border-border rounded-md hover:bg-muted"
              >
                <RefreshCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
              </button>
            </div>
          </div>

          {loading && sessions.length === 0 ? (
            <div className="flex justify-center py-8">
              <div className="animate-spin w-6 h-6 border-2 border-primary border-t-transparent rounded-full" />
            </div>
          ) : sessions.length === 0 ? (
            <div className="text-center py-8 text-sm text-muted-foreground">
              暂无对话会话，点击上方按钮创建
            </div>
          ) : (
            <div className="space-y-2">
              {sessions.map((session) => (
                <div
                  key={session.id}
                  onClick={() => {
                    loadSession(session.id);
                    setTabValue("current");
                  }}
                  className="p-3 border border-border rounded-md hover:bg-muted cursor-pointer"
                >
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <div className="font-medium">{session.session_name}</div>
                      <div className="text-xs text-muted-foreground mt-1">
                        {session.messages.length} 条消息 • 更新于{" "}
                        {new Date(session.updated_at).toLocaleString()}
                      </div>
                      {session.context_summary && (
                        <div className="text-xs text-muted-foreground mt-0.5">
                          {session.context_summary}
                        </div>
                      )}
                    </div>
                    <div className="flex gap-1">
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setCurrentSession(session);
                          openSettingsDialog();
                        }}
                        className="p-1.5 border border-border rounded-md hover:bg-muted"
                      >
                        <Settings className="w-4 h-4" />
                      </button>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDeleteSession(session.id);
                        }}
                        className="p-1.5 border border-border rounded-md hover:bg-muted"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {tabValue === "current" && currentSession && (
        <div className="flex flex-col gap-4 p-4">
          <div className="p-4 border border-border rounded-md bg-card">
            <div className="flex justify-between items-center">
              <h2 className="text-lg font-semibold">{currentSession.session_name}</h2>
              <div className="flex gap-2">
                <button
                  onClick={openSettingsDialog}
                  className="px-3 py-1.5 text-sm border border-border rounded-md hover:bg-muted"
                >
                  <div className="flex items-center gap-1">
                    <Settings className="w-4 h-4" />
                    设置
                  </div>
                </button>
                <button
                  onClick={() => loadSession(currentSession.id)}
                  className="p-1.5 border border-border rounded-md hover:bg-muted"
                >
                  <RefreshCw className="w-4 h-4" />
                </button>
              </div>
            </div>
            <div className="flex gap-2 mt-3 flex-wrap">
              <span className="inline-flex items-center gap-1 px-2 py-1 text-xs border border-border rounded-full">
                <User className="w-3 h-3" />
                {characterName}
              </span>
              <span className="inline-flex items-center px-2 py-1 text-xs border border-primary rounded-full text-primary">
                {currentSession.messages.length} 条消息
              </span>
              <span className="inline-flex items-center px-2 py-1 text-xs border border-border rounded-full">
                温度: {currentSession.settings.temperature}
              </span>
            </div>
          </div>

          <div className="flex-1 min-h-96 max-h-[600px] overflow-auto p-4 border border-border rounded-md bg-card">
            {currentSession.messages.length === 0 ? (
              <div className="text-center py-8 text-sm text-muted-foreground">
                开始与角色对话吧！
              </div>
            ) : (
              <div className="space-y-4">
                {currentSession.messages.map((msg, idx) => (
                  <div
                    key={msg.id}
                    className={`flex ${msg.role === "user" ? "justify-end" : "justify-start"}`}
                  >
                    <div
                      className={`max-w-[80%] p-3 rounded-lg ${
                        msg.role === "user" ? "bg-primary text-primary-foreground" : "bg-muted"
                      }`}
                    >
                      <div className="flex justify-between items-start gap-2 mb-1">
                        <span className="text-xs font-medium">
                          {msg.role === "user" ? "你" : characterName}
                        </span>
                        <span className="text-xs text-muted-foreground">
                          {new Date(msg.created_at).toLocaleString()}
                        </span>
                      </div>
                      <div className="whitespace-pre-wrap text-sm">{msg.content}</div>
                    </div>
                    {msg.role === "assistant" && (
                      <div className="flex gap-2 mt-2 text-xs">
                        <button
                          onClick={() => handleRegenerateResponse(msg.id)}
                          disabled={loading}
                          className="px-2 py-1 border border-border rounded-md hover:bg-muted"
                        >
                          <div className="flex items-center gap-1">
                            <RefreshCw className="w-3 h-3" />
                            重新生成
                          </div>
                        </button>
                        <button
                          onClick={() => handleDeleteMessage(msg.id)}
                          disabled={loading}
                          className="px-2 py-1 border border-border rounded-md hover:bg-muted"
                        >
                          <div className="flex items-center gap-1">
                            <Trash2 className="w-3 h-3" />
                            删除
                          </div>
                        </button>
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="p-4 border border-border rounded-md bg-card">
            <div className="flex gap-2">
              <textarea
                value={messageInput}
                onChange={(e) => setMessageInput(e.target.value)}
                onKeyPress={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault();
                    handleSendMessage();
                  }
                }}
                disabled={loading}
                placeholder="输入消息..."
                rows={3}
                className="flex-1 px-3 py-2 text-sm border border-border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary"
              />
              <button
                onClick={handleSendMessage}
                disabled={loading || !messageInput.trim()}
                className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              >
                <div className="flex items-center gap-1">
                  <Send className="w-4 h-4" />
                  发送
                </div>
              </button>
            </div>
          </div>
        </div>
      )}

      {createDialogOpen && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-card border border-border rounded-lg p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">创建对话会话</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">会话名称</label>
                <input
                  type="text"
                  value={newSessionName}
                  onChange={(e) => setNewSessionName(e.target.value)}
                  placeholder="输入会话名称"
                  className="w-full px-3 py-2 text-sm border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">系统提示词</label>
                <textarea
                  value={newSystemPrompt}
                  onChange={(e) => setNewSystemPrompt(e.target.value)}
                  placeholder="定义角色在对话中的行为和风格"
                  rows={4}
                  className="w-full px-3 py-2 text-sm border border-border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={() => setCreateDialogOpen(false)}
                className="px-4 py-2 text-sm border border-border rounded-md hover:bg-muted"
              >
                取消
              </button>
              <button
                onClick={handleCreateSession}
                disabled={loading || !newSessionName}
                className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              >
                创建
              </button>
            </div>
          </div>
        </div>
      )}

      {settingsDialogOpen && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-card border border-border rounded-lg p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">会话设置</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">会话名称</label>
                <input
                  type="text"
                  value={settings.session_name}
                  onChange={(e) => setSettings({ ...settings, session_name: e.target.value })}
                  className="w-full px-3 py-2 text-sm border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">系统提示词</label>
                <textarea
                  value={settings.system_prompt}
                  onChange={(e) => setSettings({ ...settings, system_prompt: e.target.value })}
                  rows={4}
                  className="w-full px-3 py-2 text-sm border border-border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">AI 模型</label>
                <select
                  value={settings.ai_model}
                  onChange={(e) => setSettings({ ...settings, ai_model: e.target.value })}
                  className="w-full px-3 py-2 text-sm border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                >
                  <option value="default">默认模型</option>
                  <option value="gpt-4">GPT-4</option>
                  <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">
                  温度: {settings.temperature}
                </label>
                <input
                  type="range"
                  min={0}
                  max={2}
                  step={0.1}
                  value={settings.temperature}
                  onChange={(e) =>
                    setSettings({ ...settings, temperature: parseFloat(e.target.value) })
                  }
                  className="w-full"
                />
                <div className="flex justify-between text-xs text-muted-foreground mt-1">
                  <span>精确</span>
                  <span>平衡</span>
                  <span>创意</span>
                </div>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">
                  最大令牌数: {settings.max_tokens}
                </label>
                <input
                  type="range"
                  min={100}
                  max={4000}
                  step={100}
                  value={settings.max_tokens}
                  onChange={(e) =>
                    setSettings({ ...settings, max_tokens: parseInt(e.target.value) })
                  }
                  className="w-full"
                />
                <div className="flex justify-between text-xs text-muted-foreground mt-1">
                  <span>500</span>
                  <span>1000</span>
                  <span>2000</span>
                  <span>4000</span>
                </div>
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={() => setSettingsDialogOpen(false)}
                className="px-4 py-2 text-sm border border-border rounded-md hover:bg-muted"
              >
                取消
              </button>
              <button
                onClick={handleUpdateSettings}
                disabled={loading}
                className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
              >
                保存
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default CharacterDialoguePanel;
