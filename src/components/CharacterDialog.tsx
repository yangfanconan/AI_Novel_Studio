import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Character, CharacterTimelineEvent, CreateCharacterTimelineEventRequest } from '../types';

interface CharacterDialogProps {
  isOpen: boolean;
  character?: Character;
  initialName?: string;
  onSubmit: (data: Partial<Character> & { name: string }) => void;
  onCancel: () => void;
}

const ROLE_TYPES = [
  { value: 'protagonist', label: '主角' },
  { value: 'deuteragonist', label: '第二主角' },
  { value: 'antagonist', label: '反派' },
  { value: 'supporting', label: '配角' },
  { value: 'minor', label: '小角色' },
];

const MBTI_TYPES = [
  'INTJ', 'INTP', 'ENTJ', 'ENTP',
  'INFJ', 'INFP', 'ENFJ', 'ENFP',
  'ISTJ', 'ISFJ', 'ESTJ', 'ESFJ',
  'ISTP', 'ISFP', 'ESTP', 'ESFP',
];

const ENNEAGRAM_TYPES = [
  '1号-完美型', '2号-助人型', '3号-成就型', '4号-自我型',
  '5号-理智型', '6号-疑惑型', '7号-活跃型', '8号-领袖型', '9号-和平型',
];

const EVENT_TYPES = [
  { value: 'birth', label: '出生', icon: '🎂' },
  { value: 'milestone', label: '里程碑', icon: '🏁' },
  { value: 'relationship', label: '关系变化', icon: '💔' },
  { value: 'ability', label: '能力获得', icon: '⚡' },
  { value: 'item', label: '物品获取', icon: '🎁' },
  { value: 'trauma', label: '创伤事件', icon: '💢' },
  { value: 'achievement', label: '成就达成', icon: '🏆' },
  { value: 'death', label: '死亡', icon: '💀' },
  { value: 'other', label: '其他', icon: '📝' },
];

