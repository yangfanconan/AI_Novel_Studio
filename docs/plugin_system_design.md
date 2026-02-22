# AI小说创作工作室 - 插件系统与扩展机制设计

## 一、插件系统概述

### 1.1 设计目标
- **灵活性**：支持多种插件类型和开发语言
- **安全性**：沙箱隔离，权限控制
- **易用性**：简单的开发API，丰富的文档
- **可扩展**：支持插件间通信和协作
- **高性能**：最小化性能开销

### 1.2 插件系统架构

```
┌─────────────────────────────────────────────────────────────────┐
│                    Plugin Management System                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  Plugin Registry                          │  │
│  │  - 插件注册与发现                                          │  │
│  │  - 版本管理                                                │  │
│  │  - 依赖解析                                                │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  Plugin Lifecycle                         │  │
│  │  - load → activate → deactivate → unload                  │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  Plugin Sandbox                           │  │
│  │  - 安全隔离                                                │  │
│  │  - 权限控制                                                │  │
│  │  - 资源限制                                                │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 二、插件类型

### 2.1 内置插件类型

```typescript
enum PluginType {
  // 编辑器增强插件
  EDITOR_EXTENSION = 'editor_extension',
  
  // 功能模块插件
  FEATURE_MODULE = 'feature_module',
  
  // 主题插件
  THEME = 'theme',
  
  // 语言包插件
  LANGUAGE_PACK = 'language_pack',
  
  // AI模型适配器
  AI_ADAPTER = 'ai_adapter',
  
  // 导入导出插件
  IMPORT_EXPORT = 'import_export',
  
  // 工具插件
  UTILITY = 'utility',
  
  // 集成插件
  INTEGRATION = 'integration'
}
```

### 2.2 插件能力分类

```typescript
interface PluginCapabilities {
  // 编辑器扩展
  editor: {
    customCommands: boolean;       // 自定义命令
    contextMenu: boolean;          // 右键菜单
    toolbar: boolean;              // 工具栏
    statusBar: boolean;            // 状态栏
    keybindings: boolean;          // 快捷键
    syntaxHighlighting: boolean;   // 语法高亮
    autocomplete: boolean;         // 自动补全
  };
  
  // 项目访问
  project: {
    read: boolean;                 // 读取项目数据
    write: boolean;                // 写入项目数据
    create: boolean;               // 创建新项目
    export: boolean;               // 导出项目
  };
  
  // AI功能
  ai: {
    generate: boolean;             // AI生成
    chat: boolean;                 // AI对话
    embed: boolean;                // 向量嵌入
    customModels: boolean;         // 自定义模型
  };
  
  // 文件系统
  filesystem: {
    read: boolean;                 // 读取文件
    write: boolean;                // 写入文件
    watch: boolean;                // 监听文件变化
  };
  
  // 网络
  network: {
    http: boolean;                 // HTTP请求
    websocket: boolean;            // WebSocket
  };
  
