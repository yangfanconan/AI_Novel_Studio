import React, { useState, useEffect } from "react";
import { X, Save } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import {
  WorldView,
  CreateWorldViewRequest,
  UpdateWorldViewRequest,
  WorldViewTimelineEvent,
  CreateWorldViewTimelineEventRequest,
} from "../types";

interface WorldViewEditorProps {
  worldView: WorldView | null;
  projectId: string;
  onClose: () => void;
  onSave: () => void;
  initialTitle?: string;
}

const CATEGORIES = [
  { id: "geography", name: "地理环境", icon: "🌍" },
  { id: "history", name: "历史背景", icon: "📜" },
  { id: "culture", name: "文化风俗", icon: "🎭" },
  { id: "politics", name: "政治制度", icon: "🏛️" },
  { id: "economy", name: "经济体系", icon: "💰" },
  { id: "magic", name: "魔法/科技", icon: "✨" },
  { id: "religion", name: "宗教信仰", icon: "🕍" },
  { id: "races", name: "种族生物", icon: "👥" },
  { id: "other", name: "其他", icon: "📝" },
];

const EVENT_TYPES = [
  { value: "discovery", label: "发现/诞生", icon: "💡" },
  { value: "war", label: "战争冲突", icon: "⚔️" },
  { value: "treaty", label: "条约签订", icon: "📜" },
  { value: "disaster", label: "灾难事件", icon: "🌋" },
  { value: "revolution", label: "革命变革", icon: "🔥" },
  { value: "migration", label: "人口迁移", icon: "🚶" },
  { value: "development", label: "发展进步", icon: "📈" },
  { value: "decline", label: "衰落消亡", icon: "📉" },
  { value: "other", label: "其他", icon: "📝" },
];

