import React, { useState } from "react";
import {
  X,
  FileCode,
  Zap,
  Shield,
  CheckCircle2,
  Copy,
  Download,
  Code,
  Database,
  Cpu,
  Globe,
} from "lucide-react";

interface PluginDevGuideDialogProps {
  onClose: () => void;
}

interface ApiCategoryProps {
  title: string;
  apis: string[];
}

function ApiCategory({ title, apis }: ApiCategoryProps) {
  return (
    <div className="bg-slate-50 dark:bg-slate-800/50 rounded-lg p-3">
      <h4 className="font-medium text-sm text-slate-700 dark:text-slate-300 mb-2">{title}</h4>
      <ul className="space-y-1">
        {apis.map((api, index) => (
          <li key={index} className="text-sm text-slate-600 dark:text-slate-400 font-mono">
            • {api}
          </li>
        ))}
      </ul>
    </div>
  );
}

export function PluginDevGuideDialog({ onClose }: PluginDevGuideDialogProps) {
  const [activeTab, setActiveTab] = useState<"guide" | "api">("guide");

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-900 w-[800px] max-h-[90vh] rounded-lg shadow-lg flex flex-col">
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700 shrink-0">
          <h2 className="text-lg font-semibold text-slate-900 dark:text-white flex items-center gap-2">
            <FileCode className="w-5 h-5" />
            插件开发指南
          </h2>
          <button onClick={onClose} className="p-1 hover:bg-accent rounded transition-colors">
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="flex-1 overflow-y-auto">
          <div className="flex border-b border-slate-200 dark:border-slate-700">
            <button
              onClick={() => setActiveTab("guide")}
              className={`px-6 py-3 text-sm font-medium transition-colors border-b-2 -mb-px ${
                activeTab === "guide"
                  ? "text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400"
                  : "text-slate-600 dark:text-slate-400 border-transparent hover:text-slate-900 dark:hover:text-white"
              }`}
            >
              <FileCode className="w-4 h-4 inline mr-2" />
              开发指南
            </button>
            <button
              onClick={() => setActiveTab("api")}
              className={`px-6 py-3 text-sm font-medium transition-colors border-b-2 -mb-px ${
                activeTab === "api"
                  ? "text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400"
                  : "text-slate-600 dark:text-slate-400 border-transparent hover:text-slate-900 dark:hover:text-white"
              }`}
            >
              <Code className="w-4 h-4 inline mr-2" />
              系统 API
            </button>
          </div>

          {activeTab === "guide" && (
            <div className="p-6 space-y-6">
              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <Zap className="w-5 h-5 text-blue-500" />
                  快速开始
                </h3>
                <p className="text-slate-600 dark:text-slate-400 mb-4">
                  插件是一个包含{" "}
                  <code className="bg-slate-100 dark:bg-slate-800 px-2 py-1 rounded">
                    plugin.json
                  </code>{" "}
                  配置文件的目录。 配置文件定义了插件的基本信息、权限和功能。
                </p>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <FileCode className="w-5 h-5 text-green-500" />
                  插件结构
                </h3>
                <pre className="bg-slate-100 dark:bg-slate-800 p-4 rounded-lg overflow-auto text-sm mb-4">
                  <code>
                    {`my-plugin/
├── plugin.json          # 插件配置文件
├── index.js             # 插件入口文件
├── assets/              # 资源文件（图标、图片等）
└── lib/                 # 依赖库`}
                  </code>
                </pre>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <Shield className="w-5 h-5 text-purple-500" />
                  plugin.json 配置
                </h3>
                <div className="relative">
                  <button
                    onClick={() => copyToClipboard(examplePluginJson)}
                    className="absolute top-2 right-2 p-2 bg-slate-200 dark:bg-slate-700 rounded hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors"
                    title="复制"
                  >
                    <Copy className="w-4 h-4" />
                  </button>
                  <pre className="bg-slate-100 dark:bg-slate-800 p-4 rounded-lg overflow-auto text-sm text-slate-900 dark:text-slate-100">
                    <code>{examplePluginJson}</code>
                  </pre>
                </div>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <CheckCircle2 className="w-5 h-5 text-yellow-500" />
                  插件类型
                </h3>
                <div className="grid grid-cols-2 gap-3">
                  <div className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg">
                    <h4 className="font-medium mb-1 text-slate-900 dark:text-white">
                      editor_extension
                    </h4>
                    <p className="text-sm text-slate-600 dark:text-slate-400">
                      编辑器扩展，如语法高亮、代码补全
                    </p>
                  </div>
                  <div className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg">
                    <h4 className="font-medium mb-1 text-slate-900 dark:text-white">
                      feature_module
                    </h4>
                    <p className="text-sm text-slate-600 dark:text-slate-400">
                      功能模块，添加新功能
                    </p>
                  </div>
                  <div className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg">
                    <h4 className="font-medium mb-1 text-slate-900 dark:text-white">theme</h4>
                    <p className="text-sm text-slate-600 dark:text-slate-400">主题，改变应用外观</p>
                  </div>
                  <div className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg">
                    <h4 className="font-medium mb-1 text-slate-900 dark:text-white">utility</h4>
                    <p className="text-sm text-slate-600 dark:text-slate-400">工具类插件</p>
                  </div>
                </div>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-white">
                  安装插件
                </h3>
                <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
                  <p className="text-sm text-slate-700 dark:text-slate-300 mb-2">
                    1. 将插件目录放入应用的插件目录中
                  </p>
                  <p className="text-sm text-slate-700 dark:text-slate-300 mb-2">
                    2. 点击"安装插件"按钮，输入插件路径
                  </p>
                  <p className="text-sm text-slate-700 dark:text-slate-300">3. 激活插件即可使用</p>
                </div>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-white">
                  示例插件
                </h3>
                <div className="flex gap-2">
                  <button
                    onClick={() => downloadExamplePlugin("wordcount")}
                    className="flex-1 px-4 py-3 bg-primary text-primary-foreground rounded-lg flex items-center justify-center gap-2 hover:bg-primary/90"
                  >
                    <Download className="w-4 h-4" />
                    字数统计插件
                  </button>
                  <button
                    onClick={() => downloadExamplePlugin("inspiration")}
                    className="flex-1 px-4 py-3 bg-secondary text-secondary-foreground rounded-lg flex items-center justify-center gap-2 hover:bg-secondary/80"
                  >
                    <Download className="w-4 h-4" />
                    灵感生成器插件
                  </button>
                </div>
              </section>
            </div>
          )}

          {activeTab === "api" && (
            <div className="p-6 space-y-6">
              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <Cpu className="w-5 h-5 text-blue-500" />
                  调用系统 API
                </h3>
                <p className="text-slate-600 dark:text-slate-400 mb-4">
                  插件可以通过 Tauri 的{" "}
                  <code className="bg-slate-100 dark:bg-slate-800 px-2 py-1 rounded">invoke</code>{" "}
                  API 调用后端命令。
                </p>
                <pre className="bg-slate-100 dark:bg-slate-800 p-4 rounded-lg overflow-auto text-sm mb-4">
                  <code>
                    {`// 在插件中调用系统 API
import { invoke } from '@tauri-apps/api/core';

// 获取所有项目
const projects = await invoke('get_projects');

// 保存章节
await invoke('save_chapter', { 
  project_id: 'xxx', 
  title: '章节标题',
  content: '章节内容',
  sort_order: 1
});

// AI 续写
const result = await invoke('ai_continue_novel', {
  content: '前文内容...',
  model: 'default'
});`}
                  </code>
                </pre>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <Database className="w-5 h-5 text-green-500" />
                  数据操作 API
                </h3>
                <div className="space-y-3">
                  <ApiCategory
                    title="项目管理"
                    apis={[
                      "get_projects - 获取所有项目",
                      "create_project - 创建项目",
                      "update_project - 更新项目",
                      "delete_project - 删除项目",
                    ]}
                  />
                  <ApiCategory
                    title="章节管理"
                    apis={[
                      "get_chapters - 获取章节列表",
                      "save_chapter - 保存章节",
                      "update_chapter - 更新章节",
                      "delete_chapter - 删除章节",
                    ]}
                  />
                  <ApiCategory
                    title="角色管理"
                    apis={[
                      "get_characters - 获取角色列表",
                      "create_character - 创建角色",
                      "update_character - 更新角色",
                      "delete_character - 删除角色",
                    ]}
                  />
                  <ApiCategory
                    title="情节点管理"
                    apis={[
                      "get_plot_points - 获取情节点",
                      "create_plot_point - 创建情节点",
                      "update_plot_point - 更新情节点",
                      "delete_plot_point - 删除情节点",
                    ]}
                  />
                  <ApiCategory
                    title="世界观管理"
                    apis={[
                      "get_world_views - 获取世界观",
                      "create_world_view - 创建世界观",
                      "update_world_view - 更新世界观",
                      "delete_world_view - 删除世界观",
                    ]}
                  />
                  <ApiCategory
                    title="知识库"
                    apis={[
                      "get_knowledge_entries - 获取知识条目",
                      "create_knowledge_entry - 创建知识条目",
                      "search_knowledge - 搜索知识",
                      "build_knowledge_context - 构建上下文",
                    ]}
                  />
                </div>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <Globe className="w-5 h-5 text-purple-500" />
                  AI 功能 API
                </h3>
                <div className="space-y-3">
                  <ApiCategory
                    title="AI 写作"
                    apis={[
                      "ai_continue_novel - AI 续写",
                      "ai_rewrite_content - AI 改写",
                      "ai_format_content - AI 一键排版",
                      "ai_generate_character - AI 生成角色",
                      "ai_generate_plot_points - AI 生成情节点",
                      "ai_generate_worldview - AI 生成世界观",
                      "ai_generate_storyboard - AI 生成分镜",
                    ]}
                  />
                  <ApiCategory
                    title="AI 分析"
                    apis={[
                      "analyze_writing_style - 分析写作风格",
                      "analyze_rhythm - 分析节奏",
                      "analyze_emotion - 分析情感",
                      "analyze_readability - 分析可读性",
                      "detect_repetitions - 检测重复",
                      "check_logic - 检查逻辑",
                    ]}
                  />
                </div>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 flex items-center gap-2 text-slate-900 dark:text-white">
                  <Shield className="w-5 h-5 text-yellow-500" />
                  系统设置 API
                </h3>
                <div className="space-y-3">
                  <ApiCategory
                    title="模型管理"
                    apis={[
                      "get_models - 获取模型列表",
                      "get_default_model - 获取默认模型",
                      "set_default_model - 设置默认模型",
                      "register_openai_model - 注册 OpenAI 模型",
                      "register_ollama_model - 注册 Ollama 模型",
                    ]}
                  />
                  <ApiCategory
                    title="API 密钥"
                    apis={[
                      "get_api_keys - 获取 API 密钥列表",
                      "set_api_key - 设置 API 密钥",
                      "get_bigmodel_api_key - 获取智谱 API 密钥",
                    ]}
                  />
                  <ApiCategory
                    title="AI 参数"
                    apis={["get_ai_params - 获取 AI 参数", "set_ai_params - 设置 AI 参数"]}
                  />
                </div>
              </section>

              <section>
                <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-white">
                  系统变量
                </h3>
                <p className="text-slate-600 dark:text-slate-400 mb-4">
                  插件可以访问以下系统变量：
                </p>
                <pre className="bg-slate-100 dark:bg-slate-800 p-4 rounded-lg overflow-auto text-sm">
                  <code>
                    {`// Tauri 环境
window.__TAURI__          // Tauri API 对象
window.__TAURI__.invoke   // 调用后端命令

// 插件信息
window.__PLUGIN_ID__      // 当前插件 ID
window.__PLUGIN_VERSION__ // 当前插件版本
window.__PLUGIN_PATH__    // 当前插件路径

// 应用信息
window.__APP_VERSION__    // 应用版本
window.__APP_PLATFORM__   // 运行平台 (windows/macos/linux)`}
                  </code>
                </pre>
              </section>
            </div>
          )}
        </div>

        <div className="px-6 py-4 border-t border-slate-200 dark:border-slate-700 shrink-0">
          <button
            onClick={onClose}
            className="w-full px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}

const examplePluginJson = `{
  "info": {
    "id": "my-plugin",
    "version": "1.0.0",
    "name": "我的插件",
    "description": "这是一个示例插件",
    "author": {
      "name": "开发者",
      "email": "dev@example.com"
    },
    "pluginType": "utility",
    "minAppVersion": "1.0.0",
    "homepage": "https://example.com",
    "repository": "https://github.com/example/my-plugin",
    "keywords": ["example", "demo"]
  },
  "permissions": [],
  "capabilities": ["editor"],
  "contributes": [
    {
      "type": "command",
      "id": "my-plugin.hello",
      "label": "打招呼",
      "description": "向用户打招呼",
      "icon": "icon.png"
    }
  ],
  "dependencies": {}
}`;

const downloadExamplePlugin = (type: "wordcount" | "inspiration") => {
  const plugins = {
    wordcount: {
      pluginJson: `{
  "info": {
    "id": "wordcount-plugin",
    "version": "1.0.0",
    "name": "字数统计工具",
    "description": "统计编辑器中的字数、字符数",
    "author": {
      "name": "AI Novel Studio"
    },
    "pluginType": "utility",
    "minAppVersion": "1.0.0"
  },
  "permissions": [],
  "capabilities": ["editor"],
  "contributes": [
    {
      "type": "command",
      "id": "wordcount.count",
      "label": "统计字数",
      "description": "统计当前编辑器的字数"
    }
  ],
  "dependencies": {}
}`,
      indexJs: `// 字数统计插件
console.log('字数统计插件已加载');

// 注册命令
if (window.__TAURI__ && window.__TAURI__.invoke) {
  // 示例：可以通过 Tauri API 调用主应用功能
  async function countWords() {
    console.log('统计字数功能被调用');
    // 这里可以获取编辑器内容并统计
    // 实际实现需要通过 Tauri 事件系统与主应用通信
    alert('字数统计功能已调用！');
  }

  // 暴露给外部调用
  window.wordcountPlugin = {
    countWords
  };
}`,
    },
    inspiration: {
      pluginJson: `{
  "info": {
    "id": "inspiration-plugin",
    "version": "1.0.0",
    "name": "灵感生成器",
    "description": "随机生成写作灵感和创意",
    "author": {
      "name": "AI Novel Studio"
    },
    "pluginType": "feature_module",
    "minAppVersion": "1.0.0"
  },
  "permissions": [],
  "capabilities": ["editor"],
  "contributes": [
    {
      "type": "command",
      "id": "inspiration.generate",
      "label": "生成灵感",
      "description": "随机生成一个写作灵感"
    }
  ],
  "dependencies": {}
}`,
      indexJs: `// 灵感生成器插件
console.log('灵感生成器插件已加载');

const inspirations = [
  '一个隐藏在图书馆里的古老秘密',
  '突然发现自己能听懂动物的语言',
  '收到一封来自未来的信',
  '发现一个能回到过去的怀表',
  '在废弃的游乐园里遇到了不可思议的朋友',
  '镜子里的倒影开始有了自己的意识',
  '一本会自动书写的日记',
  '一条通往异世界的神秘隧道',
  '一个永不枯萎的花园',
  '能够看到人们过去的回忆'
];

function generateInspiration() {
  const randomIndex = Math.floor(Math.random() * inspirations.length);
  return inspirations[randomIndex];
}

if (window.__TAURI__ && window.__TAURI__.invoke) {
  // 暴露给外部调用
  window.inspirationPlugin = {
    generate: generateInspiration,
    getAll: () => inspirations
  };
}`,
    },
  };

  const plugin = plugins[type];
  const pluginName = type === "wordcount" ? "wordcount-plugin" : "inspiration-plugin";
  const pluginTitle = type === "wordcount" ? "字数统计工具" : "灵感生成器";

  const files: Record<string, string> = {
    "plugin.json": plugin.pluginJson,
    "index.js": plugin.indexJs,
    "README.md": `# ${pluginTitle}

这是一个 AI Novel Studio 的示例插件。

## 安装
1. 将此文件夹放入应用的插件目录
2. 在插件管理器中点击"安装插件"
3. 输入此文件夹的路径
4. 激活插件

## 功能
${type === "wordcount" ? "统计编辑器中的字数和字符数" : "随机生成写作灵感和创意"}`,
  };

  const content = Object.entries(files)
    .map(([name, data]) => `=== ${name} ===\n${data}`)
    .join("\n\n");

  const blob = new Blob([content], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `${pluginName}.txt`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);

  setTimeout(() => {
    alert(
      `示例插件已下载！\n\n文件名：${pluginName}.txt\n\n请将内容提取到文件夹中，然后使用插件管理器安装。`
    );
  }, 100);
};
