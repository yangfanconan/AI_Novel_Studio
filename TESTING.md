# AI Novel Studio - 企业级测试文档

## 📋 测试架构概述

本项目采用分层测试策略，确保代码质量和系统稳定性：

```
┌─────────────────────────────────────────┐
│         端到端测试 (E2E)          │
│         (Playwright)                  │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         集成测试 (Integration)         │
│         (Rust + Tauri)              │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         单元测试 (Unit)                 │
│  Rust: cargo test                    │
│  React: Vitest                       │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         代码覆盖率 (Coverage)            │
│         (V8 + lcov)                   │
└───────────────────────────────────────────┘
```

## 🔍 日志系统

### 日志级别

- **DEBUG**: 详细调试信息
- **INFO**: 一般信息
- **WARN**: 警告信息
- **ERROR**: 错误信息

### 日志格式

```
[timestamp][level][req:{request_id}][feat:{feature}][action:{action}][parent:{parent_id}] {message}
```

### 调用链路追踪

每个请求都有唯一的 `request_id`，支持完整调用链追踪：

```rust
let logger = Logger::new().with_feature("project-service").with_action("create_project");
logger.info("Starting project creation");

let child_logger = logger.child();
child_logger.info("Validating project data");
```

### 专用日志函数

```rust
// 命令日志
log_command_start(&logger, "create_project", &format!("{:?}", request));
log_command_success(&logger, "create_project", &format!("Created: {}", id));
log_command_error(&logger, "create_project", &error);

// 数据库操作日志
log_database_operation(&logger, "insert", "projects", "Inserting new project");

// AI 操作日志
log_ai_operation(&logger, "generate", "gpt-4", "Generating content");

// 验证错误日志
log_validation_error(&logger, "name", "Name cannot be empty");

// 性能指标日志
log_performance_metric(&logger, "query_time", 45.2, "ms");
```

## 🧪 Rust 测试

### 运行单元测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_create_project

# 运行测试并显示输出
cargo test -- --nocapture

# 运行集成测试
cargo test --test integration_tests
```

### 测试框架特性

**TestSuite** - 测试套件管理
```rust
let mut suite = TestSuite::new("Project Management");
suite.add_test("test name", || {
    assert_eq!("assertion_name", actual, expected);
});
suite.print_summary();
```

**自定义断言**
```rust
assert_eq!("name", actual, expected);
assert_ne!("name", actual, not_expected);
assert_true!("name", condition);
assert_false!("name", condition);
assert_some!("name", option_value);
assert_none!("name", option_value);
assert_contains!("name", "haystack", "needle");
assert_err!("name", result);
assert_ok!("name", result);
```

**测试数据库**
```rust
let db = TestDatabase::new()?;
let conn = db.get_connection();

// 自动清理，无需手动删除
```

**测试日志捕获**
```rust
let logger = TestLogger::new();
logger.assert_contains("expected message");
logger.assert_count(5);
```

### 测试覆盖率

```bash
# 安装工具
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html
```

## 🎨 React 测试

### 运行测试

```bash
# 运行所有测试
npm run test

# 运行 UI 模式
npm run test:ui

# 运行覆盖率测试
npm run test:coverage

# 监视模式
npm run test -- --watch
```

### 测试框架配置

**vitest.config.ts**
- 支持 jsdom 环境
- 代码覆盖率配置
- 路径别名支持

**测试设置**
```typescript
import '@testing-library/jest-dom';
import { cleanup } from '@testing-library/react';

afterEach(() => {
  cleanup();
});
```

### 组件测试示例

```typescript
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

describe('ProjectList Component', () => {
  it('should render project list correctly', async () => {
    render(<ProjectList {...props} />);
    
    await waitFor(() => {
      expect(screen.getByText('Project Name')).toBeInTheDocument();
    });
  });

  it('should handle user interactions', async () => {
    const user = userEvent.setup();
    render(<Component {...props} />);
    
    await user.click(screen.getByText('Button Text'));
    expect(mockFunction).toHaveBeenCalled();
  });
});
```

## 🎭 端到端测试

### 运行 E2E 测试

```bash
# 运行所有 E2E 测试
npm run test:e2e

# 运行 UI 模式
npm run test:e2e:ui

