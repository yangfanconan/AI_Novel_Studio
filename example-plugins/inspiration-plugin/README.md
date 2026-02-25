# 灵感生成器插件

一个用于 AI Novel Studio 的灵感生成插件，提供随机的写作灵感和创意。

## 功能

- **通用灵感**：生成随机的创意点子
- **角色灵感**：生成角色设定灵感
- **情节灵感**：生成情节发展灵感
- **批量生成**：一次生成多个灵感
- **查看全部**：获取所有可用的灵感列表

## 安装

1. 将此文件夹复制到应用的插件目录中
2. 打开插件管理器
3. 点击"安装插件"
4. 输入此文件夹的路径
5. 激活插件

## 使用方法

### 生成通用灵感

```javascript
const inspiration = window.inspirationPlugin.generate();
console.log(inspiration);
// 示例输出: "一个隐藏在图书馆里的古老秘密"
```

### 生成角色灵感

```javascript
const characterInspiration = window.inspirationPlugin.generate('character');
console.log(characterInspiration);
// 示例输出: "一个沉默寡言但内心丰富的图书管理员"
```

### 生成情节灵感

```javascript
const plotInspiration = window.inspirationPlugin.generate('plot');
console.log(plotInspiration);
// 示例输出: "主角发现了一个能改变过去的机会，但代价是失去现在最珍贵的人"
```

### 批量生成灵感

```javascript
const inspirations = window.inspirationPlugin.generateMultiple('plot', 3);
console.log(inspirations);
// 示例输出: ["灵感1", "灵感2", "灵感3"]
```

### 获取所有灵感

```javascript
const allInspirations = window.inspirationPlugin.getAll('character');
console.log(allInspirations);
// 返回所有角色灵感的数组
```

## API

### generate(type?: string): string

生成一个随机灵感。

**参数：**
- `type` (可选) - 灵感类型：
  - `'generic'` (默认) - 通用灵感
  - `'character'` - 角色灵感
  - `'plot'` - 情节灵感

**返回：**
- `string` - 随机灵感文本

### generateMultiple(type?: string, count?: number): string[]

生成多个随机灵感。

**参数：**
- `type` (可选) - 灵感类型，默认 `'generic'`
- `count` (可选) - 生成数量，默认 `5`

**返回：**
- `string[]` - 灵感数组

### getAll(type?: string): string[]

获取所有可用的灵感列表。

**参数：**
- `type` (可选) - 灵感类型，默认 `'generic'`

**返回：**
- `string[]` - 灵感数组

## 示例

在浏览器控制台中运行：

```javascript
// 获取3个情节灵感
const plotIdeas = window.inspirationPlugin.generateMultiple('plot', 3);
console.log('情节灵感:', plotIdeas);

// 获取所有角色灵感
const characters = window.inspirationPlugin.getAll('character');
console.log(`共有 ${characters.length} 个角色灵感`);
```

## 版本

v1.0.0 - 初始版本