export function CharacterDialog({
  isOpen,
  character,
  initialName,
  onSubmit,
  onCancel,
}: CharacterDialogProps) {
  const [activeTab, setActiveTab] = useState<'basic' | 'personality' | 'ability' | 'timeline'>('basic');
  const [name, setName] = useState('');
  const [roleType, setRoleType] = useState('');
  const [race, setRace] = useState('');
  const [age, setAge] = useState<number | undefined>();
  const [gender, setGender] = useState('');
  const [birthDate, setBirthDate] = useState('');
  const [appearance, setAppearance] = useState('');
  const [personality, setPersonality] = useState('');
  const [background, setBackground] = useState('');
  const [skills, setSkills] = useState('');
  const [status, setStatus] = useState('');
  const [bazi, setBazi] = useState('');
  const [ziwei, setZiwei] = useState('');
  const [mbti, setMbti] = useState('');
  const [enneagram, setEnneagram] = useState('');
  const [items, setItems] = useState('');

  const [timelineEvents, setTimelineEvents] = useState<CharacterTimelineEvent[]>([]);
  const [isLoadingTimeline, setIsLoadingTimeline] = useState(false);
  const [showEventForm, setShowEventForm] = useState(false);
  const [editingEvent, setEditingEvent] = useState<CharacterTimelineEvent | null>(null);
  const [eventForm, setEventForm] = useState({
    event_type: 'milestone',
    event_title: '',
    event_description: '',
    story_time: '',
    emotional_state: '',
    state_changes: '',
  });

  useEffect(() => {
    if (isOpen) {
      if (character) {
        setName(character.name);
        setRoleType(character.role_type || '');
        setRace(character.race || '');
        setAge(character.age);
        setGender(character.gender || '');
        setBirthDate(character.birth_date || '');
        setAppearance(character.appearance || '');
        setPersonality(character.personality || '');
        setBackground(character.background || '');
        setSkills(character.skills || '');
        setStatus(character.status || '');
        setBazi(character.bazi || '');
        setZiwei(character.ziwei || '');
        setMbti(character.mbti || '');
        setEnneagram(character.enneagram || '');
        setItems(character.items || '');
        loadTimelineEvents(character.id);
      } else {
        setName(initialName || '');
        setRoleType('');
        setRace('');
        setAge(undefined);
        setGender('');
        setBirthDate('');
        setAppearance('');
        setPersonality('');
        setBackground('');
        setSkills('');
        setStatus('');
        setBazi('');
        setZiwei('');
        setMbti('');
        setEnneagram('');
        setItems('');
        setTimelineEvents([]);
      }
      setActiveTab('basic');
      setShowEventForm(false);
      setEditingEvent(null);
    }
  }, [isOpen, character, initialName]);

  const loadTimelineEvents = async (characterId: string) => {
    setIsLoadingTimeline(true);
    try {
      const events = await invoke<CharacterTimelineEvent[]>('get_character_timeline', {
        characterId,
      });
      setTimelineEvents(events);
    } catch (error) {
      console.error('Failed to load timeline events:', error);
      setTimelineEvents([]);
    } finally {
      setIsLoadingTimeline(false);
    }
  };

  const handleCreateEvent = async () => {
    if (!character || !eventForm.event_title.trim()) return;

    try {
      const request: CreateCharacterTimelineEventRequest = {
        character_id: character.id,
        event_type: eventForm.event_type,
        event_title: eventForm.event_title,
        event_description: eventForm.event_description,
        story_time: eventForm.story_time || undefined,
        emotional_state: eventForm.emotional_state || undefined,
        state_changes: eventForm.state_changes || undefined,
        sort_order: timelineEvents.length,
      };

      const newEvent = await invoke<CharacterTimelineEvent>('create_character_timeline_event', {
        request,
      });
      setTimelineEvents([...timelineEvents, newEvent]);
      resetEventForm();
    } catch (error) {
      console.error('Failed to create event:', error);
    }
  };

  const handleUpdateEvent = async () => {
    if (!editingEvent) return;

    try {
      const updatedEvent = await invoke<CharacterTimelineEvent>(
        'update_character_timeline_event',
        {
          eventId: editingEvent.id,
          request: {
            event_type: eventForm.event_type,
            event_title: eventForm.event_title,
            event_description: eventForm.event_description,
            story_time: eventForm.story_time || null,
            emotional_state: eventForm.emotional_state || null,
            state_changes: eventForm.state_changes || null,
          },
        }
      );
      setTimelineEvents(
        timelineEvents.map((e) => (e.id === updatedEvent.id ? updatedEvent : e))
      );
      resetEventForm();
    } catch (error) {
      console.error('Failed to update event:', error);
    }
  };

  const handleDeleteEvent = async (eventId: string) => {
    if (!confirm('确定要删除这个事件吗？')) return;

    try {
      await invoke('delete_character_timeline_event', { eventId });
      setTimelineEvents(timelineEvents.filter((e) => e.id !== eventId));
    } catch (error) {
      console.error('Failed to delete event:', error);
    }
  };

  const resetEventForm = () => {
    setShowEventForm(false);
    setEditingEvent(null);
    setEventForm({
      event_type: 'milestone',
      event_title: '',
      event_description: '',
      story_time: '',
      emotional_state: '',
      state_changes: '',
    });
  };

  const startEditEvent = (event: CharacterTimelineEvent) => {
    setEditingEvent(event);
    setEventForm({
      event_type: event.event_type,
      event_title: event.event_title,
      event_description: event.event_description,
      story_time: event.story_time || '',
      emotional_state: event.emotional_state || '',
      state_changes: event.state_changes || '',
    });
    setShowEventForm(true);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (name.trim()) {
      onSubmit({
        name: name.trim(),
        role_type: roleType || undefined,
        race: race || undefined,
        age,
        gender: gender || undefined,
        birth_date: birthDate || undefined,
        appearance: appearance || undefined,
        personality: personality || undefined,
        background: background || undefined,
        skills: skills || undefined,
        status: status || undefined,
        bazi: bazi || undefined,
        ziwei: ziwei || undefined,
        mbti: mbti || undefined,
        enneagram: enneagram || undefined,
        items: items || undefined,
      });
    }
  };

  const getEventTypeInfo = (type: string) => {
    return EVENT_TYPES.find((t) => t.value === type) || EVENT_TYPES[EVENT_TYPES.length - 1];
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-lg shadow-lg w-full max-w-4xl p-6 max-h-[90vh] overflow-hidden flex flex-col">
        <h2 className="text-lg font-semibold mb-4">
          {character ? '编辑角色' : '新建角色'}
        </h2>

        <div className="flex border-b border-gray-200 mb-4">
          {[
            { id: 'basic', label: '基本信息', icon: '👤' },
            { id: 'personality', label: '性格分析', icon: '🧠' },
            { id: 'ability', label: '能力装备', icon: '⚔️' },
            { id: 'timeline', label: '事件时间线', icon: '📅' },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id as typeof activeTab)}
              className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              {tab.icon} {tab.label}
            </button>
          ))}
        </div>

        <form onSubmit={handleSubmit} className="flex-1 overflow-y-auto">
          {activeTab === 'basic' && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">角色名称 *</label>
                  <input
                    type="text"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">角色身份</label>
                  <select
                    value={roleType}
                    onChange={(e) => setRoleType(e.target.value)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  >
                    <option value="">请选择</option>
                    {ROLE_TYPES.map((type) => (
                      <option key={type.value} value={type.value}>
                        {type.label}
                      </option>
                    ))}
                  </select>
                </div>
              </div>

              <div className="grid grid-cols-3 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">种族</label>
                  <input
                    type="text"
                    value={race}
                    onChange={(e) => setRace(e.target.value)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                    placeholder="如：人类、精灵..."
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">年龄</label>
                  <input
                    type="number"
                    value={age || ''}
                    onChange={(e) => setAge(e.target.value ? parseInt(e.target.value) : undefined)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">性别</label>
                  <select
                    value={gender}
                    onChange={(e) => setGender(e.target.value)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  >
                    <option value="">请选择</option>
                    <option value="男">男</option>
                    <option value="女">女</option>
                    <option value="其他">其他</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">出生日期/时间</label>
                <input
                  type="text"
                  value={birthDate}
                  onChange={(e) => setBirthDate(e.target.value)}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  placeholder="如：龙历三千年三月初三"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">外貌描述</label>
                <textarea
                  value={appearance}
                  onChange={(e) => setAppearance(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">性格特点</label>
                <textarea
                  value={personality}
                  onChange={(e) => setPersonality(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">背景故事</label>
                <textarea
                  value={background}
                  onChange={(e) => setBackground(e.target.value)}
                  rows={4}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                />
              </div>
            </div>
          )}

          {activeTab === 'personality' && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">MBTI 人格类型</label>
                  <select
                    value={mbti}
                    onChange={(e) => setMbti(e.target.value)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  >
                    <option value="">请选择</option>
                    {MBTI_TYPES.map((type) => (
                      <option key={type} value={type}>
                        {type}
                      </option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">九型人格</label>
                  <select
                    value={enneagram}
                    onChange={(e) => setEnneagram(e.target.value)}
                    className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  >
                    <option value="">请选择</option>
                    {ENNEAGRAM_TYPES.map((type) => (
                      <option key={type} value={type}>
                        {type}
                      </option>
                    ))}
                  </select>
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">八字</label>
                <input
                  type="text"
                  value={bazi}
                  onChange={(e) => setBazi(e.target.value)}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                  placeholder="如：甲子年乙丑月丙寅日丁卯时"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">紫微斗数</label>
                <textarea
                  value={ziwei}
                  onChange={(e) => setZiwei(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                  placeholder="命宫、身宫等主要星曜配置..."
                />
              </div>
            </div>
          )}

          {activeTab === 'ability' && (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">技能列表</label>
                <textarea
                  value={skills}
                  onChange={(e) => setSkills(e.target.value)}
                  rows={4}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                  placeholder="列出角色掌握的技能，每行一个..."
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">当前状态</label>
                <textarea
                  value={status}
                  onChange={(e) => setStatus(e.target.value)}
                  rows={3}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                  placeholder="角色的当前状态、健康状况、情绪等..."
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">持有物品</label>
                <textarea
                  value={items}
                  onChange={(e) => setItems(e.target.value)}
                  rows={4}
                  className="w-full px-3 py-2 border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                  placeholder="角色随身携带的重要物品，每行一个..."
                />
              </div>
            </div>
          )}

          {activeTab === 'timeline' && (
            <div className="space-y-4">
              {!character ? (
                <div className="text-center py-8 text-muted-foreground">
                  请先创建角色后再添加时间线事件
                </div>
              ) : (
                <>
                  <div className="flex justify-between items-center">
                    <h3 className="font-medium">角色事件时间线</h3>
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
                            placeholder="如：第一章、三年后..."
                          />
                        </div>
                        <div>
                          <label className="block text-sm font-medium mb-1">情绪状态</label>
                          <input
                            type="text"
                            value={eventForm.emotional_state}
                            onChange={(e) =>
                              setEventForm({ ...eventForm, emotional_state: e.target.value })
                            }
                            className="w-full px-3 py-2 border border-border rounded-md"
                            placeholder="如：悲伤、愤怒、喜悦..."
                          />
                        </div>
                      </div>

                      <div>
                        <label className="block text-sm font-medium mb-1">状态变化</label>
                        <textarea
                          value={eventForm.state_changes}
                          onChange={(e) =>
                            setEventForm({ ...eventForm, state_changes: e.target.value })
                          }
                          rows={2}
                          className="w-full px-3 py-2 border border-border rounded-md resize-none"
                          placeholder="事件导致的角色状态变化，如获得能力、失去物品等..."
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
                          {editingEvent ? '更新' : '添加'}
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
                                    {event.emotional_state && (
                                      <span className="text-blue-600 dark:text-blue-400">
                                        😢 {event.emotional_state}
                                      </span>
                                    )}
                                    {event.state_changes && (
                                      <span className="text-green-600 dark:text-green-400">
                                        🔄 {event.state_changes}
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

          <div className="flex justify-end gap-2 pt-4 border-t mt-4">
            <button
              type="button"
              onClick={onCancel}
              className="px-4 py-2 text-sm text-muted-foreground hover:text-foreground transition-colors"
            >
              取消
            </button>
            <button
              type="submit"
              disabled={!name.trim()}
              className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {character ? '更新' : '创建'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