  // UI扩展
  ui: {
    panels: boolean;               // 面板
    modals: boolean;               // 模态框
    notifications: boolean;        // 通知
    settings: boolean;             // 设置页面
  };
}
```

---

## 三、插件开发接口

### 3.1 插件清单 (plugin.json)

```json
{
  "id": "com.example.my-plugin",
  "name": "我的插件",
  "version": "1.0.0",
  "description": "一个示例插件",
  "author": {
    "name": "开发者名称",
    "email": "developer@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/plugin",
  "repository": {
    "type": "git",
    "url": "https://github.com/example/plugin"
  },
  "license": "MIT",
  "keywords": ["小说", "创作", "工具"],
  "type": "feature_module",
  
  "main": "dist/index.js",
  "icon": "assets/icon.png",
  "readme": "README.md",
  "changelog": "CHANGELOG.md",
  
  "engines": {
    "novelStudio": ">=1.0.0"
  },
  
  "categories": ["Writing", "AI"],
  "activationEvents": [
    "onCommand:myPlugin.hello",
    "onLanguage:novel",
    "onProjectOpen"
  ],
  
  "contributes": {
    "commands": [
      {
        "id": "myPlugin.hello",
        "title": "Say Hello",
        "category": "My Plugin",
        "icon": "assets/hello.svg"
      }
    ],
    "menus": {
      "editor/context": [
        {
          "command": "myPlugin.hello",
          "group": "navigation"
        }
      ]
    },
    "keybindings": [
      {
        "command": "myPlugin.hello",
        "key": "ctrl+shift+h",
        "mac": "cmd+shift+h"
      }
    ],
    "configuration": {
      "title": "My Plugin Settings",
      "properties": {
        "myPlugin.apiKey": {
          "type": "string",
          "default": "",
          "description": "API密钥"
        }
      }
    },
    "views": {
      "explorer": [
        {
          "id": "myPlugin.panel",
          "name": "My Panel"
        }
      ]
    }
  },
  
  "permissions": [
    "project:read",
    "ai:generate",
    "ui:notification"
  ],
  
  "dependencies": {
    "other-plugin": "^1.0.0"
  }
}
```

### 3.2 插件主入口

```typescript
// src/index.ts
import {
  Plugin,
  PluginContext,
  Command,
  Disposable
} from '@ai-novel-studio/plugin-api';

export default class MyPlugin implements Plugin {
  private context: PluginContext;
  private disposables: Disposable[] = [];
  
  async activate(context: PluginContext): Promise<void> {
    this.context = context;
    
    // 注册命令
    this.registerCommands();
    
    // 注册视图
    this.registerViews();
    
    // 监听事件
    this.registerEventListeners();
    
    // 初始化配置
    await this.initializeSettings();
    
    context.logger.info('My Plugin activated successfully');
  }
  
  async deactivate(): Promise<void> {
    // 清理资源
    this.disposables.forEach(d => d.dispose());
    this.disposables = [];
    
    this.context.logger.info('My Plugin deactivated');
  }
  
  private registerCommands(): void {
    const command: Command = {
      id: 'myPlugin.hello',
      handler: this.sayHello.bind(this)
    };
    
    const disposable = this.context.commands.registerCommand(command);
    this.disposables.push(disposable);
  }
  
  private async sayHello(): Promise<void> {
    const project = this.context.project.getCurrent();
    
    if (!project) {
      this.context.ui.showWarning('请先打开一个项目');
      return;
    }
    
    const result = await this.context.ai.generate({
      prompt: `为项目"${project.name}"生成一句问候语`,
      temperature: 0.8
    });
    
    this.context.ui.showMessage(result.text);
  }
  
  private registerViews(): void {
    const view = this.context.ui.createTreeView('myPlugin.panel', {
      treeDataProvider: new MyTreeDataProvider(this.context)
    });
    
    this.disposables.push(view);
  }
  
  private registerEventListeners(): void {
    const disposable = this.context.events.on('editor.save', async (doc) => {
      await this.onDocumentSave(doc);
    });
    
    this.disposables.push(disposable);
  }
  
  private async onDocumentSave(document: Document): Promise<void> {
    // 保存时的处理逻辑
  }
  
  private async initializeSettings(): Promise<void> {
    const config = this.context.configuration;
    const apiKey = config.get<string>('myPlugin.apiKey');
    
    if (!apiKey) {
      const key = await this.context.ui.showInput({
        prompt: '请输入API密钥',
        password: true
      });
      
      if (key) {
        config.update('myPlugin.apiKey', key);
      }
    }
  }
}
```

### 3.3 插件API接口

```typescript
// @ai-novel-studio/plugin-api

/**
 * 插件接口
 */
export interface Plugin {
  readonly id: string;
  readonly name: string;
  readonly version: string;
  
  activate(context: PluginContext): Promise<void>;
  deactivate(): Promise<void>;
}

/**
 * 插件上下文
 */
export interface PluginContext {
  // 唯一标识
  readonly extensionId: string;
  
  // 订阅管理
  readonly subscriptions: Disposable[];
  
  // API访问
  readonly commands: CommandAPI;
  readonly editor: EditorAPI;
  readonly project: ProjectAPI;
  readonly ai: AIAPI;
  readonly ui: UIAPI;
  readonly storage: StorageAPI;
  readonly filesystem: FileSystemAPI;
  readonly network: NetworkAPI;
  
  // 工具
  readonly logger: Logger;
  readonly events: EventEmitter;
  readonly configuration: Configuration;
  
  // 路径
  readonly extensionPath: string;
  readonly globalStoragePath: string;
  readonly logPath: string;
}

/**
 * 命令API
 */
export interface CommandAPI {
  registerCommand(command: Command): Disposable;
  executeCommand<T>(commandId: string, ...args: any[]): Promise<T>;
  getCommands(): string[];
}

export interface Command {
  id: string;
  handler: (...args: any[]) => any;
  thisArg?: any;
}

/**
 * 编辑器API
 */
export interface EditorAPI {
  // 文档操作
  getActiveDocument(): Document | undefined;
  getDocuments(): Document[];
  openDocument(uri: string): Promise<Document>;
  saveDocument(document: Document): Promise<void>;
  saveAll(): Promise<void>;
  
  // 文本操作
  getText(): string;
  setText(text: string): void;
  getSelection(): Selection;
  setSelection(selection: Selection): void;
  insertText(text: string, position?: Position): void;
  
  // 装饰
  setDecorations(
    decorationType: DecorationType,
    ranges: Range[]
  ): void;
  
  // 事件
  onDidChangeTextDocument(
    listener: (e: TextDocumentChangeEvent) => any
  ): Disposable;
  
  onDidChangeSelection(
    listener: (e: SelectionChangeEvent) => any
  ): Disposable;
}

/**
 * 项目API
 */
export interface ProjectAPI {
  getCurrent(): Project | undefined;
  getAll(): Project[];
  create(options: CreateProjectOptions): Promise<Project>;
  open(path: string): Promise<Project>;
  close(project: Project): Promise<void>;
  
  // 章节操作
  getChapters(project: Project): Chapter[];
  createChapter(project: Project, options: CreateChapterOptions): Promise<Chapter>;
  
  // 角色操作
  getCharacters(project: Project): Character[];
  createCharacter(project: Project, data: CharacterData): Promise<Character>;
  
  // 世界观
  getWorldSettings(project: Project): WorldSetting[];
  
  // 大纲
  getPlotOutline(project: Project): PlotOutline;
}

/**
 * AI API
 */
export interface AIAPI {
  generate(request: GenerateRequest): Promise<GenerateResponse>;
  streamGenerate(
    request: GenerateRequest,
    onChunk: (chunk: StreamChunk) => void
  ): Promise<void>;
  
  chat(request: ChatRequest): Promise<ChatResponse>;
  streamChat(
    request: ChatRequest,
    onChunk: (chunk: ChatChunk) => void
  ): Promise<void>;
  
  embed(text: string | string[]): Promise<number[] | number[][]>;
  
  // 模型管理
  getModels(): ModelInfo[];
  getCurrentModel(): string;
  setModel(modelId: string): void;
}

/**
 * UI API
 */
export interface UIAPI {
  // 消息
  showMessage(message: string, ...items: string[]): Promise<string | undefined>;
  showInformation(message: string): void;
  showWarning(message: string): void;
  showError(message: string): void;
  
  // 输入
  showInput(options: InputBoxOptions): Promise<string | undefined>;
  showQuickPick<T extends QuickPickItem>(
    items: T[],
    options?: QuickPickOptions
  ): Promise<T | undefined>;
  
  // 对话框
  showOpenDialog(options: OpenDialogOptions): Promise<string[] | undefined>;
  showSaveDialog(options: SaveDialogOptions): Promise<string | undefined>;
  
  // 视图
  createTreeView<T>(
    viewId: string,
    options: TreeViewOptions<T>
  ): TreeView<T>;
  
  // 面板
  createWebviewPanel(
    viewType: string,
    title: string,
    options: WebviewPanelOptions
  ): WebviewPanel;
  
  // 状态栏
  createStatusBarItem(priority?: number): StatusBarItem;
  
  // 通知
  showNotification(options: NotificationOptions): void;
}

/**
 * 存储API
 */
export interface StorageAPI {
  // 键值存储
  get<T>(key: string): Promise<T | undefined>;
  set<T>(key: string, value: T): Promise<void>;
  delete(key: string): Promise<void>;
  clear(): Promise<void>;
  
  // 键值存储（同步）
  getSync<T>(key: string): T | undefined;
  setSync<T>(key: string, value: T): void;
  
  // 数据库
  getDatabase(): Promise<Database>;
}

/**
 * 文件系统API
 */
export interface FileSystemAPI {
  readFile(path: string): Promise<string>;
  readBinaryFile(path: string): Promise<Uint8Array>;
  writeFile(path: string, content: string): Promise<void>;
  writeBinaryFile(path: string, content: Uint8Array): Promise<void>;
  
  exists(path: string): Promise<boolean>;
  stat(path: string): Promise<FileStat>;
  mkdir(path: string): Promise<void>;
  readdir(path: string): Promise<string[]>;
  unlink(path: string): Promise<void>;
  
  // 监听
  watch(
    path: string,
    options?: WatchOptions
  ): Disposable;
  
  // 对话框
  showOpenDialog(options: OpenDialogOptions): Promise<string[] | undefined>;
  showSaveDialog(options: SaveDialogOptions): Promise<string | undefined>;
}

/**
 * 网络API
 */
export interface NetworkAPI {
  fetch(url: string, options?: RequestInit): Promise<Response>;
  get<T>(url: string, options?: RequestOptions): Promise<T>;
  post<T>(url: string, body: any, options?: RequestOptions): Promise<T>;
  
  // WebSocket
  createWebSocket(url: string): WebSocket;
}
```

---

## 四、插件生命周期

### 4.1 生命周期流程

```
┌─────────────┐
│   发现插件   │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│   加载清单   │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│   解析依赖   │
└──────┬──────┘
       │
       ↓
┌─────────────┐     ┌──────────┐
│   检查权限   │────→│ 用户授权  │
└──────┬──────┘     └──────────┘
       │
       ↓
┌─────────────┐
│   加载代码   │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│   初始化插件  │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│   激活插件   │ (activate)
└──────┬──────┘
       │
       │ ←──── 运行中 ────→
       │
       ↓
┌─────────────┐
│   停用插件   │ (deactivate)
└──────┬──────┘
       │
       ↓
┌─────────────┐
│   卸载插件   │
└─────────────┘
```

### 4.2 激活事件

```typescript
enum ActivationEvent {
  // 应用启动时激活
  ON_STARTUP = 'onStartup',
  
  // 打开项目时激活
  ON_PROJECT_OPEN = 'onProjectOpen',
  
  // 命令被调用时激活
  ON_COMMAND = 'onCommand',
  
  // 特定语言时激活
  ON_LANGUAGE = 'onLanguage',
  
  // 文件系统变化时激活
  ON_FILE_CHANGE = 'onFileChange',
  
  // 视图打开时激活
  ON_VIEW = 'onView',
  
  // 包含特定文件时激活
  ON_FILE = 'onFile',
  
  // 工作区包含特定文件时激活
  ON_WORKSPACE_CONTAINS = 'onWorkspaceContains'
}

// plugin.json 配置示例
{
  "activationEvents": [
    "onStartup",
    "onCommand:myPlugin.hello",
    "onLanguage:novel",
    "onProjectOpen"
  ]
}
```

### 4.3 生命周期管理

```typescript
// 插件生命周期管理器
class PluginLifecycleManager {
  private plugins: Map<string, PluginInstance> = new Map();
  
  async loadPlugin(pluginPath: string): Promise<void> {
    // 1. 读取清单
    const manifest = await this.readManifest(pluginPath);
    
    // 2. 验证清单
    this.validateManifest(manifest);
    
    // 3. 检查依赖
    await this.checkDependencies(manifest);
    
    // 4. 检查权限
    const granted = await this.checkPermissions(manifest);
    if (!granted) {
      throw new Error('权限未授予');
    }
    
    // 5. 加载插件代码
    const pluginModule = await this.loadPluginCode(pluginPath);
    
    // 6. 创建插件实例
    const instance = new PluginInstance(manifest, pluginModule);
    
    this.plugins.set(manifest.id, instance);
  }
  
  async activatePlugin(pluginId: string): Promise<void> {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      throw new Error(`插件未找到: ${pluginId}`);
    }
    
    if (instance.state !== PluginState.INACTIVE) {
      return;
    }
    
    // 创建插件上下文
    const context = this.createPluginContext(instance);
    
    // 激活插件
    instance.state = PluginState.ACTIVATING;
    
    try {
      await instance.plugin.activate(context);
      instance.state = PluginState.ACTIVE;
      instance.context = context;
    } catch (error) {
      instance.state = PluginState.ERROR;
      throw error;
    }
  }
  
  async deactivatePlugin(pluginId: string): Promise<void> {
    const instance = this.plugins.get(pluginId);
    if (!instance) {
      return;
    }
    
    if (instance.state !== PluginState.ACTIVE) {
      return;
    }
    
    instance.state = PluginState.DEACTIVATING;
    
    try {
      // 清理订阅
      instance.context.subscriptions.forEach(d => d.dispose());
      
      // 调用deactivate
      await instance.plugin.deactivate();
      
      instance.state = PluginState.INACTIVE;
    } catch (error) {
      instance.state = PluginState.ERROR;
      throw error;
    }
  }
  
  async unloadPlugin(pluginId: string): Promise<void> {
    await this.deactivatePlugin(pluginId);
    this.plugins.delete(pluginId);
  }
}

