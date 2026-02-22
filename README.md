# AI Novel Studio

AI小说创作工作室 - 一个专业的AI辅助小说创作工具

## 项目简介

AI Novel Studio 是一个基于 Tauri + React + TypeScript 开发的桌面应用，旨在为专业小说创作者提供全流程的创作支持。应用集成了 AI 辅助创作、项目管理、素材管理、多媒体生成等功能。

## 技术栈

### 前端
- **框架**: React 18 + TypeScript 5
- **UI库**: TailwindCSS + Lucide Icons
- **状态管理**: Zustand
- **构建工具**: Vite

### 后端
- **核心框架**: Tauri 2.0 (Rust)
- **数据库**: SQLite (rusqlite)
- **异步运行时**: Tokio
- **序列化**: Serde

## 项目结构

```
AI_Novel_Studio/
├── src/                          # 前端源代码
│   ├── components/                 # React组件
│   │   ├── TextEditor.tsx        # 文本编辑器
│   │   ├── ProjectList.tsx        # 项目列表
│   │   ├── ChapterList.tsx        # 章节列表
│   │   └── CreateProjectDialog.tsx # 创建项目对话框
│   ├── pages/                    # 页面组件
│   ├── stores/                   # 状态管理
│   │   └── projectStore.ts       # 项目状态store
│   ├── services/                 # API服务
│   │   └── api.ts              # Tauri API调用
│   ├── types/                    # TypeScript类型定义
│   │   └── index.ts            # 共享类型
│   ├── test/                      # 测试工具和设置
│   │   └── setup.ts            # 测试环境配置
│   ├── hooks/                    # 自定义Hooks
│   ├── utils/                    # 工具函数
│   ├── App.tsx                  # 主应用组件
│   ├── main.tsx                 # 应用入口
│   └── index.css                # 全局样式
├── src-tauri/                   # Rust后端代码
│   ├── src/
│   │   ├── main.rs             # 应用入口
│   │   ├── database.rs         # 数据库操作
│   │   ├── models.rs           # 数据模型
│   │   ├── commands.rs         # Tauri命令
│   │   └── build.rs           # 构建脚本
│   ├── tests/                    # Rust测试
│   │   └── common/            # 测试工具
│   ├── Cargo.toml             # Rust依赖配置
│   └── tauri.conf.json       # Tauri配置
├── e2e/                         # 端到端测试
│   └── project-management.spec.ts
├── public/                      # 静态资源
├── scripts/                     # 自动化脚本
│   └── run-tests.sh         # 测试运行脚本
├── docs/                        # 文档
│   ├── technical_architecture.md
│   ├── product_requirements.md
│   ├── multi_model_architecture.md
│   ├── multimedia_generation_architecture.md
│   ├── plugin_system_design.md
│   └── TESTING.md            # 测试文档
├── package.json                # Node.js依赖
├── tsconfig.json              # TypeScript配置
├── vite.config.ts            # Vite配置
├── vitest.config.ts          # 测试配置
├── playwright.config.ts       # E2E测试配置
├── tailwind.config.js        # TailwindCSS配置
└── postcss.config.js        # PostCSS配置
```

## 已实现功能

### MVP核心功能

#### 1. 项目管理
- 创建新项目
- 项目列表展示
- 项目详情查看
- 项目类型选择（玄幻、都市、科幻等）
- 项目更新和删除

#### 2. 章节管理
- 创建新章节
- 章节列表展示
- 章节内容编辑
- 字数统计
- 章节更新（标题和内容）
- 章节删除

#### 3. 角色管理
- 创建角色档案
- 角色列表展示
- 角色信息编辑
- 角色更新
- 角色删除

#### 4. 数据库
- SQLite数据库初始化
- 数据持久化存储
- 外键关联管理
- 完整的索引优化

#### 5. UI框架
- 三栏布局（项目列表、编辑器、章节列表）
- 响应式设计
- TailwindCSS样式
- 深色/浅色主题支持

### Phase 2: 核心功能增强 ✅

#### 6. AI辅助写作
- ✅ AI续写功能 - 支持流式输出
- ✅ AI改写功能 - 文本优化和风格调整
- ✅ 多模型支持 - OpenAI和Ollama适配器
- ✅ 提示词模板管理

#### 7. 世界观管理
- ✅ 创建世界观条目
- ✅ 世界观分类管理（地理、历史、魔法等）
- ✅ 世界观内容编辑
- ✅ 世界观列表展示
- ✅ 世界观更新和删除

