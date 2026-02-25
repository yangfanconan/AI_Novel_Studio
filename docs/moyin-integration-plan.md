# Moyin-Creator 集成计划

## 项目分析

### Moyin-Creator 核心功能
魔因漫创是一款专业的 AI 影视生产工具，核心功能包括：

1. **剧本解析引擎** - 智能拆解剧本为场景、分镜、对白
2. **角色一致性系统** - 6层身份锚点，角色圣经管理
3. **场景生成** - 多视角联合图生成
4. **专业分镜系统** - 电影级摄影参数（景别、机位、运动）
5. **Seedance 2.0 支持** - 多镜头合并叙事视频生成
6. **批量化生产工作流** - 全流程自动化
7. **多供应商AI调度** - API Key轮询负载均衡

### 关键技术组件
- `@opencut/ai-core` 包：
  - `PromptCompiler` - 提示词模板编译器（Mustache风格）
  - `CharacterBibleManager` - 角色一致性管理
  - `TaskPoller` - 异步任务轮询（动态超时）
  - `TaskQueue` - 任务队列管理

## 集成方案

### 推荐方案：渐进式功能移植

考虑到 InfiniteNote 已有完善的多媒体生成模块，建议采用功能移植方式，将 moyin-creator 的核心能力整合到现有架构中。

## 集成计划

### Phase 1: AI Core 引擎移植（高优先级）

#### 1.1 提示词编译器
- **文件**: `src-tauri/src/ai/prompt_compiler.rs`
- **功能**: 
  - Mustache风格模板引擎
  - 场景图片/视频提示词生成
  - 剧本生成提示词
  - 负面提示词管理

#### 1.2 角色圣经管理器
- **文件**: `src-tauri/src/ai/character_bible.rs`
- **功能**:
  - 角色视觉特征管理
  - 风格令牌管理
  - 色彩调色板
  - 参考图绑定
  - 三视图生成
  - 一致性提示词生成

#### 1.3 任务轮询器
- **文件**: `src-tauri/src/ai/task_poller.rs`
- **功能**:
  - 异步任务状态轮询
  - 动态超时调整
  - 进度回调
  - 取消支持

### Phase 2: 剧本解析引擎（中优先级）

#### 2.1 剧本解析器
- **文件**: `src-tauri/src/script_parser/`
- **功能**:
  - 智能识别场景、分镜、对白
  - 角色自动识别
  - 情绪/镜头语言解析
  - 多集/多幕结构支持

#### 2.2 分镜管理
- **扩展**: `src-tauri/src/multimedia_generation/storyboard.rs`
- **功能**:
  - 电影级摄影参数
  - 景别/机位/运动方式
  - 自动排版导出

### Phase 3: 多供应商调度（中优先级）

#### 3.1 供应商管理
- **文件**: `src-tauri/src/ai/provider_scheduler.rs`
- **功能**:
  - 多API Key轮询
  - 负载均衡
  - 失败自动重试
  - 供应商健康检查

#### 3.2 任务队列
- **文件**: `src-tauri/src/ai/task_queue.rs`
- **功能**:
  - 批量任务管理
  - 优先级队列
  - 并发控制
  - 进度追踪

### Phase 4: 前端集成（高优先级）

#### 4.1 角色一致性面板
- **文件**: `src/components/CharacterConsistencyPanel.tsx`
- **功能**:
  - 角色视觉特征编辑
  - 参考图上传
  - 三视图预览
  - 一致性检查

#### 4.2 剧本解析面板
- **文件**: `src/components/ScriptParserPanel.tsx`
- **功能**:
  - 剧本导入
  - 场景/分镜预览
  - 一键生成按钮

#### 4.3 批量生产面板
- **文件**: `src/components/BatchProductionPanel.tsx`
- **功能**:
  - 任务队列管理
  - 批量生图/生视频
  - 进度监控

### Phase 5: Seedance 2.0 集成（低优先级）

- 多镜头合并叙事视频
- 多模态引用（@Image/@Video/@Audio）
- 首帧图网格拼接
- 参数约束校验

## 数据库扩展

### 新增表
```sql
-- 角色圣经
CREATE TABLE character_bibles (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    visual_traits TEXT,
    style_tokens TEXT,
    color_palette TEXT,
    personality TEXT,
    reference_images TEXT,
    three_view_images TEXT,
    created_at TEXT,
    updated_at TEXT
);

-- 剧本场景
CREATE TABLE script_scenes (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    scene_id INTEGER,
    narration TEXT,
    visual_content TEXT,
    action TEXT,
    camera TEXT,
    character_description TEXT,
    created_at TEXT
);

-- AI任务队列
CREATE TABLE ai_task_queue (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    task_type TEXT NOT NULL,
    status TEXT NOT NULL,
    provider TEXT,
    input_data TEXT,
    output_data TEXT,
    error_message TEXT,
    retry_count INTEGER DEFAULT 0,
    created_at TEXT,
    updated_at TEXT
);
```

## 预计工作量

| 阶段 | 内容 | 预计时间 |
|------|------|----------|
| Phase 1 | AI Core 引擎移植 | 2-3天 |
| Phase 2 | 剧本解析引擎 | 2-3天 |
| Phase 3 | 多供应商调度 | 1-2天 |
| Phase 4 | 前端集成 | 2-3天 |
| Phase 5 | Seedance 2.0 | 2-3天 |
| **总计** | | **9-14天** |

## 执行顺序

1. ✅ 分析 moyin-creator 项目结构和核心功能
2. 🔄 Phase 1: AI Core 引擎移植（优先）
3. ⏳ Phase 4: 前端集成（与Phase 1并行）
4. ⏳ Phase 2: 剧本解析引擎
5. ⏳ Phase 3: 多供应商调度
6. ⏳ Phase 5: Seedance 2.0 集成

## 注意事项

1. **许可证**: moyin-creator 采用 AGPL-3.0，需注意开源义务
2. **技术栈差异**: moyin-creator 是 Electron + React，需适配 Tauri + React
3. **状态管理**: moyin-creator 使用 Zustand，InfiniteNote 使用 React State
4. **存储**: moyin-creator 使用文件存储，需适配 SQLite

## 立即开始的任务

1. 创建 `src-tauri/src/ai/prompt_compiler.rs` - 提示词编译器
2. 创建 `src-tauri/src/ai/character_bible.rs` - 角色圣经管理
3. 创建 `src-tauri/src/ai/task_poller.rs` - 任务轮询器
4. 扩展数据库 schema
5. 创建前端组件

---

请确认此计划，我将立即开始实施 Phase 1。