enum PluginState {
  INACTIVE = 'inactive',
  ACTIVATING = 'activating',
  ACTIVE = 'active',
  DEACTIVATING = 'deactivating',
  ERROR = 'error'
}

class PluginInstance {
  manifest: PluginManifest;
  plugin: Plugin;
  state: PluginState = PluginState.INACTIVE;
  context?: PluginContext;
  
  constructor(manifest: PluginManifest, plugin: Plugin) {
    this.manifest = manifest;
    this.plugin = plugin;
  }
}
```

---

## 五、插件安全机制

### 5.1 权限系统

```typescript
// 权限定义
interface Permission {
  id: string;
  name: string;
  description: string;
  risk: 'low' | 'medium' | 'high';
}

const PERMISSIONS: Permission[] = [
  {
    id: 'project:read',
    name: '读取项目数据',
    description: '允许插件读取当前打开的项目数据',
    risk: 'low'
  },
  {
    id: 'project:write',
    name: '修改项目数据',
    description: '允许插件修改、删除项目数据',
    risk: 'high'
  },
  {
    id: 'ai:generate',
    name: '使用AI生成',
    description: '允许插件调用AI生成功能',
    risk: 'medium'
  },
  {
    id: 'filesystem:read',
    name: '读取文件',
    description: '允许插件读取文件系统',
    risk: 'medium'
  },
  {
    id: 'filesystem:write',
    name: '写入文件',
    description: '允许插件写入、删除文件',
    risk: 'high'
  },
  {
    id: 'network:http',
    name: '网络访问',
    description: '允许插件发起网络请求',
    risk: 'medium'
  }
];

