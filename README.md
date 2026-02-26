<div align="center">

# 🚀 AI Novel Studio

[![Tauri](https://img.shields.io/badge/Tauri-2.0.0-blue?logo=tauri&logoColor=white)](https://tauri.app)
[![React](https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=white)](https://react.dev)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.5-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Active-success)](https://github.com/yangfan-ai/ai-novel-studio)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-informational)](https://github.com/tauri-apps/tauri)

**[English](#english) | [中文](#中文)**

</div>

---

## 中文

<div align="center">

### ⚡ 下一代 AI 辅助小说创作工作台

一个专业级、全功能的 AI 小说创作工作室，融合了智能写作辅助、多媒体生成、协作编辑、插件系统和云同步等企业级特性。

</div>

---

## ✨ 核心特性

### 🎯 全流程创作支持
- **项目管理** - 创建、组织、导出小说项目，支持多类型（玄幻、都市、科幻等）
- **章节管理** - 章节编辑、字数统计、排序管理
- **角色系统** - 完整的角色档案、关系图谱、成长追踪
- **世界观构建** - 地理、历史、魔法体系等世界观管理
- **情节大纲** - 树状情节结构、情节点管理、与章节关联

### 🤖 AI 智能辅助
- **智能续写** - 基于上下文的 AI 流式续写
- **文本改写** - 风格调整、语言优化、内容精炼
- **多模型支持** - OpenAI、Ollama 本地模型适配
- **提示词管理** - 自定义 AI 提示词模板

### 🎭 智能写作引擎（L1-L3 三层架构）
- **L1 规划层（蓝图系统）**
  - 项目蓝图管理 - 定义整体创作方向和风格
  - 角色关系图谱 - 可视化角色关系网络
  - 世界观设定 - 构建完整的世界规则体系
- **L2 导演层（章节导演）**
  - 章节导演脚本 - 为每章定义创作目标、基调、节奏
  - 宏观/微观节拍 - 精细控制章节情节点
  - 跨章节拍选择 - 从大纲关联节拍到章节
  - 后置护栏检查 - 自动检查内容合规性（长度、禁止角色/话题）
- **L3 写作层（智能执行）**
  - 导演脚本注入 - AI写作时自动加载章节导演指令
  - 信息可见性过滤 - 自动过滤禁止角色，防止信息泄露
  - 向量检索（RAG）- 语义搜索相关章节内容作为上下文
  - 章节向量化 - 自动将章节分块存储到向量数据库
  - 自动摘要生成 - AI 自动生成章节摘要
  - AI 多版本评审 - 生成多版本内容并自动评估

### 🎬 影视化创作工具 (moyin-creator 集成)
- **Seedance 2.0** - 多模态参考（图片/视频/音频）构建 AI 提示词
- **分镜系统** - 专业分镜编辑器，支持 12 种镜头类型、6 种拍摄角度、16 种运镜方式
- **ComfyUI 集成** - AI 图像生成工作流支持
- **剧本转换** - 小说转剧本格式（好莱坞/中国/日本标准）
- **场景提取** - 智能提取小说场景用于可视化

### 🔌 插件系统
- **多脚本引擎** - 支持 JavaScript、Python、Lua 脚本
- **插件生命周期** - 安装、启用、禁用、卸载
- **插件市场** - 浏览、搜索、下载社区插件
- **API 扩展** - 插件可访问核心创作 API

### ☁️ 云同步与协作
- **多云支持** - Google Drive、Dropbox、OneDrive、iCloud、WebDAV
- **实时协作** - 多用户同时编辑，光标同步、操作广播
- **冲突解决** - 基于时间戳的自动冲突处理
- **同步历史** - 完整的同步日志和版本回溯

### 🛠️ 开发者工具
- **插件开发向导** - 内置插件开发文档和示例
- **日志系统** - 企业级日志记录，支持 DEBUG/INFO/WARN/ERROR 级别
- **测试框架** - Vitest + Playwright E2E 测试
- **代码质量** - ESLint + Prettier + TypeScript 严格模式

---

## 🏗️ 技术架构

### 前端技术栈
```
React 18          ──────────>  UI 框架
TypeScript 5.5     ──────────>  类型安全
TailwindCSS        ──────────>  样式系统
Zustand           ──────────>  状态管理
Lucide Icons       ──────────>  图标库
Vite              ──────────>  构建工具
```

### 后端技术栈
```
Tauri 2.0         ──────────>  桌面应用框架
Rust               ──────────>  核心语言
Tokio              ──────────>  异步运行时
SQLite             ──────────>  数据持久化
Serde              ──────────>  序列化框架
```

---

## 📦 安装

### 从源码构建

#### 环境要求
- Node.js 18+
- Rust 1.70+
- npm 9+

#### 克隆仓库
```bash
git clone https://github.com/yangfan-ai/ai-novel-studio.git
cd ai-novel-studio
```

#### 安装依赖
```bash
npm install
```

#### 开发模式
```bash
npm run tauri dev
```

#### 生产构建
```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

---

## 🎮 使用指南

### 快速开始

1. **创建项目** - 点击"新建项目"，选择类型和模板
2. **编辑章节** - 在文本编辑器中开始写作
3. **AI 辅助** - 使用 AI 工具栏进行续写或改写
4. **管理素材** - 在右侧面板添加角色、世界观、情节
5. **导出作品** - 支持多种格式导出

### 插件开发

1. 打开 **插件管理器** → **开发向导**
2. 查看插件 API 文档和示例代码
3. 创建插件目录并实现 ScriptEngine trait
4. 安装测试插件

---

## 📸 功能展示

| 模块 | 功能 | 状态 |
|--------|------|--------|
| 项目管理 | 创建/编辑/删除/导出 | ✅ |
| 章节编辑 | 富文本/字数统计/历史记录 | ✅ |
| 角色系统 | 档案/关系/成长追踪 | ✅ |
| 世界观 | 分类管理/内容编辑 | ✅ |
| 情节大纲 | 树状结构/情节点 | ✅ |
| AI 续写 | 流式输出/多模型 | ✅ |
| AI 改写 | 风格调整/内容优化 | ✅ |
| **智能写作引擎** | **L1-L3 三层架构** | ✅ |
| ├─ L1 规划层 | 蓝图/角色关系/世界观 | ✅ |
| ├─ L2 导演层 | 章节导演脚本/节拍/护栏 | ✅ |
| └─ L3 写作层 | RAG检索/向量化/摘要 | ✅ |
| 多媒体生成 | 分镜/剧本/插画 | ✅ |
| ComfyUI | 工作流管理 | ✅ |
| Seedance | 多模态提示词 | ✅ |
| 插件系统 | JS/Python/Lua | ✅ |
| 插件市场 | 浏览/下载/评价 | ✅ |
| 云同步 | 多云/冲突解决 | ✅ |
| 实时协作 | 多用户/光标同步 | ✅ |
| 日志系统 | 企业级追踪 | ✅ |
| 测试框架 | 单元/E2E/覆盖率 | ✅ |

---

## 🗂 项目结构

```
ai-novel-studio/
├── src/                          # 前端源代码
│   ├── components/                 # React 组件 (45+)
│   ├── services/                   # API 服务层
│   ├── stores/                    # Zustand 状态管理
│   ├── types/                     # TypeScript 类型定义
│   ├── utils/                     # 工具函数
│   └── App.tsx                   # 主应用入口
├── src-tauri/                   # Rust 后端
│   ├── src/
│   │   ├── ai/                   # AI 模块 (12)
│   │   ├── database/              # 数据库操作
│   │   ├── plugin_system/         # 插件系统
│   │   ├── cloud_sync/            # 云同步模块
│   │   └── commands/             # Tauri 命令 (69+)
│   └── Cargo.toml                # Rust 依赖配置
├── e2e/                         # E2E 测试
├── tests/                        # 单元测试
├── docs/                         # 技术文档
└── scripts/                      # 自动化脚本
```

---

## 🤝 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 代码规范

```bash
# 运行代码检查
npm run lint

# 自动修复 lint 问题
npm run lint:fix

# 格式化代码
npm run format

# 运行测试
npm run test:all
```

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 🙏 致谢

- [Tauri](https://tauri.app) - 强大的桌面应用框架
- [React](https://react.dev) - 声明式 UI 库
- [TailwindCSS](https://tailwindcss.com) - 实用优先的 CSS 框架
- [Lucide](https://lucide.dev) - 精美的图标库

---

## 📮 联系我们

- **Issues**: [GitHub Issues](https://github.com/yangfan-ai/ai-novel-studio/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yangfan-ai/ai-novel-studio/discussions)

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给我们一个 Star！**

Made with ❤️ by AI Novel Studio Team

</div>

---

## English

<div align="center">

### ⚡ Next-Gen AI-Powered Novel Creation Workspace

A professional, full-featured AI novel creation studio integrating intelligent writing assistance, multimedia generation, collaborative editing, plugin system, and cloud synchronization with enterprise-grade features.

</div>

---

## ✨ Key Features

### 🎯 Full-Stack Creative Support
- **Project Management** - Create, organize, and export novel projects with multiple genres (Fantasy, Urban, Sci-Fi, etc.)
- **Chapter Management** - Chapter editing, word count, and sorting
- **Character System** - Complete character profiles, relationship graphs, and growth tracking
- **World Building** - Manage geography, history, magic systems, and more
- **Plot Outlining** - Tree-based plot structure, plot points, and chapter associations

### 🤖 AI Intelligent Assistance
- **Smart Continuation** - Context-aware AI streaming continuation
- **Text Rewriting** - Style adjustment, language optimization, and content refinement
- **Multi-Model Support** - OpenAI, Ollama local model adapters
- **Prompt Management** - Custom AI prompt templates

### � Intelligent Writing Engine (L1-L3 Three-Layer Architecture)
- **L1 Planning Layer (Blueprint System)**
  - Project Blueprint Management - Define overall creative direction and style
  - Character Relationship Graph - Visualize character relationship networks
  - World View Settings - Build complete world rule systems
- **L2 Director Layer (Chapter Director)**
  - Chapter Director Script - Define creative goals, tone, and pace for each chapter
  - Macro/Micro Beats - Fine-grained control of chapter plot points
  - Cross-Chapter Beat Selection - Associate beats from outline to chapters
  - Post Guardrails Check - Automatic content compliance checking (length, forbidden characters/topics)
- **L3 Writing Layer (Intelligent Execution)**
  - Director Script Injection - Automatically load chapter director instructions during AI writing
  - Information Visibility Filtering - Automatically filter forbidden characters to prevent info leakage
  - Vector Retrieval (RAG) - Semantic search for relevant chapter content as context
  - Chapter Vectorization - Automatically chunk and store chapters in vector database
  - Auto Summary Generation - AI automatically generates chapter summaries
  - AI Multi-Version Review - Generate multiple versions and automatically evaluate

### �� Cinematic Creation Tools (moyin-creator Integration)
- **Seedance 2.0** - Multi-modal reference (image/video/audio) for AI prompt building
- **Storyboard System** - Professional storyboard editor with 12 shot types, 6 camera angles, 16 movements
- **ComfyUI Integration** - AI image generation workflow support
- **Script Conversion** - Novel to script formats (Hollywood/Chinese/Japanese standards)
- **Scene Extraction** - Intelligent scene extraction for visualization

### 🔌 Plugin System
- **Multi-Script Engines** - Support for JavaScript, Python, Lua scripts
- **Plugin Lifecycle** - Install, enable, disable, uninstall
- **Plugin Marketplace** - Browse, search, and download community plugins
- **API Extensions** - Plugins can access core creative APIs

### ☁️ Cloud Sync & Collaboration
- **Multi-Cloud Support** - Google Drive, Dropbox, OneDrive, iCloud, WebDAV
- **Real-time Collaboration** - Multi-user editing with cursor sync and operation broadcasting
- **Conflict Resolution** - Timestamp-based automatic conflict handling
- **Sync History** - Complete sync logs and version rollback

### 🛠️ Developer Tools
- **Plugin Development Wizard** - Built-in plugin docs and examples
- **Logging System** - Enterprise-grade logging with DEBUG/INFO/WARN/ERROR levels
- **Testing Framework** - Vitest + Playwright E2E testing
- **Code Quality** - ESLint + Prettier + TypeScript strict mode

---

## 🏗️ Tech Stack

### Frontend
```
React 18          ──────────>  UI Framework
TypeScript 5.5     ──────────>  Type Safety
TailwindCSS        ──────────>  Styling System
Zustand           ──────────>  State Management
Lucide Icons       ──────────>  Icon Library
Vite              ──────────>  Build Tool
```

### Backend
```
Tauri 2.0         ──────────>  Desktop App Framework
Rust               ──────────>  Core Language
Tokio              ──────────>  Async Runtime
SQLite             ──────────>  Data Persistence
Serde              ──────────>  Serialization Framework
```

---

## 📦 Installation

### Build from Source

#### Requirements
- Node.js 18+
- Rust 1.70+
- npm 9+

#### Clone Repository
```bash
git clone https://github.com/yangfan-ai/ai-novel-studio.git
cd ai-novel-studio
```

#### Install Dependencies
```bash
npm install
```

#### Development Mode
```bash
npm run tauri dev
```

#### Production Build
```bash
npm run tauri build
```

Build artifacts are located in `src-tauri/target/release/bundle/`

---

## 🎮 Usage Guide

### Quick Start

1. **Create Project** - Click "New Project", select type and template
2. **Edit Chapters** - Start writing in the text editor
3. **AI Assistance** - Use AI toolbar for continuation or rewriting
4. **Manage Assets** - Add characters, world views, and plots in the right panel
5. **Export Works** - Support for multiple export formats

### Plugin Development

1. Open **Plugin Manager** → **Development Wizard**
2. View plugin API docs and example code
3. Create plugin directory and implement ScriptEngine trait
4. Install test plugin

---

## 📸 Feature Matrix

| Module | Features | Status |
|----------|-----------|---------|
| Project Management | Create/Edit/Delete/Export | ✅ |
| Chapter Editing | Rich Text/Word Count/History | ✅ |
| Character System | Profiles/Relationships/Growth | ✅ |
| World Building | Categories/Content Editing | ✅ |
| Plot Outlining | Tree Structure/Plot Points | ✅ |
| AI Continuation | Streaming/Multi-Model | ✅ |
| AI Rewriting | Style Adjustment/Optimization | ✅ |
| **Intelligent Writing Engine** | **L1-L3 Three-Layer Architecture** | ✅ |
| ├─ L1 Planning Layer | Blueprint/Relations/World View | ✅ |
| ├─ L2 Director Layer | Chapter Director/Beats/Guardrails | ✅ |
| └─ L3 Writing Layer | RAG/Vectorization/Summary | ✅ |
| Multimedia Generation | Storyboard/Script/Illustration | ✅ |
| ComfyUI | Workflow Management | ✅ |
| Seedance | Multi-modal Prompts | ✅ |
| Plugin System | JS/Python/Lua | ✅ |
| Plugin Marketplace | Browse/Download/Ratings | ✅ |
| Cloud Sync | Multi-Cloud/Conflict Resolution | ✅ |
| Real-time Collaboration | Multi-User/Cursor Sync | ✅ |
| Logging System | Enterprise Tracking | ✅ |
| Testing Framework | Unit/E2E/Coverage | ✅ |

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Code Standards

```bash
# Run linting
npm run lint

# Auto-fix lint issues
npm run lint:fix

# Format code
npm run format

# Run tests
npm run test:all
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app) - Powerful desktop application framework
- [React](https://react.dev) - Declarative UI library
- [TailwindCSS](https://tailwindcss.com) - Utility-first CSS framework
- [Lucide](https://lucide.dev) - Beautiful icon library

---

## 📮 Contact Us

- **Issues**: [GitHub Issues](https://github.com/yangfan-ai/ai-novel-studio/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yangfan-ai/ai-novel-studio/discussions)

---

<div align="center">

**⭐ If this project helps you, please give us a Star!**

Made with ❤️ by AI Novel Studio Team

</div>
