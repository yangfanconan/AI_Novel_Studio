# Moyin-Creator 集成开发计划

## 项目背景

将开源项目 [moyin-creator](https://github.com/MemeCalculate/moyin-creator) 的核心功能集成到 InfiniteNote AI_Novel_Studio 中。

### Moyin-Creator 核心能力
- 📝 剧本解析引擎 - 智能拆解剧本为场景、分镜、对白
- 🎭 角色一致性系统 - 6层身份锚点，角色圣经管理
- 🖼️ 场景生成 - 多视角联合图生成
- 🎞️ 专业分镜系统 - 电影级摄影参数
- ⭐ Seedance 2.0 - 多镜头合并叙事视频
- 🚀 批量化生产 - 全流程自动化
- 🤖 多供应商AI调度 - API轮询负载均衡

## 开发计划

### 阶段1: AI Core 引擎移植（立即开始）

**任务列表:**
1. 创建提示词编译器 `src-tauri/src/ai/prompt_compiler.rs`
2. 创建角色圣经管理器 `src-tauri/src/ai/character_bible.rs`
3. 创建任务轮询器 `src-tauri/src/ai/task_poller.rs`
4. 创建任务队列 `src-tauri/src/ai/task_queue.rs`
5. 创建供应商调度器 `src-tauri/src/ai/provider_scheduler.rs`
6. 扩展数据库 schema（character_bibles, script_scenes, ai_task_queue）
7. 注册新命令到 main.rs

**预计时间:** 2-3天

### 阶段2: 剧本解析引擎

**任务列表:**
1. 创建剧本解析器模块 `src-tauri/src/script_parser/`
2. 实现场景/分镜/对白自动识别
3. 实现角色/情绪/镜头语言解析
4. 支持多集/多幕剧本结构

**预计时间:** 2-3天

### 阶段3: 前端集成

**任务列表:**
1. 创建角色一致性面板 `src/components/CharacterConsistencyPanel.tsx`
2. 创建剧本解析面板 `src/components/ScriptParserPanel.tsx`
3. 创建批量生产面板 `src/components/BatchProductionPanel.tsx`
4. 集成到主界面 App.tsx
5. 集成到 ProjectList.tsx 工具栏

**预计时间:** 2-3天

### 阶段4: Seedance 2.0 集成（可选）

**任务列表:**
1. 多镜头合并叙事视频生成
2. 多模态引用支持（@Image/@Video/@Audio）
3. 首帧图网格拼接
4. 参数约束校验

**预计时间:** 2-3天

## 立即执行的任务（阶段1第一批）

1. ✅ 分析 moyin-creator 项目结构
2. 🔄 创建 `src-tauri/src/ai/prompt_compiler.rs`
3. 🔄 创建 `src-tauri/src/ai/character_bible.rs`
4. 🔄 创建 `src-tauri/src/ai/task_poller.rs`
5. ⏳ 扩展数据库 schema
6. ⏳ 注册命令到 main.rs
7. ⏳ 创建前端服务层

## 技术要点

### 提示词编译器
- Mustache风格模板引擎 `{{variable}}`
- 支持场景图片/视频提示词生成
- 支持剧本生成提示词
- 负面提示词管理

### 角色圣经
- 视觉特征管理（visualTraits）
- 风格令牌（styleTokens）
- 色彩调色板（colorPalette）
- 参考图绑定（referenceImages）
- 三视图生成（threeViewImages）
- 一致性提示词生成

### 任务轮询
- 异步任务状态轮询
- 动态超时调整（基于服务器估计时间）
- 进度回调
- 取消支持
- 网络错误重试

### 任务队列
- 批量任务管理
- 优先级队列
- 并发控制
- 进度追踪
- 失败自动重试

## 注意事项

1. **许可证合规**: moyin-creator 采用 AGPL-3.0 许可证
2. **技术栈适配**: 从 Electron 适配到 Tauri
3. **存储适配**: 从文件存储适配到 SQLite
4. **状态管理**: 从 Zustand 适配到 React State