// 权限管理器
class PermissionManager {
  private grantedPermissions: Map<string, Set<string>> = new Map();
  
  async requestPermission(
    pluginId: string,
    permission: string
  ): Promise<boolean> {
    // 检查是否已授权
    if (this.hasPermission(pluginId, permission)) {
      return true;
    }
    
    // 获取权限信息
    const permInfo = PERMISSIONS.find(p => p.id === permission);
    if (!permInfo) {
      throw new Error(`未知权限: ${permission}`);
    }
    
    // 向用户请求授权
    const granted = await this.showPermissionDialog(pluginId, permInfo);
    
    if (granted) {
      this.grantPermission(pluginId, permission);
    }
    
    return granted;
  }
  
  private async showPermissionDialog(
    pluginId: string,
    permission: Permission
  ): Promise<boolean> {
    const message = `
插件 "${pluginId}" 请求以下权限：

${permission.name}
${permission.description}

风险等级: ${this.getRiskLabel(permission.risk)}

是否授权？
    `;
    
    return await UI.showConfirm(message);
  }
  
  hasPermission(pluginId: string, permission: string): boolean {
    const permissions = this.grantedPermissions.get(pluginId);
    return permissions?.has(permission) ?? false;
  }
  
  grantPermission(pluginId: string, permission: string): void {
    if (!this.grantedPermissions.has(pluginId)) {
      this.grantedPermissions.set(pluginId, new Set());
    }
    this.grantedPermissions.get(pluginId)!.add(permission);
  }
  
  revokePermission(pluginId: string, permission: string): void {
    this.grantedPermissions.get(pluginId)?.delete(permission);
  }
  