# 调试模式
npm run test:e2e:debug

# 运行特定浏览器
npm run test:e2e -- --project=chromium
```

### Playwright 配置

**支持浏览器**
- Chromium (Chrome)
- Firefox
- WebKit (Safari)

**特性**
- 并行测试执行
- 自动截图（失败时）
- 视频录制（失败时）
- 追踪日志
- 重试机制

### E2E 测试示例

```typescript
test('should create a new project', async ({ page }) => {
  await page.goto('/');
  await page.click('button:has-text("新建项目")');
  await page.fill('input[placeholder*="项目名称"]', 'Test Project');
  await page.selectOption('select', '玄幻');
  await page.click('button:has-text("创建")');

  await expect(page.locator('text=Test Project')).toBeVisible();
});
```

## 🚀 持续集成 (CI)

### 测试运行脚本

```bash
# 交互式测试菜单
./scripts/run-tests.sh

# 运行完整 CI 流程
./scripts/run-tests.sh 10

# 或使用 npm 脚本
npm run test:all
```

### CI 流程步骤

1. **代码检查**
   - Rust: `cargo clippy`
   - TypeScript: `npm run lint`

2. **单元测试**
   - Rust: `cargo test`
   - React: `npm run test:coverage`

3. **集成测试**
   - `cargo test --test integration_tests`

4. **端到端测试**
   - `npm run test:e2e`

5. **生成报告**
   - 测试覆盖率报告
   - 测试结果汇总

### GitHub Actions 配置示例

```yaml
name: Test Suite

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          
      - name: Install dependencies
        run: |
          cd src-tauri
          cargo fetch --locked
          cargo build --release
          cd ..
          npm ci
          
      - name: Run tests
        run: ./scripts/run-tests.sh 10
        
      - name: Upload coverage
        uses: codecov/codecov-action@v4
```

## 📊 测试覆盖率目标

| 模块 | 目标覆盖率 | 当前状态 |
|--------|-------------|---------|
| 后端 (Rust) | > 80% | 待测量 |
| 前端 (React) | > 80% | 待测量 |
| 集成测试 | > 70% | 待测量 |
| 总体覆盖率 | > 75% | 待测量 |

## 📝 编写测试指南

### 单元测试原则

1. **快速**: 每个测试应该在几秒内完成
2. **独立**: 测试之间不应有依赖关系
3. **可重复**: 测试结果应该是一致的
4. **有意义**: 测试名称应该清楚地描述测试内容

### 集成测试原则

1. **测试完整流程**: 测试从开始到结束的完整用户旅程
2. **使用真实依赖**: 与实际数据库、API 交互
3. **清理资源**: 测试后清理创建的数据
4. **处理异步**: 正确处理异步操作

### E2E 测试原则

1. **用户视角**: 模拟真实用户操作
2. **跨浏览器**: 在多个浏览器中测试
3. **失败截图**: 自动截图帮助调试
4. **等待稳定**: 使用适当的等待策略

### 测试命名约定

- Rust: `test_<module>_<action>_<scenario>`
- React: `should <action> when <scenario>`
- E2E: `should <action> <resource>`

## 🔧 故障排查

### 常见问题

**测试超时**
- 增加超时时间
- 检查异步操作
- 验证测试环境

**不稳定测试**
- 检查测试顺序依赖
- 添加适当的等待
- 使用 test.each 处理变化

**覆盖率不足**
- 添加边界情况测试
- 测试错误处理路径
- 增加集成测试

### 调试技巧

```bash
# Rust 测试调试
cargo test -- --nocapture -- --show-output

# 查看详细日志
RUST_LOG=debug cargo test

# React 测试调试
npm run test:ui

# E2E 测试调试
npm run test:e2e:debug
```

## 📚 参考资料

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [React Testing Library](https://testing-library.com/react)
- [Vitest Documentation](https://vitest.dev/)
- [Playwright Documentation](https://playwright.dev/)

## 🎯 测试检查清单

在提交代码前，确保：

- [ ] 所有单元测试通过
- [ ] 代码覆盖率达标
- [ ] 集成测试通过
- [ ] E2E 测试通过
- [ ] 代码检查通过
- [ ] 日志记录完整
- [ ] 错误处理完善
- [ ] 性能指标正常