export function WorldViewEditor({
  worldView,
  projectId,
  onClose,
  onSave,
  initialTitle,
}: WorldViewEditorProps) {
  const [activeTab, setActiveTab] = useState<"basic" | "timeline">("basic");
  const [category, setCategory] = useState("geography");
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [tags, setTags] = useState("");
  const [status, setStatus] = useState("draft");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [timelineEvents, setTimelineEvents] = useState<WorldViewTimelineEvent[]>([]);
  const [isLoadingTimeline, setIsLoadingTimeline] = useState(false);
  const [showEventForm, setShowEventForm] = useState(false);
  const [editingEvent, setEditingEvent] = useState<WorldViewTimelineEvent | null>(null);
  const [eventForm, setEventForm] = useState({
    event_type: "discovery",
    event_title: "",
    event_description: "",
    story_time: "",
    impact_scope: "",
    related_characters: "",
  });

  useEffect(() => {
    if (worldView) {
      setCategory(worldView.category || "geography");
      setTitle(worldView.title);
      setContent(worldView.content);
      setTags(worldView.tags || "");
      setStatus(worldView.status);
      loadTimelineEvents(worldView.id);
    } else {
      setCategory("other");
      setTitle(initialTitle || "");
      setContent("");
      setTags("");
      setStatus("draft");
      setTimelineEvents([]);
    }
    setActiveTab("basic");
    setShowEventForm(false);
    setEditingEvent(null);
  }, [worldView, initialTitle]);

  const loadTimelineEvents = async (worldviewId: string) => {
    setIsLoadingTimeline(true);
    try {
      const events = await invoke<WorldViewTimelineEvent[]>("get_worldview_timeline", {
        worldviewId,
      });
      setTimelineEvents(events);
    } catch (error) {
      console.error("Failed to load timeline events:", error);
      setTimelineEvents([]);
    } finally {
      setIsLoadingTimeline(false);
    }
  };

  const handleCreateEvent = async () => {
    if (!worldView || !eventForm.event_title.trim()) return;

    try {
      const request: CreateWorldViewTimelineEventRequest = {
        worldview_id: worldView.id,
        event_type: eventForm.event_type,
        event_title: eventForm.event_title,
        event_description: eventForm.event_description,
        story_time: eventForm.story_time || undefined,
        impact_scope: eventForm.impact_scope || undefined,
        related_characters: eventForm.related_characters || undefined,
        sort_order: timelineEvents.length,
      };

      const newEvent = await invoke<WorldViewTimelineEvent>("create_worldview_timeline_event", {
        request,
      });
      setTimelineEvents([...timelineEvents, newEvent]);
      resetEventForm();
    } catch (error) {
      console.error("Failed to create event:", error);
    }
  };

  const handleUpdateEvent = async () => {
    if (!editingEvent) return;

    try {
      const updatedEvent = await invoke<WorldViewTimelineEvent>("update_worldview_timeline_event", {
        eventId: editingEvent.id,
        request: {
          event_type: eventForm.event_type,
          event_title: eventForm.event_title,
          event_description: eventForm.event_description,
          story_time: eventForm.story_time || null,
          impact_scope: eventForm.impact_scope || null,
          related_characters: eventForm.related_characters || null,
        },
      });
      setTimelineEvents(timelineEvents.map((e) => (e.id === updatedEvent.id ? updatedEvent : e)));
      resetEventForm();
    } catch (error) {
      console.error("Failed to update event:", error);
    }
  };

  const handleDeleteEvent = async (eventId: string) => {
    if (!confirm("确定要删除这个事件吗？")) return;

    try {
      await invoke("delete_worldview_timeline_event", { eventId });
      setTimelineEvents(timelineEvents.filter((e) => e.id !== eventId));
    } catch (error) {
      console.error("Failed to delete event:", error);
    }
  };

  const resetEventForm = () => {
    setShowEventForm(false);
    setEditingEvent(null);
    setEventForm({
      event_type: "discovery",
      event_title: "",
      event_description: "",
      story_time: "",
      impact_scope: "",
      related_characters: "",
    });
  };

  const startEditEvent = (event: WorldViewTimelineEvent) => {
    setEditingEvent(event);
    setEventForm({
      event_type: event.event_type,
      event_title: event.event_title,
      event_description: event.event_description,
      story_time: event.story_time || "",
      impact_scope: event.impact_scope || "",
      related_characters: event.related_characters || "",
    });
    setShowEventForm(true);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (worldView && worldView.id) {
        const updateData: UpdateWorldViewRequest = {
          id: worldView.id,
          category,
          title,
          content,
          tags: tags || undefined,
          status,
        };
        await invoke("update_world_view", { request: updateData });
      } else {
        const createData: CreateWorldViewRequest = {
          project_id: projectId,
          category,
          title,
          content,
          tags: tags || undefined,
        };
        await invoke("create_world_view", { request: createData });
      }
      onSave();
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const getEventTypeInfo = (type: string) => {
    return EVENT_TYPES.find((t) => t.value === type) || EVENT_TYPES[EVENT_TYPES.length - 1];
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-900 rounded-lg shadow-xl w-full max-w-4xl max-h-[90vh] overflow-hidden flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-900 dark:text-slate-100">
            {worldView?.id ? "编辑世界观" : "新建世界观"}
          </h2>
          <button
            onClick={onClose}
            className="p-1 hover:bg-slate-100 dark:hover:bg-slate-800 rounded"
          >
            <X className="w-5 h-5 text-slate-500" />
          </button>
        </div>

        <div className="flex border-b border-gray-200">
          {[
            { id: "basic", label: "基本信息", icon: "🌍" },
            { id: "timeline", label: "事件时间线", icon: "📅" },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id as typeof activeTab)}
              className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab.id
                  ? "border-blue-500 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700"
              }`}
            >
              {tab.icon} {tab.label}
            </button>
          ))}
        </div>

        <form onSubmit={handleSubmit} className="flex-1 overflow-y-auto p-4">
          {error && (
            <div className="p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded text-sm mb-4">
              {error}
            </div>
          )}

          {activeTab === "basic" && (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  分类 *
                </label>
                <div className="grid grid-cols-3 gap-2">
                  {CATEGORIES.map((cat) => (
                    <button
                      key={cat.id}
                      type="button"
                      onClick={() => setCategory(cat.id)}
                      className={`p-3 rounded-lg text-left transition-colors ${
                        category === cat.id
                          ? "bg-blue-50 dark:bg-blue-900/30 border-2 border-blue-500"
                          : "bg-slate-50 dark:bg-slate-800 border-2 border-transparent hover:bg-slate-100 dark:hover:bg-slate-700"
                      }`}
                    >
                      <div className="flex items-center gap-2">
                        <span className="text-2xl">{cat.icon}</span>
                        <div>
                          <div className="font-medium text-slate-900 dark:text-slate-100 text-sm">
                            {cat.name}
                          </div>
                        </div>
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  标题 *
                </label>
                <input
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  required
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="输入世界观标题"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  内容 *
                </label>
                <textarea
                  value={content}
                  onChange={(e) => setContent(e.target.value)}
                  rows={8}
                  required
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                  placeholder="详细描述这个世界观设定"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  标签
                </label>
                <input
                  type="text"
                  value={tags}
                  onChange={(e) => setTags(e.target.value)}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="输入标签，用逗号分隔"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  状态
                </label>
                <select
                  value={status}
                  onChange={(e) => setStatus(e.target.value)}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="draft">草稿</option>
                  <option value="in_progress">进行中</option>
                  <option value="completed">已完成</option>
                </select>
              </div>
            </div>
          )}

          {activeTab === "timeline" && (
            <div className="space-y-4">
              {!worldView ? (
                <div className="text-center py-8 text-muted-foreground">
                  请先保存世界观后再添加时间线事件
                </div>
              ) : (
                <>
                  <div className="flex justify-between items-center">
                    <h3 className="font-medium">世界观事件时间线</h3>
                    <button
                      type="button"
                      onClick={() => {
                        resetEventForm();
                        setShowEventForm(true);
                      }}
                      className="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
                    >
                      + 添加事件
                    </button>
                  </div>

                  {showEventForm && (
                    <div className="border rounded-lg p-4 bg-gray-50 dark:bg-gray-800 space-y-3">
                      <div className="grid grid-cols-2 gap-3">
                        <div>
                          <label className="block text-sm font-medium mb-1">事件类型</label>
                          <select
                            value={eventForm.event_type}
                            onChange={(e) =>
                              setEventForm({ ...eventForm, event_type: e.target.value })
                            }
                            className="w-full px-3 py-2 border border-border rounded-md"
                          >
                            {EVENT_TYPES.map((type) => (
                              <option key={type.value} value={type.value}>
                                {type.icon} {type.label}
                              </option>
                            ))}
                          </select>
                        </div>
                        <div>
                          <label className="block text-sm font-medium mb-1">事件标题 *</label>
                          <input
                            type="text"
                            value={eventForm.event_title}
                            onChange={(e) =>
                              setEventForm({ ...eventForm, event_title: e.target.value })
                            }
                            className="w-full px-3 py-2 border border-border rounded-md"
                            placeholder="简要描述事件"
                          />
                        </div>
                      </div>

                      <div>
                        <label className="block text-sm font-medium mb-1">事件描述</label>
                        <textarea
                          value={eventForm.event_description}
                          onChange={(e) =>
                            setEventForm({ ...eventForm, event_description: e.target.value })
                          }
                          rows={3}
                          className="w-full px-3 py-2 border border-border rounded-md resize-none"
                          placeholder="详细描述事件经过..."
                        />
                      </div>

                      <div className="grid grid-cols-2 gap-3">
                        <div>
                          <label className="block text-sm font-medium mb-1">故事时间</label>
                          <input
                            type="text"
                            value={eventForm.story_time}
                            onChange={(e) =>
                              setEventForm({ ...eventForm, story_time: e.target.value })
                            }
                            className="w-full px-3 py-2 border border-border rounded-md"
                            placeholder="如：第一纪元、三千年前..."
                          />
                        </div>
                        <div>
                          <label className="block text-sm font-medium mb-1">影响范围</label>
                          <input
                            type="text"
                            value={eventForm.impact_scope}
                            onChange={(e) =>
                              setEventForm({ ...eventForm, impact_scope: e.target.value })
                            }
                            className="w-full px-3 py-2 border border-border rounded-md"
                            placeholder="如：全球、局部、特定种族..."
                          />
                        </div>
                      </div>

                      <div>
                        <label className="block text-sm font-medium mb-1">相关角色</label>
                        <input
                          type="text"
                          value={eventForm.related_characters}
                          onChange={(e) =>
                            setEventForm({ ...eventForm, related_characters: e.target.value })
                          }
                          className="w-full px-3 py-2 border border-border rounded-md"
                          placeholder="与事件相关的角色名称，用逗号分隔..."
                        />
                      </div>

                      <div className="flex justify-end gap-2">
                        <button
                          type="button"
                          onClick={resetEventForm}
                          className="px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground"
                        >
                          取消
                        </button>
                        <button
                          type="button"
                          onClick={editingEvent ? handleUpdateEvent : handleCreateEvent}
                          disabled={!eventForm.event_title.trim()}
                          className="px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50"
                        >
                          {editingEvent ? "更新" : "添加"}
                        </button>
                      </div>
                    </div>
                  )}

                  {isLoadingTimeline ? (
                    <div className="text-center py-4 text-muted-foreground">加载中...</div>
                  ) : timelineEvents.length === 0 ? (
                    <div className="text-center py-8 text-muted-foreground">
                      暂无时间线事件，点击上方按钮添加
                    </div>
                  ) : (
                    <div className="space-y-3">
                      {timelineEvents.map((event, index) => {
                        const typeInfo = getEventTypeInfo(event.event_type);
                        return (
                          <div
                            key={event.id}
                            className="border rounded-lg p-4 hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors"
                          >
                            <div className="flex items-start justify-between">
                              <div className="flex items-start gap-3">
                                <div className="flex flex-col items-center">
                                  <span className="text-lg">{typeInfo.icon}</span>
                                  <span className="text-xs text-muted-foreground mt-1">
                                    #{index + 1}
                                  </span>
                                </div>
                                <div className="flex-1">
                                  <div className="flex items-center gap-2">
                                    <h4 className="font-medium">{event.event_title}</h4>
                                    <span className="text-xs px-2 py-0.5 bg-gray-100 dark:bg-gray-700 rounded">
                                      {typeInfo.label}
                                    </span>
                                  </div>
                                  {event.story_time && (
                                    <p className="text-sm text-muted-foreground mt-1">
                                      📖 {event.story_time}
                                    </p>
                                  )}
                                  {event.event_description && (
                                    <p className="text-sm mt-2 text-gray-600 dark:text-gray-300">
                                      {event.event_description}
                                    </p>
                                  )}
                                  <div className="flex gap-4 mt-2 text-sm">
                                    {event.impact_scope && (
                                      <span className="text-purple-600 dark:text-purple-400">
                                        🎯 {event.impact_scope}
                                      </span>
                                    )}
                                    {event.related_characters && (
                                      <span className="text-green-600 dark:text-green-400">
                                        👥 {event.related_characters}
                                      </span>
                                    )}
                                  </div>
                                </div>
                              </div>
                              <div className="flex gap-2">
                                <button
                                  type="button"
                                  onClick={() => startEditEvent(event)}
                                  className="text-sm text-blue-500 hover:text-blue-700"
                                >
                                  编辑
                                </button>
                                <button
                                  type="button"
                                  onClick={() => handleDeleteEvent(event.id)}
                                  className="text-sm text-red-500 hover:text-red-700"
                                >
                                  删除
                                </button>
                              </div>
                            </div>
                          </div>
                        );
                      })}
                    </div>
                  )}
                </>
              )}
            </div>
          )}

          <div className="flex justify-end gap-3 pt-4 border-t mt-4">
            <button
              type="button"
              onClick={onClose}
              disabled={loading}
              className="px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded font-medium disabled:opacity-50"
            >
              取消
            </button>
            <button
              type="submit"
              disabled={loading || !title.trim() || !content.trim()}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded font-medium disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <Save className="w-4 h-4" />
              {loading ? "保存中..." : "保存"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