  revokeAllPermissions(pluginId: string): void {
    this.grantedPermissions.delete(pluginId);
  }
}
```

### 5.2 沙箱隔离

```typescript
// 插件沙箱
class PluginSandbox {
  private vm: VM;
  private permissions: Set<string>;
  private resourceLimits: ResourceLimits;
  
  constructor(
    pluginId: string,
    permissions: string[],
    limits: ResourceLimits
  ) {
    this.permissions = new Set(permissions);
    this.resourceLimits = limits;
    
    this.vm = this.createSandboxedVM(pluginId);
  }
  
  private createSandboxedVM(pluginId: string): VM {
    const sandbox = {
      // 提供受限的API
      console: this.createSafeConsole(pluginId),
      setTimeout: this.createSafeTimeout(),
      setInterval: this.createSafeInterval(),
      fetch: this.createSafeFetch(),
      require: this.createSafeRequire()
    };
    
    return new VM({
      sandbox,
      timeout: this.resourceLimits.timeout
    });
  }
  
  private createSafeConsole(pluginId: string) {
    return {
      log: (...args: any[]) => {
        logger.info(`[${pluginId}]`, ...args);
      },
      warn: (...args: any[]) => {
        logger.warn(`[${pluginId}]`, ...args);
      },
      error: (...args: any[]) => {
        logger.error(`[${pluginId}]`, ...args);
      }
    };
  }
  
  private createSafeTimeout() {
    return (callback: Function, delay: number) => {
      if (delay > this.resourceLimits.maxTimeout) {
        throw new Error(`超时时间超出限制: ${delay}ms`);
      }
      return setTimeout(callback, delay);
    };
  }
  
  private createSafeFetch() {
    return async (url: string, options?: RequestInit) => {
      if (!this.permissions.has('network:http')) {
        throw new Error('插件没有网络访问权限');
      }
      
      // 检查URL白名单
      if (!this.isUrlAllowed(url)) {
        throw new Error(`URL不在允许列表中: ${url}`);
      }
      
      return fetch(url, options);
    };
  }
  
  private createSafeRequire() {
    return (module: string) => {
      // 只允许加载白名单内的模块
      const allowedModules = [
        'path',
        'util',
        'crypto'
      ];
      
      if (!allowedModules.includes(module)) {
        throw new Error(`模块不在允许列表中: ${module}`);
      }
      
      return require(module);
    };
  }
  
  execute(code: string): any {
    return this.vm.run(code);
  }
}

interface ResourceLimits {
  timeout: number;           // 执行超时
  maxMemory: number;         // 最大内存
  maxCpuTime: number;        // 最大CPU时间
  maxTimeout: number;        // 最大定时器延迟
  maxNetworkRequests: number; // 最大网络请求数
}
```

### 5.3 资源限制

```typescript
// 资源监控器
class ResourceMonitor {
  private pluginResources: Map<string, PluginResourceUsage> = new Map();
  
  startMonitoring(pluginId: string): void {
    this.pluginResources.set(pluginId, {
      memoryUsage: 0,
      cpuTime: 0,
      networkRequests: 0,
      fileOperations: 0
    });
  }
  
  recordMemoryUsage(pluginId: string, bytes: number): void {
    const usage = this.pluginResources.get(pluginId);
    if (usage) {
      usage.memoryUsage = bytes;
      this.checkLimits(pluginId);
    }
  }
  
  recordCpuTime(pluginId: string, ms: number): void {
    const usage = this.pluginResources.get(pluginId);
    if (usage) {
      usage.cpuTime += ms;
      this.checkLimits(pluginId);
    }
  }
  
  private checkLimits(pluginId: string): void {
    const usage = this.pluginResources.get(pluginId);
    if (!usage) return;
    
    const limits = this.getPluginLimits(pluginId);
    
    if (usage.memoryUsage > limits.maxMemory) {
      throw new Error(`插件 ${pluginId} 内存使用超出限制`);
    }
    
    if (usage.cpuTime > limits.maxCpuTime) {
      throw new Error(`插件 ${pluginId} CPU时间超出限制`);
    }
  }
  
  stopMonitoring(pluginId: string): PluginResourceUsage {
    const usage = this.pluginResources.get(pluginId);
    this.pluginResources.delete(pluginId);
    return usage!;
  }
}

interface PluginResourceUsage {
  memoryUsage: number;
  cpuTime: number;
  networkRequests: number;
  fileOperations: number;
}
```

---

## 六、插件市场

### 6.1 市场架构

```
┌─────────────────────────────────────────────────────────────────┐
│                    Plugin Marketplace                            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  Plugin Registry Service                  │  │
│  │  - 插件索引                                                │  │
│  │  - 版本管理                                                │  │
│  │  - 搜索服务                                                │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  Plugin CDN                               │  │
│  │  - 插件包存储                                              │  │
│  │  - 全球分发                                                │  │
│  │  - 缓存加速                                                │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                  Review System                            │  │
│  │  - 用户评分                                                │  │
│  │  - 评论系统                                                │  │
│  │  - 安全审核                                                │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 插件搜索与发现