#### 8. 情节大纲系统
- ✅ 创建情节点
- ✅ 情节点层级管理
- ✅ 情节点与章节关联
- ✅ 情节列表展示
- ✅ 情节更新和删除

#### 9. 角色关系图谱
- ✅ 创建角色关系
- ✅ 关系类型定义
- ✅ 角色关系可视化
- ✅ 关系图展示
- ✅ 关系更新和删除

### 企业级功能

#### 10. 日志系统
- ✅ 企业级日志记录
- ✅ 请求ID追踪
- ✅ 调用链追踪
- ✅ 性能指标记录
- ✅ 错误堆栈追踪
- ✅ 日志级别管理（DEBUG, INFO, WARN, ERROR）

#### 11. 测试框架
- ✅ 后端单元测试框架
- ✅ 自定义断言库
- ✅ 测试数据库工具
- ✅ 测试日志捕获
- ✅ 集成测试套件
- ✅ 前端测试（Vitest + React Testing Library）
- ✅ E2E测试（Playwright）
- ✅ 自动化测试运行脚本
- ✅ 测试覆盖率报告

## 开发指南

### 环境要求

- Node.js 18+
- Rust 1.70+
- npm 9+

### 安装依赖

```bash
npm install
```

### 开发模式

#### 启动前端开发服务器
```bash
npm run dev
```
前端将在 http://localhost:1420 运行

#### 启动Tauri开发模式（完整应用）
```bash
npm run tauri dev
```
这将启动前端和后端，并打开桌面应用窗口

### 构建生产版本

```bash
npm run tauri build
```

### 代码检查

```bash
# TypeScript检查
npm run build

# Rust检查
cd src-tauri
cargo check
```

## 核心API

### 项目管理

#### 创建项目
```typescript
await invoke('create_project', {
  request: {
    name: '项目名称',
    description: '项目描述',
    genre: '玄幻'
  }
})
```

#### 获取项目列表
```typescript
const projects = await invoke('get_projects')
```

### 章节管理

#### 保存章节
```typescript
await invoke('save_chapter', {
  request: {
    project_id: 'project-id',
    title: '第一章',
    content: '章节内容',
    sort_order: 0
  }
})
```

#### 获取章节列表
```typescript
const chapters = await invoke('get_chapters', { projectId: 'project-id' })
```

### 角色管理

#### 创建角色
```typescript
await invoke('create_character', {
  request: {
    project_id: 'project-id',
    name: '角色名',
    age: 25,
    gender: '男',
    appearance: '外貌描述',
    personality: '性格描述',
    background: '背景故事'
  }
})
```

#### 获取角色列表
```typescript
const characters = await invoke('get_characters', { projectId: 'project-id' })
```

## 数据库表结构

### projects（项目表）
- id: 项目ID (UUID)
- name: 项目名称
- description: 项目描述
- genre: 项目类型
- template: 项目模板
- status: 项目状态
- created_at: 创建时间
- updated_at: 更新时间

### chapters（章节表）
- id: 章节ID (UUID)
- project_id: 所属项目ID
- title: 章节标题
- content: 章节内容
- word_count: 字数统计
- sort_order: 排序号
- status: 章节状态
- created_at: 创建时间
- updated_at: 更新时间

### characters（角色表）
- id: 角色ID (UUID)
- project_id: 所属项目ID
- name: 角色名称
- age: 年龄
- gender: 性别
- appearance: 外貌描述
- personality: 性格描述
- background: 背景故事
- avatar_url: 头像URL
- created_at: 创建时间
- updated_at: 更新时间

## 后续开发计划

### Phase 2: 核心功能增强 ✅ 已完成
- ✅ AI续写功能
- ✅ AI改写功能
- ✅ 世界观管理
- ✅ 情节大纲系统
- ✅ 角色关系图谱
- ✅ 企业级日志系统
- ✅ 完整测试框架

### Phase 3: 多媒体生成
- [ ] 分镜脚本生成
- [ ] 剧本格式转换
- [ ] 漫画分镜生成
- [ ] 插画生成

### Phase 4: 扩展系统
- [ ] 插件系统
- [ ] 脚本系统
- [ ] 多模型集成
- [ ] 云同步功能
- [ ] 协作编辑功能
- [ ] 导出功能（PDF、EPUB、Word等）

## 注意事项

### Tauri开发环境
- macOS开发需要Xcode命令行工具
- Windows开发需要Visual Studio C++构建工具
- Linux开发需要WebKitGTK库

### 数据存储
- 数据库文件位置: `~/Library/Application Support/com.ainovelstudio.app/novel_studio.db` (macOS)
- 数据备份: 建议定期备份数据库文件

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！
