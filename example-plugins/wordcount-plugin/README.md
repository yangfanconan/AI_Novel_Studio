# 字数统计工具插件

一个用于 AI Novel Studio 的字数统计插件。

## 功能

- 统计总字符数
- 统计不含空格字符数
- 统计词数
- 统计段落数
- 统计句子数

## 安装

1. 将此文件夹复制到应用的插件目录中
2. 打开插件管理器
3. 点击"安装插件"
4. 输入此文件夹的路径
5. 激活插件

## 使用方法

在浏览器控制台中运行：

```javascript
const text = "这是一个示例文本。\\n\\n它包含两个段落。";
const result = window.wordcountPlugin.count(text);
console.log(result);
```

输出：
```javascript
{
  words: 14,
  chars: 32,
  charsNoSpaces: 30,
  paragraphs: 2,
  sentences: 2
}
```

## API

### count(text: string): WordCountResult

统计给定文本的字数信息。

**参数：**
- `text` - 要统计的文本

**返回：**
```typescript
interface WordCountResult {
  words: number;           // 词数
  chars: number;           // 字符数
  charsNoSpaces: number;    // 不含空格的字符数
  paragraphs: number;       // 段落数
  sentences: number;        // 句子数
}
```

## 版本

v1.0.0 - 初始版本