```typescript
// 插件市场客户端
class PluginMarketplaceClient {
  private apiClient: APIClient;
  
  async search(query: SearchQuery): Promise<SearchResult[]> {
    return this.apiClient.get('/plugins/search', {
      params: query
    });
  }
  
  async getPlugin(pluginId: string): Promise<PluginInfo> {
    return this.apiClient.get(`/plugins/${pluginId}`);
  }
  
  async getPluginVersions(
    pluginId: string
  ): Promise<PluginVersion[]> {
    return this.apiClient.get(`/plugins/${pluginId}/versions`);
  }
  
  async downloadPlugin(
    pluginId: string,
    version: string
  ): Promise<string> {
    const url = await this.getDownloadUrl(pluginId, version);
    const downloadPath = await this.downloadFile(url);
    
    // 验证签名
    await this.verifySignature(downloadPath);
    
    return downloadPath;
  }
  
  async installPlugin(
    pluginId: string,
    version?: string
  ): Promise<void> {
    // 获取最新版本
    if (!version) {
      const pluginInfo = await this.getPlugin(pluginId);
      version = pluginInfo.latestVersion;
    }
    
    // 下载插件
    const downloadPath = await this.downloadPlugin(pluginId, version);
    
    // 安装插件
    await this.pluginManager.install(downloadPath);
    
    // 清理下载文件
    await fs.unlink(downloadPath);
  }
  
  async uninstallPlugin(pluginId: string): Promise<void> {
    await this.pluginManager.uninstall(pluginId);
  }
  
  async updatePlugin(pluginId: string): Promise<void> {
    const installed = await this.pluginManager.getPlugin(pluginId);
    const latest = await this.getPlugin(pluginId);
    
    if (installed.version === latest.latestVersion) {
      return; // 已是最新版本
    }
    
    await this.installPlugin(pluginId, latest.latestVersion);
  }
}

interface SearchQuery {
  keyword?: string;
  category?: string;
  sortBy?: 'downloads' | 'rating' | 'updated';
  page?: number;
  pageSize?: number;
}

interface SearchResult {
  id: string;
  name: string;
  description: string;
  icon: string;
  author: string;
  version: string;
  downloads: number;
  rating: number;
  category: string;
}

interface PluginInfo {
  id: string;
  name: string;
  description: string;
  longDescription?: string;
  icon: string;
  screenshots?: string[];
  author: AuthorInfo;
  version: string;
  latestVersion: string;
  downloads: number;
  rating: number;
  reviews: number;
  category: string;
  tags: string[];
  readme: string;
  changelog: string;
  license: string;
  repository?: string;
  homepage?: string;
  publishedAt: string;
  updatedAt: string;
}
```

### 6.3 插件审核机制

```typescript
// 插件审核流程
class PluginReviewSystem {
  async submitPlugin(submission: PluginSubmission): Promise<void> {
    // 1. 自动安全检查
    const securityResult = await this.securityScan(submission.packagePath);
    if (!securityResult.passed) {
      throw new Error(`安全检查未通过: ${securityResult.issues.join(', ')}`);
    }
    
    // 2. 代码审查
    const codeReviewResult = await this.codeReview(submission.packagePath);
    
    // 3. 功能测试
    const testResult = await this.runTests(submission.packagePath);
    
    // 4. 人工审核
    await this.submitForManualReview({
      submission,
      securityResult,
      codeReviewResult,
      testResult
    });
  }
  
  private async securityScan(packagePath: string): Promise<SecurityScanResult> {
    const issues: string[] = [];
    
    // 检查恶意代码
    const malwareCheck = await this.checkMalware(packagePath);
    if (!malwareCheck.safe) {
      issues.push(...malwareCheck.threats);
    }
    
    // 检查敏感权限
    const manifest = await this.readManifest(packagePath);
    const sensitivePermissions = manifest.permissions?.filter(
      p => this.isSensitivePermission(p)
    );
    if (sensitivePermissions?.length > 0) {
      issues.push(`请求敏感权限: ${sensitivePermissions.join(', ')}`);
    }
    
    // 检查外部请求
    const networkCheck = await this.checkNetworkCalls(packagePath);
    if (networkCheck.suspicious) {
      issues.push(...networkCheck.issues);
    }
    
    return {
      passed: issues.length === 0,
      issues
    };
  }
  
  private async codeReview(packagePath: string): Promise<CodeReviewResult> {
    // 静态代码分析
    const analysis = await this.staticAnalysis(packagePath);
    
    // 代码质量检查
    const quality = await this.codeQualityCheck(packagePath);
    
    return {
      score: quality.score,
      issues: analysis.issues,
      suggestions: analysis.suggestions
    };
  }
}
```

---

## 七、脚本系统

### 7.1 脚本语言支持

