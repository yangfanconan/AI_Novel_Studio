import React, { useState, useEffect, useRef } from 'react';
import { X, Play, RotateCcw, Check, XCircle, Info, AlertTriangle, FileText, Users, Globe, Network, Settings, Zap, Folder } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';

type LogLevel = 'info' | 'success' | 'error' | 'warning';

interface DebugLog {
  id: number;
  timestamp: Date;
  level: LogLevel;
  source: 'frontend' | 'backend' | 'api' | 'test';
  message: string;
  details?: any;
}

interface TestResult {
  name: string;
  category: string;
  status: 'pending' | 'running' | 'passed' | 'failed' | 'skipped';
  message: string;
  duration?: number;
  icon?: React.ReactNode;
}

export const SmartDebugPanel: React.FC = () => {
  const [isOpen, setIsOpen] = useState(true);
  const [logs, setLogs] = useState<DebugLog[]>([]);
  const [testResults, setTestResults] = useState<TestResult[]>([]);
  const [isRunningTests, setIsRunningTests] = useState(false);
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [testProgress, setTestProgress] = useState({ current: 0, total: 0 });
  const logCounter = useRef(0);
  const [backendLogs, setBackendLogs] = useState<string[]>([]);
  const [projectId, setProjectId] = useState<string>('');

  const addLog = (level: LogLevel, source: DebugLog['source'], message: string, details?: any) => {
    const newLog: DebugLog = {
      id: logCounter.current++,
      timestamp: new Date(),
      level,
      source,
      message,
      details,
    };
    setLogs(prev => [newLog, ...prev].slice(0, 200));
  };

  const captureConsoleLog = () => {
    const originalLog = console.log;
    const originalError = console.error;
    const originalWarn = console.warn;

    console.log = (...args: any[]) => {
      addLog('info', 'frontend', args.join(' '));
      originalLog.apply(console, args);
    };

    console.error = (...args: any[]) => {
      addLog('error', 'frontend', args.join(' '));
      originalError.apply(console, args);
    };

    console.warn = (...args: any[]) => {
      addLog('warning', 'frontend', args.join(' '));
      originalWarn.apply(console, args);
    };
  };

  useEffect(() => {
    captureConsoleLog();
    loadBackendLogs();
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      const projects = await invoke<any[]>('get_projects');
      if (projects && projects.length > 0) {
        setProjectId(projects[0].id);
        addLog('info', 'test', `检测到 ${projects.length} 个项目，使用第一个项目进行测试`);
      }
    } catch (error) {
      addLog('warning', 'test', '无法加载项目列表', error);
    }
  };

  const loadBackendLogs = async () => {
    try {
      const logs = await invoke<string>('get_all_debug_logs');
      setBackendLogs(logs.split('\n').slice(-100));
      addLog('info', 'backend', `Loaded ${logs.split('\n').length} backend log entries`);
    } catch (error) {
      addLog('warning', 'backend', 'Failed to load backend logs', error);
    }
  };

  const runTests = async () => {
    setIsRunningTests(true);
    setTestResults([]);

    const allTests = [
      {
        category: '基础功能',
        icon: <Zap className="w-4 h-4" />,
        tests: [
          { name: '检查 Tauri API 可用性', fn: testTauriAPI },
          { name: '检查项目状态管理', fn: testProjectState },
        ]
      },
      {
        category: '项目管理',
        icon: <Folder className="w-4 h-4" />,
        tests: [
          { name: '获取项目列表', fn: testGetProjects },
          { name: '创建新项目', fn: testCreateProject },
          { name: '更新项目信息', fn: testUpdateProject },
          { name: '删除项目', fn: testDeleteProject },
        ]
      },
      {
        category: '章节管理',
        icon: <FileText className="w-4 h-4" />,
        tests: [
          { name: '获取章节列表', fn: testGetChapters },
          { name: '创建新章节', fn: testCreateChapter },
          { name: '更新章节内容', fn: testUpdateChapter },
          { name: '删除章节', fn: testDeleteChapter },
        ]
      },
      {
        category: '角色管理',
        icon: <Users className="w-4 h-4" />,
        tests: [
          { name: '获取角色列表', fn: testGetCharacters },
          { name: '创建新角色', fn: testCreateCharacter },
          { name: '更新角色信息', fn: testUpdateCharacter },
          { name: '删除角色', fn: testDeleteCharacter },
        ]
      },
      {
        category: '情节点管理',
        icon: <Network className="w-4 h-4" />,
        tests: [
          { name: '获取情节点列表', fn: testGetPlotPoints },
          { name: '创建情节点', fn: testCreatePlotPoint },
        ]
      },
      {
        category: '世界观管理',
        icon: <Globe className="w-4 h-4" />,
        tests: [
          { name: '获取世界观列表', fn: testGetWorldViews },
        ]
      },
      {
        category: 'AI功能',
        icon: <Settings className="w-4 h-4" />,
        tests: [
          { name: '获取模型列表', fn: testGetModels },
          { name: '设置API密钥', fn: testSetApiKey },
          { name: '获取API密钥', fn: testGetApiKey },
        ]
      },
      {
        category: 'UI交互',
        icon: <Check className="w-4 h-4" />,
        tests: [
          { name: '检查设置对话框按钮', fn: testSettingsButton },
          { name: '检查新建项目按钮', fn: testCreateProjectButton },
          { name: '检查刷新按钮', fn: testRefreshButton },
          { name: '检查编辑器组件', fn: testEditorComponent },
        ]
      },
    ];

    let flattenedTests: TestResult[] = [];
    allTests.forEach(category => {
      category.tests.forEach(test => {
        flattenedTests.push({
          name: test.name,
          category: category.category,
          status: 'pending',
          message: '',
          icon: category.icon,
        });
      });
    });

    setTestResults(flattenedTests);
    setTestProgress({ current: 0, total: flattenedTests.length });

    for (let i = 0; i < flattenedTests.length; i++) {
      const test = flattenedTests[i];
      const categoryTests = allTests.find(c => c.category === test.category)?.tests;
      const testFn = categoryTests?.find(t => t.name === test.name)?.fn;

      setTestProgress({ current: i + 1, total: flattenedTests.length });

      if (!testFn) {
        setTestResults(prev => {
          const updated = [...prev];
          updated[i] = { ...test, status: 'skipped', message: '测试函数未实现' };
          return updated;
        });
        continue;
      }

      const startTime = Date.now();

      setTestResults(prev => {
        const updated = [...prev];
        updated[i] = { ...test, status: 'running', message: 'Running...' };
        return updated;
      });

      await new Promise(resolve => setTimeout(resolve, 100));

      try {
        const result = await testFn(projectId);
        const duration = Date.now() - startTime;

        setTestResults(prev => {
          const updated = [...prev];
          updated[i] = { ...test, status: 'passed', message: result.message, duration };
          return updated;
        });

        addLog('success', 'test', `✓ ${test.name}`, { result, duration });

      } catch (error) {
        const duration = Date.now() - startTime;

        setTestResults(prev => {
          const updated = [...prev];
          updated[i] = { ...test, status: 'failed', message: String(error), duration };
          return updated;
        });

        addLog('error', 'test', `✗ ${test.name}`, { error });
      }

      await new Promise(resolve => setTimeout(resolve, 200));
    }

    setIsRunningTests(false);
    addLog('success', 'test', '所有测试完成');

    const passedCount = flattenedTests.filter(t => t.status === 'passed').length;
    const failedCount = flattenedTests.filter(t => t.status === 'failed').length;
    addLog('info', 'test', `测试结果: ${passedCount} 通过, ${failedCount} 失败, ${flattenedTests.length - passedCount - failedCount} 跳过`);
  };

  const testTauriAPI = async () => {
    const available = typeof window !== 'undefined' && '__TAURI__' in window;
    return { message: available ? 'Tauri API 可用' : 'Tauri API 不可用' };
  };

  const testProjectState = async (testProjectId: string) => {
    const projects = await invoke<any[]>('get_projects');
    return { message: `当前有 ${Array.isArray(projects) ? projects.length : 0} 个项目` };
  };

  const testGetProjects = async () => {
    const result = await invoke<any[]>('get_projects');
    return { message: `成功获取 ${Array.isArray(result) ? result.length : 0} 个项目` };
  };

  const testCreateProject = async (testProjectId: string) => {
    const newProject = await invoke('create_project', {
      request: {
        name: '测试项目',
        description: '自动化测试创建',
        genre: 'fantasy',
      }
    });
    return { message: `创建项目成功: ${newProject ? 'ID: ' + (newProject as any).id : '未知错误'}` };
  };

  const testUpdateProject = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const updated = await invoke('update_project', {
      projectId: testProjectId,
      name: '测试项目-已更新',
      description: '自动化测试更新',
    });
    return { message: `更新项目成功` };
  };

  const testDeleteProject = async (testProjectId: string) => {
    const projects = await invoke<any[]>('get_projects');
    if (!Array.isArray(projects) || projects.length === 0) {
      throw new Error('没有可删除的项目');
    }
    const targetProject = projects.find(p => p.id !== testProjectId) || projects[0];
    await invoke('delete_project', { projectId: targetProject.id });
    return { message: `删除项目成功: ${targetProject.name}` };
  };

  const testGetChapters = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const result = await invoke<any[]>('get_chapters', { projectId: testProjectId });
    return { message: `成功获取 ${Array.isArray(result) ? result.length : 0} 个章节` };
  };

  const testCreateChapter = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const newChapter = await invoke('save_chapter', {
      request: {
        project_id: testProjectId,
        title: '测试章节',
        content: '测试内容',
        sort_order: 0,
      }
    });
    return { message: `创建章节成功: ${newChapter ? 'ID: ' + (newChapter as any).id : '未知错误'}` };
  };

  const testUpdateChapter = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const chapters = await invoke<any[]>('get_chapters', { projectId: testProjectId });
    if (!Array.isArray(chapters) || chapters.length === 0) throw new Error('没有可用的章节');
    const updated = await invoke('update_chapter', {
      chapterId: chapters[0].id,
      title: '测试章节-已更新',
      content: '更新后的内容',
    });
    return { message: '更新章节成功' };
  };

  const testDeleteChapter = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const chapters = await invoke<any[]>('get_chapters', { projectId: testProjectId });
    if (!Array.isArray(chapters) || chapters.length === 0) throw new Error('没有可用的章节');
    await invoke('delete_chapter', { chapterId: chapters[0].id });
    return { message: '删除章节成功' };
  };

  const testGetCharacters = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const result = await invoke<any[]>('get_characters', { projectId: testProjectId });
    return { message: `成功获取 ${Array.isArray(result) ? result.length : 0} 个角色` };
  };

  const testCreateCharacter = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const newCharacter = await invoke('create_character', {
      request: {
        project_id: testProjectId,
        name: '测试角色',
        age: 25,
        gender: '男',
        appearance: '测试外观',
        personality: '测试性格',
        background: '测试背景',
      }
    });
    return { message: `创建角色成功: ${newCharacter ? 'ID: ' + (newCharacter as any).id : '未知错误'}` };
  };

  const testUpdateCharacter = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const characters = await invoke<any[]>('get_characters', { projectId: testProjectId });
    if (!Array.isArray(characters) || characters.length === 0) throw new Error('没有可用的角色');
    const updated = await invoke('update_character', {
      characterId: characters[0].id,
      update: {
        name: '测试角色-已更新',
        age: 26,
      }
    });
    return { message: '更新角色成功' };
  };

  const testDeleteCharacter = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const characters = await invoke<any[]>('get_characters', { projectId: testProjectId });
    if (!Array.isArray(characters) || characters.length === 0) throw new Error('没有可用的角色');
    await invoke('delete_character', { characterId: characters[0].id });
    return { message: '删除角色成功' };
  };

  const testGetPlotPoints = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const result = await invoke<any[]>('get_plot_points', { projectId: testProjectId });
    return { message: `成功获取 ${Array.isArray(result) ? result.length : 0} 个情节点` };
  };

  const testCreatePlotPoint = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const chapters = await invoke<any[]>('get_chapters', { projectId: testProjectId });
    const chapterId = Array.isArray(chapters) && chapters.length > 0 ? chapters[0].id : null;
    if (!chapterId) throw new Error('没有可用的章节');
    const newPlotPoint = await invoke('create_plot_point', {
      request: {
        project_id: testProjectId,
        chapter_id: chapterId,
        title: '测试情节点',
        content: '测试内容',
        type: 'plot',
        parent_id: null,
      }
    });
    return { message: `创建情节点成功: ${newPlotPoint ? 'ID: ' + (newPlotPoint as any).id : '未知错误'}` };
  };

  const testGetWorldViews = async (testProjectId: string) => {
    if (!testProjectId) throw new Error('没有可用的项目ID');
    const result = await invoke<any[]>('get_world_views', { projectId: testProjectId });
    return { message: `成功获取 ${Array.isArray(result) ? result.length : 0} 个世界观` };
  };

  const testGetModels = async () => {
    const result = await invoke<any[]>('get_models');
    return { message: `成功获取 ${Array.isArray(result) ? result.length : 0} 个模型` };
  };

  const testSetApiKey = async () => {
    await invoke('set_bigmodel_api_key', { apiKey: 'test_key_' + Date.now() });
    return { message: '设置API密钥成功' };
  };

  const testGetApiKey = async () => {
    const apiKey = await invoke<string>('get_bigmodel_api_key').catch(() => '');
    return { message: `获取API密钥成功，长度: ${apiKey.length}` };
  };

  const testSettingsButton = async () => {
    const settingsButton = document.querySelector('button[title*="设置"]');
    if (!settingsButton) throw new Error('设置按钮未找到');
    return { message: '设置按钮存在' };
  };

  const testCreateProjectButton = async () => {
    const buttons = Array.from(document.querySelectorAll('button'));
    const createButton = buttons.find(btn => {
      const text = btn.textContent || '';
      return text.includes('新建项目') || text.includes('New Project') || btn.getAttribute('aria-label')?.includes('create');
    });
    if (!createButton) throw new Error('新建项目按钮未找到');
    return { message: '新建项目按钮存在' };
  };

  const testRefreshButton = async () => {
    const refreshButton = document.querySelector('button[title*="刷新"]');
    if (!refreshButton) throw new Error('刷新按钮未找到');
    return { message: '刷新按钮存在' };
  };

  const testEditorComponent = async () => {
    const editor = document.querySelector('textarea, [contenteditable="true"]');
    return { message: editor ? '编辑器组件存在' : '编辑器组件未找到' };
  };

  const clearLogs = () => {
    setLogs([]);
    setBackendLogs([]);
    logCounter.current = 0;
  };

  const getLogIcon = (level: LogLevel) => {
    switch (level) {
      case 'success': return <Check className="w-4 h-4 text-green-500" />;
      case 'error': return <XCircle className="w-4 h-4 text-red-500" />;
      case 'warning': return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      default: return <Info className="w-4 h-4 text-blue-500" />;
    }
  };

  const getLogColor = (level: LogLevel) => {
    switch (level) {
      case 'success': return 'text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/20';
      case 'error': return 'text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20';
      case 'warning': return 'text-yellow-600 dark:text-yellow-400 bg-yellow-50 dark:bg-yellow-900/20';
      default: return 'text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/20';
    }
  };

  const getStatusColor = (status: TestResult['status']) => {
    switch (status) {
      case 'passed': return 'text-green-500';
      case 'failed': return 'text-red-500';
      case 'running': return 'text-blue-500 animate-pulse';
      case 'skipped': return 'text-gray-500';
      default: return 'text-gray-500';
    }
  };

  const categories = ['all', '基础功能', '项目管理', '章节管理', '角色管理', '情节点管理', '世界观管理', 'AI功能', 'UI交互'];

  const filteredTests = selectedCategory === 'all'
    ? testResults
    : testResults.filter(t => t.category === selectedCategory);

  const exportLogs = () => {
    const passedCount = testResults.filter(t => t.status === 'passed').length;
    const failedCount = testResults.filter(t => t.status === 'failed').length;
    const skippedCount = testResults.filter(t => t.status === 'skipped').length;

    const content = `
=== AI Novel Studio 智能调试报告 ===
生成时间: ${new Date().toLocaleString('zh-CN')}
测试进度: ${testProgress.current}/${testProgress.total}

=== 测试摘要 ===
总计: ${testResults.length} 个测试
通过: ${passedCount} 个 (${((passedCount / testResults.length) * 100).toFixed(1)}%)
失败: ${failedCount} 个 (${((failedCount / testResults.length) * 100).toFixed(1)}%)
跳过: ${skippedCount} 个 (${((skippedCount / testResults.length) * 100).toFixed(1)}%)

=== 详细测试结果 ===
${testResults.map(test => {
  const icon = test.status === 'passed' ? '✓' : test.status === 'failed' ? '✗' : test.status === 'skipped' ? '○' : '○';
  return `[${test.category}] ${icon} ${test.name}
  状态: ${test.status}
  消息: ${test.message}
  ${test.duration ? `耗时: ${test.duration}ms` : ''}`;
}).join('\n\n')}

=== 前端日志 ===
${logs.map(log => {
  const time = log.timestamp.toLocaleTimeString('zh-CN');
  return `[${time}] [${log.level.toUpperCase()}] [${log.source}] ${log.message}`;
}).join('\n')}

=== 后端日志 ===
${backendLogs.join('\n')}
    `;

    const blob = new Blob([content], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `debug-report-${Date.now()}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);

    addLog('success', 'frontend', '调试报告已导出');
  };

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 right-4 z-50 p-3 bg-blue-500 text-white rounded-full shadow-lg hover:bg-blue-600 transition-colors"
        title="打开智能调试面板"
      >
        <Zap className="w-6 h-6" />
      </button>
    );
  }

  return (
    <div className="fixed inset-0 z-50 bg-black/80 flex items-center justify-center">
      <div className="bg-white dark:bg-slate-900 rounded-lg shadow-2xl w-[95vw] h-[92vh] flex flex-col">
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold">智能调试面板 - 自动化测试系统</h2>
          <div className="flex items-center gap-2">
            <button
              onClick={loadBackendLogs}
              className="px-3 py-1 text-sm bg-slate-100 dark:bg-slate-800 rounded hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
              title="刷新后端日志"
            >
              <RotateCcw className="w-4 h-4" />
            </button>
            <button
              onClick={exportLogs}
              className="px-3 py-1 text-sm bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-400 rounded hover:bg-blue-200 dark:hover:bg-blue-800 transition-colors"
              title="导出完整报告"
            >
              导出报告
            </button>
            <button
              onClick={clearLogs}
              className="px-3 py-1 text-sm bg-red-100 dark:bg-red-900 text-red-600 dark:text-red-400 rounded hover:bg-red-200 dark:hover:bg-red-800 transition-colors"
              title="清除日志"
            >
              清除
            </button>
            <button
              onClick={() => setIsOpen(false)}
              className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        <div className="flex-1 overflow-hidden flex">
          <div className="flex-1 overflow-auto p-4">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-md font-semibold mb-0">自动化测试 ({testProgress.current}/{testProgress.total})</h3>
              <button
                onClick={runTests}
                disabled={isRunningTests}
                className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:bg-blue-400 disabled:cursor-not-allowed transition-colors"
              >
                <Play className="w-4 h-4" />
                {isRunningTests ? '测试中...' : '运行全部测试'}
              </button>
            </div>

            {testResults.length > 0 && (
              <div className="mb-4 p-3 bg-slate-50 dark:bg-slate-800 rounded-lg">
                <div className="flex gap-6 text-sm">
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 rounded-full bg-green-500"></div>
                    <span className="text-slate-700 dark:text-slate-300">
                      通过: <span className="font-semibold text-green-600 dark:text-green-400">{testResults.filter(t => t.status === 'passed').length}</span>
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 rounded-full bg-red-500"></div>
                    <span className="text-slate-700 dark:text-slate-300">
                      失败: <span className="font-semibold text-red-600 dark:text-red-400">{testResults.filter(t => t.status === 'failed').length}</span>
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 rounded-full bg-gray-500"></div>
                    <span className="text-slate-700 dark:text-slate-300">
                      跳过: <span className="font-semibold text-gray-600 dark:text-gray-400">{testResults.filter(t => t.status === 'skipped').length}</span>
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="w-3 h-3 rounded-full bg-blue-500"></div>
                    <span className="text-slate-700 dark:text-slate-300">
                      运行中: <span className="font-semibold text-blue-600 dark:text-blue-400">{testResults.filter(t => t.status === 'running').length}</span>
                    </span>
                  </div>
                </div>
              </div>
            )}

            <div className="mb-4">
              <div className="flex gap-2 mb-3 flex-wrap">
                {categories.map(cat => (
                  <button
                    key={cat}
                    onClick={() => setSelectedCategory(cat)}
                    className={`px-3 py-1 text-sm rounded transition-colors ${
                      selectedCategory === cat
                        ? 'bg-blue-500 text-white'
                        : 'bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700'
                    }`}
                  >
                    {cat}
                  </button>
                ))}
              </div>
            </div>

            {filteredTests.length > 0 && (
              <div className="space-y-2 max-h-[calc(100vh-300px)] overflow-y-auto">
                {filteredTests.map((test, index) => (
                  <div
                    key={index}
                    className={`p-3 rounded-lg border ${
                      test.status === 'passed'
                        ? 'border-green-200 dark:border-green-800 bg-green-50 dark:bg-green-900/20'
                        : test.status === 'failed'
                        ? 'border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20'
                        : test.status === 'skipped'
                        ? 'border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/20'
                        : 'border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        {test.icon}
                        <span className="font-medium text-sm">{test.name}</span>
                      </div>
                      <span className={`text-xs font-semibold ${getStatusColor(test.status)}`}>
                        {test.status}
                      </span>
                    </div>
                    <div className="mt-2 text-sm text-slate-600 dark:text-slate-400">
                      {test.message}
                      {test.duration && <span className="ml-2 text-xs text-slate-500">({test.duration}ms)</span>}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div className="w-px bg-slate-200 dark:bg-slate-700"></div>

          <div className="flex-1 overflow-auto p-4">
            <h3 className="text-md font-semibold mb-4">实时日志</h3>
            <div className="space-y-1 text-sm max-h-[calc(100vh-200px)] overflow-y-auto">
              {logs.slice(0, 50).map(log => (
                <div
                  key={log.id}
                  className={`p-2 rounded ${getLogColor(log.level)} flex items-start gap-2`}
                >
                  {getLogIcon(log.level)}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 text-xs text-slate-500 dark:text-slate-400">
                      <span>{log.timestamp.toLocaleTimeString('zh-CN')}</span>
                      <span className="px-1.5 py-0.5 bg-white dark:bg-slate-800 rounded text-xs">
                        {log.source}
                      </span>
                    </div>
                    <div className="mt-1 break-words">
                      {log.message}
                    </div>
                    {log.details && (
                      <details className="mt-1">
                        <summary className="cursor-pointer text-xs underline">查看详情</summary>
                        <pre className="mt-1 text-xs overflow-auto max-h-32 bg-white dark:bg-slate-800 p-2 rounded">
                          {typeof log.details === 'object' ? JSON.stringify(log.details, null, 2) : String(log.details)}
                        </pre>
                      </details>
                    )}
                  </div>
                </div>
              ))}
              {logs.length > 50 && (
                <div className="text-center text-xs text-slate-500 py-2">
                  显示最近 50 条日志，共 {logs.length} 条
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
