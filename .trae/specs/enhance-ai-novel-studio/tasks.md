# Tasks

## Phase 1: Bug 修复（高优先级）

- [x] Task 1: 修复项目删除功能
  - [x] SubTask 1.1: 修改前端 `api.ts` 中 `deleteProject` 的参数名从 `id` 改为 `projectId`
  - [x] SubTask 1.2: 验证删除功能正常工作

- [x] Task 2: 修复章节删除功能
  - [x] SubTask 2.1: 检查前端 `deleteChapter` 参数传递
  - [x] SubTask 2.2: 验证章节删除功能正常工作

- [x] Task 3: 修复对话框不显示的问题
  - [x] SubTask 3.1: 确保 `ResizableLayout` 正确渲染 `children`
  - [x] SubTask 3.2: 验证所有对话框（新建项目、设置）正常打开

## Phase 2: AI 生成功能

- [x] Task 4: 后端 - AI 生成角色功能
  - [x] SubTask 4.1: 在 `commands.rs` 添加 `ai_generate_character` 命令
  - [x] SubTask 4.2: 在 `ai/service.rs` 添加角色生成逻辑和提示词模板
  - [x] SubTask 4.3: 添加前端 API 接口

- [x] Task 5: 前端 - AI 生成角色 UI
  - [x] SubTask 5.1: 在 `CharacterList.tsx` 添加"AI 生成角色"按钮
  - [x] SubTask 5.2: 创建角色生成对话框组件
  - [x] SubTask 5.3: 实现生成预览和确认功能

- [x] Task 6: 后端 - AI 生成角色关系功能
  - [x] SubTask 6.1: 在 `commands.rs` 添加 `ai_generate_character_relations` 命令
  - [x] SubTask 6.2: 添加关系生成逻辑和提示词模板
  - [x] SubTask 6.3: 添加前端 API 接口

- [x] Task 7: 前端 - AI 生成角色关系 UI
  - [x] SubTask 7.1: 在 `CharacterRelationGraph.tsx` 添加"AI 生成关系"按钮
  - [x] SubTask 7.2: 实现关系建议预览和确认功能

- [x] Task 8: 后端 - AI 生成世界观功能
  - [x] SubTask 8.1: 在 `commands.rs` 添加 `ai_generate_worldview` 命令
  - [x] SubTask 8.2: 添加世界观生成逻辑和提示词模板
  - [x] SubTask 8.3: 添加前端 API 接口

- [x] Task 9: 前端 - AI 生成世界观 UI
  - [x] SubTask 9.1: 在 `WorldViewList.tsx` 添加"AI 生成世界观"按钮
  - [x] SubTask 9.2: 创建世界观生成对话框
  - [x] SubTask 9.3: 实现生成预览和编辑功能

- [x] Task 10: 后端 - AI 生成情节点功能
  - [x] SubTask 10.1: 在 `commands.rs` 添加 `ai_generate_plot_points` 命令
  - [x] SubTask 10.2: 添加情节点生成逻辑和提示词模板
  - [x] SubTask 10.3: 添加前端 API 接口

- [x] Task 11: 前端 - AI 生成情节点 UI
  - [x] SubTask 11.1: 在 `PlotPointList.tsx` 添加"AI 生成情节点"按钮
  - [x] SubTask 11.2: 实现生成预览和确认功能

## Phase 3: 多媒体内容生成

- [x] Task 12: 后端 - 分镜提示词生成
  - [x] SubTask 12.1: 在 `commands.rs` 添加 `ai_generate_storyboard` 命令
  - [x] SubTask 12.2: 添加分镜生成逻辑和结构化输出模板
  - [x] SubTask 12.3: 添加前端 API 接口

- [x] Task 13: 前端 - 分镜提示词 UI
  - [x] SubTask 13.1: 在编辑器工具栏添加"生成分镜"按钮
  - [x] SubTask 13.2: 创建分镜预览对话框
  - [x] SubTask 13.3: 支持复制和导出分镜内容

## Phase 4: 排版与阅读系统

- [x] Task 14: 后端 - AI 一键排版功能
  - [x] SubTask 14.1: 在 `commands.rs` 添加 `ai_format_content` 命令
  - [x] SubTask 14.2: 添加排版规则（对话加粗、段落优化等）
  - [x] SubTask 14.3: 添加前端 API 接口

- [x] Task 15: 前端 - 排版功能 UI
  - [x] SubTask 15.1: 在 `AIToolbar.tsx` 添加"AI 排版"按钮
  - [x] SubTask 15.2: 实现排版预览和应用功能
  - [x] SubTask 15.3: 支持排版选项配置

- [x] Task 16: 阅读模式
  - [x] SubTask 16.1: 创建 `ReadingMode.tsx` 组件
  - [x] SubTask 16.2: 实现沉浸式阅读界面
  - [x] SubTask 16.3: 添加字体大小、背景色等个性化选项

## Phase 5: 系统设置增强

- [x] Task 17: 系统设置完善
  - [x] SubTask 17.1: 在 `ModelSettingsDialog.tsx` 添加默认模型选择
  - [x] SubTask 17.2: 添加 AI 参数配置（温度、最大 token 等）
  - [x] SubTask 17.3: 添加 API 密钥管理功能
  - [x] SubTask 17.4: 实现设置持久化存储

---

# Task Dependencies

- [Task 5] depends on [Task 4]
- [Task 7] depends on [Task 6]
- [Task 9] depends on [Task 8]
- [Task 11] depends on [Task 10]
- [Task 13] depends on [Task 12]
- [Task 15] depends on [Task 14]
- [Task 16] 可以独立进行
- [Task 17] 可以独立进行

---

# Parallelizable Work

以下任务可以并行执行：
- Phase 1 所有 Bug 修复任务可以并行
- Phase 2 中 Task 4-5, Task 6-7, Task 8-9, Task 10-11 可以分组并行
- Phase 3, 4, 5 可以在 Phase 2 完成后并行开始