```typescript
// 脚本引擎管理器
class ScriptEngineManager {
  private engines: Map<string, ScriptEngine> = new Map();
  
  constructor() {
    this.engines.set('python', new PythonEngine());
    this.engines.set('javascript', new JavaScriptEngine());
    this.engines.set('lua', new LuaEngine());
  }
  
  getEngine(language: string): ScriptEngine {
    const engine = this.engines.get(language);
    if (!engine) {
      throw new Error(`不支持的脚本语言: ${language}`);
    }
    return engine;
  }
  
  async executeScript(
    language: string,
    code: string,
    context: ScriptContext
  ): Promise<ScriptResult> {
    const engine = this.getEngine(language);
    return engine.execute(code, context);
  }
}

// 脚本引擎接口
interface ScriptEngine {
  language: string;
  
  execute(code: string, context: ScriptContext): Promise<ScriptResult>;
  
  validate(code: string): Promise<ValidationResult>;
  
  getCompletions(
    code: string,
    position: number
  ): Promise<CompletionItem[]>;
}

// Python脚本引擎
class PythonEngine implements ScriptEngine {
  language = 'python';
  private runtime: PythonRuntime;
  
  constructor() {
    this.runtime = new PythonRuntime({
      pythonPath: this.findPython(),
      timeout: 30000
    });
  }
  
  async execute(
    code: string,
    context: ScriptContext
  ): Promise<ScriptResult> {
    // 注入上下文
    const wrappedCode = this.wrapCode(code, context);
    
    // 执行代码
    const result = await this.runtime.execute(wrappedCode);
    
    return {
      output: result.stdout,
      returnValue: result.returnValue,
      error: result.stderr
    };
  }
  
  private wrapCode(code: string, context: ScriptContext): string {
    return `
import json
from ai_novel_studio import Editor, Project, AI, UI

# 注入上下文
_editor = json.loads('${JSON.stringify(context.editor)}')
_project = json.loads('${JSON.stringify(context.project)}')

# 用户代码
${code}
    `;
  }
}
```

### 7.2 脚本API

```python
# Python脚本API示例
from ai_novel_studio import (
    Editor,
    Project,
    AI,
    UI,
    Storage
)

# 获取当前文档
document = Editor.get_active_document()

# 获取选中文本
selection = Editor.get_selection()
selected_text = selection.text

# AI续写
if selected_text:
    continuation = AI.generate(
        prompt=f"续写以下内容：\n{selected_text}",
        temperature=0.8,
        max_tokens=500
    )
    
    # 插入到文档
    Editor.insert_text(continuation.text)
    
    # 显示通知
    UI.show_message("AI续写完成！")

# 批量处理章节
project = Project.get_current()
chapters = Project.get_chapters(project)

for chapter in chapters:
    # 生成章节摘要
    summary = AI.generate(
        prompt=f"用一句话总结以下章节：\n{chapter.content[:1000]}",
        max_tokens=100
    )
    
    # 保存摘要
    Storage.set(f"summary_{chapter.id}", summary.text)

UI.show_message(f"已处理 {len(chapters)} 个章节")
```

### 7.3 可视化脚本编辑器

```typescript
// 可视化脚本节点
interface ScriptNode {
  id: string;
  type: NodeType;
  position: { x: number; y: number };
  data: NodeData;
  inputs: Port[];
  outputs: Port[];
}

enum NodeType {
  START = 'start',
  END = 'end',
  CONDITION = 'condition',
  ACTION = 'action',
  AI_GENERATE = 'ai_generate',
  TEXT_PROCESS = 'text_process',
  FILE_OPERATION = 'file_operation',
  LOOP = 'loop'
}

// 可视化脚本编辑器
class VisualScriptEditor {
  private nodes: Map<string, ScriptNode> = new Map();
  private edges: Edge[] = [];
  
  addNode(type: NodeType, position: { x: number; y: number }): ScriptNode {
    const node: ScriptNode = {
      id: generateId(),
      type,
      position,
      data: this.getDefaultNodeData(type),
      inputs: this.getDefaultInputs(type),
      outputs: this.getDefaultOutputs(type)
    };
    
    this.nodes.set(node.id, node);
    return node;
  }
  
  connect(
    sourceNodeId: string,
    sourcePort: string,
    targetNodeId: string,
    targetPort: string
  ): void {
    this.edges.push({
      id: generateId(),
      source: sourceNodeId,
      sourcePort,
      target: targetNodeId,
      targetPort
    });
  }
  
  generateCode(language: string): string {
    const generator = this.getCodeGenerator(language);
    return generator.generate(this.nodes, this.edges);
  }
}
```

---

## 八、插件开发指南

### 8.1 开发环境搭建

```bash
# 创建插件项目
mkdir my-plugin
cd my-plugin

# 初始化项目
npm init -y

# 安装依赖
npm install --save-dev typescript @ai-novel-studio/plugin-api

# 创建目录结构
mkdir -p src assets
```

### 8.2 项目结构

```
my-plugin/
├── src/
│   ├── index.ts           # 主入口
│   ├── commands.ts        # 命令定义
│   ├── views/             # 视图组件
│   ├── services/          # 服务层
│   └── utils/             # 工具函数
├── assets/
│   ├── icon.png           # 插件图标
│   └── images/            # 图片资源
├── tests/
│   └── index.test.ts      # 测试文件
├── plugin.json            # 插件清单
├── tsconfig.json          # TypeScript配置
├── package.json           # NPM配置
└── README.md              # 说明文档
```

### 8.3 开发示例

