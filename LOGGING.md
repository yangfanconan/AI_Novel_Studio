# 日志系统文档

## 概述

本项目的日志系统提供企业级的日志追踪能力，支持：
- 多级别日志（DEBUG, INFO, WARN, ERROR）
- 请求链路追踪
- 性能监控
- 错误上下文记录

## 前端日志系统

### 日志类位置
`src/utils/logger.ts`

### 日志级别
```typescript
enum LogLevel {
  DEBUG = 'DEBUG',
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR',
}
```

### 日志上下文
```typescript
interface LogContext {
  feature?: string;      // 功能模块
  action?: string;       // 具体操作
  userId?: string;       // 用户ID
  requestId?: string;     // 请求ID
  component?: string;     // 组件名称
  // ... 其他自定义字段
}
```

### 使用示例

#### 基本日志记录
```typescript
import { logger } from './utils/logger';

// 记录信息
logger.info('Operation completed');

// 记录警告
logger.warn('Potential issue detected');

// 记录错误
logger.error('Operation failed', error, { userId: '123' });
```

#### 带上下文的日志
```typescript
logger.info('Creating project', { 
  feature: 'project-service',
  action: 'createProject',
  projectId: '123',
  data: { name: 'Test Project' }
});
```

#### 动作追踪
```typescript
const track = logger.trackAction('complexOperation');
try {
  // 执行操作
  performComplexOperation();
  track(); // 记录完成和耗时
} catch (error) {
  logger.error('Operation failed', error, { action: 'complexOperation' });
}
```

#### 创建子Logger
```typescript
import { createLogger } from './utils/logger';

const projectLogger = createLogger({ 
  feature: 'project-service',
  userId: currentUserId 
});

projectLogger.info('Project operation');
```

## 后端日志系统

### 日志模块位置
`src-tauri/src/logger.rs`

### 日志级别
```rust
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
```

### 使用示例

#### 基本日志
```rust
use logger::Logger;

let logger = Logger::new();

logger.debug("Debug message");
logger.info("Info message");
logger.warn("Warning message");
logger.error("Error message");
```

#### 带上下文的日志
```rust
let logger = Logger::new()
    .with_feature("project-service")
    .with_action("create_project")
    .with_request_id("1234567890");

logger.info("Processing project creation");
```

#### 动作追踪
```rust
let logger = Logger::new().with_feature("chapter-service");

logger.track_action("save_chapter", || {
    // 执行操作
    save_chapter_to_db()
});
```

#### 错误记录
```rust
use std::error::Error;

let logger = Logger::new();
let error = some_operation();

if let Err(e) = error {
    logger.error_with_cause("Failed to save chapter", &e);
}
```

## 日志格式

### 前端日志格式
```
[时间戳][级别][请求ID] 消息 | Context: {上下文JSON} | Error: 错误消息
```

示例：
```
[2024-01-01T12:00:00.000Z][INFO][1234567890-abc123] Creating project | Context: {"feature":"project-service","action":"createProject","projectId":"123"}
```

### 后端日志格式
```
[时间戳][级别][请求ID][功能][操作] 消息
```

示例：
```
[1704110400000][INFO][1234567890-abc123][project-service][create_project] Action started | Params: CreateProjectRequest { name: "Test" }
[1704110400100][INFO][1234567890-abc123][project-service][create_project] Action completed | Duration: 100ms
```

## 调用链路追踪

每个请求都会生成唯一的`requestId`，贯穿整个调用链：

```
前端: [ID-abc123] createProject开始
  ↓ 调用Tauri命令
后端: [ID-abc123] create_project命令开始
  ↓ 数据库操作
后端: [ID-abc123] create_project命令完成
  ↓ 返回结果
前端: [ID-abc123] createProject完成
```

## 性能监控

使用`trackAction`方法自动记录操作耗时：

```typescript
// 前端
const track = logger.trackAction('saveChapter');
// ... 执行操作
track(); // 输出：Action completed | Duration: 125.45ms
```

```rust
// 后端
logger.track_action("save_chapter", || {
    // ... 执行操作
});
// 输出：Action completed | Duration: 100ms
```

## 错误追踪

### 前端错误
```typescript
try {
  await someAsyncOperation();
} catch (error) {
  logger.error('Operation failed', error instanceof Error ? error : new Error(String(error)), {
    feature: 'my-feature',
    action: 'my-action',
    additionalContext: 'value'
  });
  // 自动记录错误堆栈
}
```

### 后端错误
```rust
match some_operation() {
    Ok(result) => {
        logger.info("Operation succeeded");
    }
    Err(e) => {
        logger.error_with_cause("Operation failed", &e);
        // e会自动显示错误原因
    }
}
```

## 日志输出

### 开发环境
- 所有级别的日志都输出到控制台
- 日志格式带颜色，易于阅读
- 包含完整上下文信息

### 生产环境
- 可配置日志级别
- 可将日志发送到远程日志服务
- 可记录到文件

## 测试日志

### 验证日志功能
```typescript
import { logger } from './utils/logger';

describe('Logger', () => {
  it('should log messages correctly', () => {
    const spy = vi.spyOn(console, 'info');
    logger.info('Test message');
    expect(spy).toHaveBeenCalled();
  });
});
```

## 日志最佳实践

1. **使用合适的日志级别**
   - DEBUG: 详细的调试信息，只在开发时使用
   - INFO: 重要的业务操作和状态变化
   - WARN: 潜在问题，但不影响功能
   - ERROR: 错误，需要立即处理

2. **包含有意义的上下文**
   ```typescript
   // 好
   logger.info('Project created', { 
     feature: 'project-service',
     projectId: '123',
     name: 'Test Project'
   });

   // 不好
   logger.info('Created');
   ```

3. **使用动作追踪**
   ```typescript
   // 自动记录性能
   const track = logger.trackAction('complexOperation');
   // ... 操作
   track();
   ```

4. **记录错误详情**
   ```typescript
   logger.error('Failed to save chapter', error, {
     action: 'saveChapter',
     chapterId: '123',
     retryCount: 3
   });
   ```

5. **避免过度日志**
   - 不要在循环中记录大量日志
   - 使用DEBUG级别记录详细信息
   - 生产环境只记录INFO及以上级别

## 调试技巧

1. **按请求ID过滤**
   ```bash
   # 查看特定请求的所有日志
   grep "1234567890-abc123" logs.txt
   ```

2. **按功能过滤**
   ```bash
   # 查看项目服务的所有日志
   grep "project-service" logs.txt
   ```

3. **按级别过滤**
   ```bash
   # 只查看错误
   grep "ERROR" logs.txt
   ```

## 未来改进

- [ ] 添加日志文件输出
- [ ] 集成远程日志服务（如Sentry、LogRocket）
- [ ] 实现日志轮转和归档
- [ ] 添加日志搜索和分析工具
- [ ] 实现性能监控仪表板