```typescript
// src/index.ts
import { Plugin, PluginContext } from '@ai-novel-studio/plugin-api';
import { registerCommands } from './commands';
import { registerViews } from './views';
import { MyService } from './services/my-service';

export default class MyPlugin implements Plugin {
  private service: MyService;
  
  async activate(context: PluginContext): Promise<void> {
    // 初始化服务
    this.service = new MyService(context);
    
    // 注册命令
    registerCommands(context);
    
    // 注册视图
    registerViews(context);
    
    // 监听事件
    this.setupEventListeners(context);
    
    context.logger.info('My Plugin 已激活');
  }
  
  async deactivate(): Promise<void> {
    await this.service.cleanup();
  }
  
  private setupEventListeners(context: PluginContext): void {
    context.events.on('editor.save', async (doc) => {
      await this.service.onDocumentSave(doc);
    });
  }
}

// src/commands.ts
import { PluginContext } from '@ai-novel-studio/plugin-api';

export function registerCommands(context: PluginContext): void {
  context.commands.registerCommand({
    id: 'myPlugin.analyzeText',
    handler: async () => {
      const document = context.editor.getActiveDocument();
      if (!document) {
        context.ui.showWarning('请先打开一个文档');
        return;
      }
      
      const text = document.getText();
      const analysis = await context.ai.generate({
        prompt: `分析以下文本的写作风格、情感和主题：\n${text.slice(0, 1000)}`
      });
      
      context.ui.showMessage(analysis.text);
    }
  });
}

// src/services/my-service.ts
import { PluginContext, Document } from '@ai-novel-studio/plugin-api';

export class MyService {
  constructor(private context: PluginContext) {}
  
  async onDocumentSave(document: Document): Promise<void> {
    // 保存后的处理逻辑
  }
  
  async cleanup(): Promise<void> {
    // 清理资源
  }
}
```

### 8.4 打包发布

```bash
# 构建插件
npm run build

# 打包插件
npm run package

# 发布到市场
npm run publish
```

```json
// package.json scripts
{
  "scripts": {
    "build": "tsc",
    "package": "novel-studio package",
    "publish": "novel-studio publish"
  }
}
```

---

## 九、官方插件列表

### 9.1 核心插件

| 插件名称 | 功能描述 | 类型 |
|---------|---------|------|
| EPUB导出器 | 导出EPUB格式电子书 | import_export |
| PDF导出器 | 导出PDF格式文档 | import_export |
| 语法检查 | 多语言语法和拼写检查 | editor_extension |
| 翻译助手 | AI辅助翻译 | feature_module |
| 语音朗读 | 文本转语音朗读 | utility |
| 思维导图 | 可视化思维导图 | feature_module |
| 灵感库 | 随机生成创意灵感 | utility |
| 名字生成器 | 生成角色名字 | utility |
| 情节生成器 | 随机生成情节片段 | utility |
| 角色对话 | 与角色进行AI对话 | feature_module |

### 9.2 社区插件

| 插件名称 | 功能描述 | 作者 |
|---------|---------|------|
| Markdown增强 | 增强的Markdown支持 | community |
| 代码高亮 | 代码块语法高亮 | community |
| Git集成 | Git版本控制集成 | community |
| 云同步 | 第三方云存储同步 | community |
| 统计面板 | 详细的写作统计 | community |

---

## 十、最佳实践

### 10.1 性能优化

```typescript
// ✅ 好的实践：延迟加载
export default class MyPlugin implements Plugin {
  private heavyModule: any;
  
  async activate(context: PluginContext): Promise<void> {
    // 只注册轻量级功能
    context.commands.registerCommand({
      id: 'myPlugin.heavyAction',
      handler: async () => {
        // 需要时才加载重量级模块
        if (!this.heavyModule) {
          this.heavyModule = await import('./heavy-module');
        }
        return this.heavyModule.doSomething();
      }
    });
  }
}

// ❌ 不好的实践：激活时加载所有内容
export default class MyPlugin implements Plugin {
  async activate(context: PluginContext): Promise<void> {
    // 激活时加载重量级模块，影响启动速度
    const heavyModule = await import('./heavy-module');
  }
}
```

### 10.2 错误处理

```typescript
// ✅ 好的实践：完善的错误处理
async function myCommand(context: PluginContext): Promise<void> {
  try {
    const document = context.editor.getActiveDocument();
    
    if (!document) {
      context.ui.showWarning('请先打开一个文档');
      return;
    }
    
    const result = await context.ai.generate({
      prompt: '...'
    });
    
    context.ui.showMessage('操作成功');
    
  } catch (error) {
    context.logger.error('命令执行失败', error);
    context.ui.showError(`操作失败: ${error.message}`);
  }
}
```

### 10.3 资源清理

```typescript
// ✅ 好的实践：正确清理资源
export default class MyPlugin implements Plugin {
  private disposables: Disposable[] = [];
  private timers: NodeJS.Timeout[] = [];
  
  async activate(context: PluginContext): Promise<void> {
    // 使用订阅管理
    const disposable = context.events.on('editor.save', this.onSave);
    this.disposables.push(disposable);
    
    // 记录定时器
    const timer = setInterval(this.periodicTask, 60000);
    this.timers.push(timer);
  }
  
  async deactivate(): Promise<void> {
    // 清理所有订阅
    this.disposables.forEach(d => d.dispose());
    
    // 清理所有定时器
    this.timers.forEach(t => clearInterval(t));
    
    // 清空数组
    this.disposables = [];
    this.timers = [];
  }
}
```

---

**文档版本**: v1.0  
**最后更新**: 2026-02-19  
**维护者**: AI小说创作工作室开发团队
