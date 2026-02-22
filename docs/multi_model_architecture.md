# AI小说创作工作室 - 多模型接入架构设计

## 一、架构概述

### 1.1 设计目标
- **统一接口**：提供统一的API接口，屏蔽不同模型差异
- **灵活切换**：支持运行时动态切换模型
- **成本优化**：根据任务复杂度自动选择合适的模型
- **负载均衡**：支持多模型并行、故障转移
- **本地优先**：优先支持本地模型，保护隐私

### 1.2 整体架构

```
┌─────────────────────────────────────────────────────────────────┐
│                     Application Layer                            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Unified AI Interface                         │  │
│  │  generate() | chat() | embed() | stream()                │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ↓ ↑
┌─────────────────────────────────────────────────────────────────┐
│                     AI Engine Core                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Model Router │  │Prompt Manager│  │ Cost Tracker │         │
│  │ 模型路由      │  │ 提示词管理   │  │ 成本追踪      │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Cache Layer  │  │Rate Limiter  │  │ Retry Logic  │         │
│  │ 缓存层        │  │ 速率限制     │  │ 重试机制      │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                              ↓ ↑
┌─────────────────────────────────────────────────────────────────┐
│                     Model Adapter Layer                          │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │ OpenAI  │ │ Claude  │ │ Gemini  │ │ 文心    │ │ 通义    │  │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │
│  │ Ollama  │ │ vLLM    │ │LocalAI  │ │智谱GLM  │ │ Custom  │  │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ↓ ↑
┌─────────────────────────────────────────────────────────────────┐
│                     Transport Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ HTTP/HTTPS   │  │ WebSocket    │  │ gRPC         │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

---

## 二、统一模型接口

### 2.1 核心接口定义

```typescript
// 统一模型接口
interface UnifiedModelInterface {
  // 模型标识
  readonly id: string;
  readonly name: string;
  readonly provider: string;
  readonly type: ModelType;
  
  // 能力声明
  readonly capabilities: ModelCapabilities;
  
  // 文本生成
  generate(request: GenerateRequest): Promise<GenerateResponse>;
  
  // 流式生成
  streamGenerate(
    request: GenerateRequest,
    onChunk: (chunk: StreamChunk) => void,
    onComplete?: (response: GenerateResponse) => void,
    onError?: (error: Error) => void
  ): Promise<void>;
  
  // 对话
  chat(request: ChatRequest): Promise<ChatResponse>;
  
  // 流式对话
  streamChat(
    request: ChatRequest,
    onChunk: (chunk: ChatChunk) => void,
    onComplete?: (response: ChatResponse) => void,
    onError?: (error: Error) => void
  ): Promise<void>;
  
  // 向量嵌入
  embed(request: EmbedRequest): Promise<EmbedResponse>;
  
  // Token计数
  countTokens(text: string): Promise<number>;
  
  // 健康检查
  healthCheck(): Promise<boolean>;
}

// 模型类型
enum ModelType {
  CHAT = 'chat',
  COMPLETION = 'completion',
  EMBEDDING = 'embedding',
  IMAGE = 'image',
  AUDIO = 'audio',
  MULTIMODAL = 'multimodal'
}

// 模型能力
interface ModelCapabilities {
  maxTokens: number;
  supportsStreaming: boolean;
  supportsFunctionCalling: boolean;
  supportsVision: boolean;
  supportsJSON: boolean;
  supportedLanguages: string[];
  contextWindow: number;
}
```

### 2.2 请求/响应结构

```typescript
// 生成请求
interface GenerateRequest {
  prompt: string;
  systemPrompt?: string;
  
  // 模型参数
  temperature?: number;
  maxTokens?: number;
  topP?: number;
  topK?: number;
  frequencyPenalty?: number;
  presencePenalty?: number;
  stopSequences?: string[];
  
  // 高级选项
  responseFormat?: ResponseFormat;
  seed?: number;
  
  // 元数据
  metadata?: Record<string, any>;
  timeout?: number;
}

// 生成响应
interface GenerateResponse {
  id: string;
  text: string;
  
  // 使用统计
  usage: TokenUsage;
  
  // 模型信息
  model: string;
  provider: string;
  
  // 完成原因
  finishReason: FinishReason;
  
  // 元数据
  latency: number;
  cost?: number;
  metadata?: Record<string, any>;
}

// Token使用统计
interface TokenUsage {
  promptTokens: number;
  completionTokens: number;
  totalTokens: number;
}

// 完成原因
enum FinishReason {
  STOP = 'stop',
  LENGTH = 'length',
  CONTENT_FILTER = 'content_filter',
  ERROR = 'error'
}

// 对话请求
interface ChatRequest {
  messages: ChatMessage[];
  systemPrompt?: string;
  
  // 模型参数
  temperature?: number;
  maxTokens?: number;
  topP?: number;
  stopSequences?: string[];
  
  // 工具调用
  tools?: Tool[];
  toolChoice?: ToolChoice;
  
  // 元数据
  metadata?: Record<string, any>;
  timeout?: number;
}

// 对话消息
interface ChatMessage {
  role: 'system' | 'user' | 'assistant' | 'tool';
  content: string | MessageContent[];
  name?: string;
  toolCallId?: string;
  toolCalls?: ToolCall[];
}

// 多模态内容
interface MessageContent {
  type: 'text' | 'image' | 'image_url';
  text?: string;
  imageUrl?: { url: string };
}

// 对话响应
interface ChatResponse {
  id: string;
  message: ChatMessage;
  
  // 使用统计
  usage: TokenUsage;
  
  // 模型信息
  model: string;
  provider: string;
  
  // 完成原因
  finishReason: FinishReason;
  
  // 元数据
  latency: number;
  cost?: number;
}

// 流式响应块
interface StreamChunk {
  delta: string;
  accumulated: string;
  done: boolean;
  usage?: Partial<TokenUsage>;
}

// 嵌入请求
interface EmbedRequest {
  input: string | string[];
  model?: string;
}

// 嵌入响应
interface EmbedResponse {
  embeddings: number[][];
  usage: TokenUsage;
  model: string;
}
```

---

## 三、模型适配器

### 3.1 适配器接口

```typescript
// 模型适配器基础接口
interface ModelAdapter extends UnifiedModelInterface {
  // 配置
  configure(config: AdapterConfig): void;
  
  // 获取配置
  getConfig(): AdapterConfig;
  
  // 验证配置
  validateConfig(): Promise<boolean>;
}

// 适配器配置
interface AdapterConfig {
  apiKey?: string;
  baseUrl?: string;
  timeout?: number;
  maxRetries?: number;
  defaultModel?: string;
  customHeaders?: Record<string, string>;
}
```

### 3.2 OpenAI适配器

```typescript
class OpenAIAdapter implements ModelAdapter {
  readonly id = 'openai';
  readonly name = 'OpenAI';
  readonly provider = 'openai';
  readonly type = ModelType.CHAT;
  
  readonly capabilities: ModelCapabilities = {
    maxTokens: 128000,
    supportsStreaming: true,
    supportsFunctionCalling: true,
    supportsVision: true,
    supportsJSON: true,
    supportedLanguages: ['en', 'zh', 'ja', 'ko', 'multi'],
    contextWindow: 128000
  };
  
  private client: OpenAI;
  private config: OpenAIConfig;
  
  constructor(config: OpenAIConfig) {
    this.config = config;
    this.client = new OpenAI({
      apiKey: config.apiKey,
      baseURL: config.baseUrl || 'https://api.openai.com/v1',
      timeout: config.timeout || 60000,
      dangerouslyAllowBrowser: false
    });
  }
  
  async generate(request: GenerateRequest): Promise<GenerateResponse> {
    const startTime = Date.now();
    
    const response = await this.client.chat.completions.create({
      model: this.config.defaultModel || 'gpt-4-turbo-preview',
      messages: [
        ...(request.systemPrompt ? [{ role: 'system', content: request.systemPrompt }] : []),
        { role: 'user', content: request.prompt }
      ],
      temperature: request.temperature ?? 0.7,
      max_tokens: request.maxTokens ?? 2000,
      top_p: request.topP,
      frequency_penalty: request.frequencyPenalty,
      presence_penalty: request.presencePenalty,
      stop: request.stopSequences,
      response_format: request.responseFormat,
      seed: request.seed,
      stream: false
    });
    
    const choice = response.choices[0];
    
    return {
      id: response.id,
      text: choice.message.content || '',
      usage: {
        promptTokens: response.usage?.prompt_tokens || 0,
        completionTokens: response.usage?.completion_tokens || 0,
        totalTokens: response.usage?.total_tokens || 0
      },
      model: response.model,
      provider: this.provider,
      finishReason: this.mapFinishReason(choice.finish_reason),
      latency: Date.now() - startTime,
      cost: this.calculateCost(response.usage)
    };
  }
  
  async streamGenerate(
    request: GenerateRequest,
    onChunk: (chunk: StreamChunk) => void,
    onComplete?: (response: GenerateResponse) => void,
    onError?: (error: Error) => void
  ): Promise<void> {
    let accumulated = '';
    let usage: TokenUsage | undefined;
    
    try {
      const stream = await this.client.chat.completions.create({
        model: this.config.defaultModel || 'gpt-4-turbo-preview',
        messages: [
          ...(request.systemPrompt ? [{ role: 'system', content: request.systemPrompt }] : []),
          { role: 'user', content: request.prompt }
        ],
        temperature: request.temperature ?? 0.7,
        max_tokens: request.maxTokens ?? 2000,
        stream: true
      });
      
      for await (const chunk of stream) {
        const delta = chunk.choices[0]?.delta?.content || '';
        accumulated += delta;
        
        onChunk({
          delta,
          accumulated,
          done: false
        });
        
        if (chunk.usage) {
          usage = {
            promptTokens: chunk.usage.prompt_tokens,
            completionTokens: chunk.usage.completion_tokens,
            totalTokens: chunk.usage.total_tokens
          };
        }
      }
      
      onChunk({
        delta: '',
        accumulated,
        done: true,
        usage
      });
      
      if (onComplete) {
        onComplete({
          id: `stream-${Date.now()}`,
          text: accumulated,
          usage: usage || { promptTokens: 0, completionTokens: 0, totalTokens: 0 },
          model: this.config.defaultModel || 'gpt-4-turbo-preview',
          provider: this.provider,
          finishReason: FinishReason.STOP,
          latency: 0
        });
      }
    } catch (error) {
      if (onError) {
        onError(error as Error);
      }
      throw error;
    }
  }
  
  async chat(request: ChatRequest): Promise<ChatResponse> {
    const startTime = Date.now();
    
    const messages = this.convertMessages(request.messages, request.systemPrompt);
    
    const response = await this.client.chat.completions.create({
      model: this.config.defaultModel || 'gpt-4-turbo-preview',
      messages,
      temperature: request.temperature ?? 0.7,
      max_tokens: request.maxTokens ?? 2000,
      tools: request.tools,
      tool_choice: request.toolChoice,
      stream: false
    });
    
    const choice = response.choices[0];
    
    return {
      id: response.id,
      message: {
        role: 'assistant',
        content: choice.message.content || '',
        toolCalls: choice.message.tool_calls?.map(tc => ({
          id: tc.id,
          type: 'function',
          function: {
            name: tc.function.name,
            arguments: tc.function.arguments
          }
        }))
      },
      usage: {
        promptTokens: response.usage?.prompt_tokens || 0,
        completionTokens: response.usage?.completion_tokens || 0,
        totalTokens: response.usage?.total_tokens || 0
      },
      model: response.model,
      provider: this.provider,
      finishReason: this.mapFinishReason(choice.finish_reason),
      latency: Date.now() - startTime,
      cost: this.calculateCost(response.usage)
    };
  }
  
  async embed(request: EmbedRequest): Promise<EmbedResponse> {
    const input = Array.isArray(request.input) ? request.input : [request.input];
    
    const response = await this.client.embeddings.create({
      model: request.model || 'text-embedding-3-small',
      input
    });
    
    return {
      embeddings: response.data.map(d => d.embedding),
      usage: {
        promptTokens: response.usage.prompt_tokens,
        completionTokens: 0,
        totalTokens: response.usage.total_tokens
      },
      model: response.model
    };
  }
  
  async countTokens(text: string): Promise<number> {
    // 使用tiktoken库
    const encoding = await this.getEncoding();
    return encoding.encode(text).length;
  }
  
  async healthCheck(): Promise<boolean> {
    try {
      await this.client.models.list();
      return true;
    } catch {
      return false;
    }
  }
  
  private calculateCost(usage: any): number {
    // OpenAI定价 (示例)
    const model = this.config.defaultModel || 'gpt-4-turbo-preview';
    const pricing: Record<string, { input: number; output: number }> = {
      'gpt-4-turbo-preview': { input: 0.01 / 1000, output: 0.03 / 1000 },
      'gpt-4': { input: 0.03 / 1000, output: 0.06 / 1000 },
      'gpt-3.5-turbo': { input: 0.0005 / 1000, output: 0.0015 / 1000 }
    };
    
    const price = pricing[model] || pricing['gpt-3.5-turbo'];
    return (usage.prompt_tokens * price.input) + (usage.completion_tokens * price.output);
  }
  
  private mapFinishReason(reason: string): FinishReason {
    const mapping: Record<string, FinishReason> = {
      'stop': FinishReason.STOP,
      'length': FinishReason.LENGTH,
      'content_filter': FinishReason.CONTENT_FILTER
    };
    return mapping[reason] || FinishReason.STOP;
  }
  
  private convertMessages(
    messages: ChatMessage[],
    systemPrompt?: string
  ): OpenAI.ChatCompletionMessageParam[] {
    const result: OpenAI.ChatCompletionMessageParam[] = [];
    
    if (systemPrompt) {
      result.push({ role: 'system', content: systemPrompt });
    }
    
    for (const msg of messages) {
      result.push({
        role: msg.role as any,
        content: typeof msg.content === 'string' 
          ? msg.content 
          : msg.content.map(c => ({ type: c.type, text: c.text, image_url: c.imageUrl })),
        name: msg.name
      });
    }
    
    return result;
  }
}
```

### 3.3 Ollama适配器

```typescript
class OllamaAdapter implements ModelAdapter {
  readonly id = 'ollama';
  readonly name = 'Ollama';
  readonly provider = 'ollama';
  readonly type = ModelType.CHAT;
  
  readonly capabilities: ModelCapabilities = {
    maxTokens: 32000,
    supportsStreaming: true,
    supportsFunctionCalling: false,
    supportsVision: true,
    supportsJSON: false,
    supportedLanguages: ['multi'],
    contextWindow: 32000
  };
  
  private config: OllamaConfig;
  private baseUrl: string;
  
  constructor(config: OllamaConfig) {
    this.config = config;
    this.baseUrl = config.baseUrl || 'http://localhost:11434';
  }
  
  async generate(request: GenerateRequest): Promise<GenerateResponse> {
    const startTime = Date.now();
    
    const response = await fetch(`${this.baseUrl}/api/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model: this.config.defaultModel || 'llama2',
        prompt: request.prompt,
        system: request.systemPrompt,
        stream: false,
        options: {
          temperature: request.temperature,
          num_predict: request.maxTokens,
          top_p: request.topP,
          top_k: request.topK,
          stop: request.stopSequences
        }
      })
    });
    
    const data = await response.json();
    
    return {
      id: `ollama-${Date.now()}`,
      text: data.response,
      usage: {
        promptTokens: data.prompt_eval_count || 0,
        completionTokens: data.eval_count || 0,
        totalTokens: (data.prompt_eval_count || 0) + (data.eval_count || 0)
      },
      model: data.model,
      provider: this.provider,
      finishReason: data.done ? FinishReason.STOP : FinishReason.LENGTH,
      latency: Date.now() - startTime,
      cost: 0 // 本地模型无成本
    };
  }
  
  async streamGenerate(
    request: GenerateRequest,
    onChunk: (chunk: StreamChunk) => void,
    onComplete?: (response: GenerateResponse) => void,
    onError?: (error: Error) => void
  ): Promise<void> {
    let accumulated = '';
    let totalTokens = 0;
    
    try {
      const response = await fetch(`${this.baseUrl}/api/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: this.config.defaultModel || 'llama2',
          prompt: request.prompt,
          system: request.systemPrompt,
          stream: true,
          options: {
            temperature: request.temperature,
            num_predict: request.maxTokens
          }
        })
      });
      
      const reader = response.body?.getReader();
      if (!reader) throw new Error('无法获取响应流');
      
      const decoder = new TextDecoder();
      
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        
        const lines = decoder.decode(value).split('\n').filter(Boolean);
        
        for (const line of lines) {
          const data = JSON.parse(line);
          accumulated += data.response || '';
          totalTokens++;
          
          onChunk({
            delta: data.response || '',
            accumulated,
            done: data.done
          });
        }
      }
      
      if (onComplete) {
        onComplete({
          id: `ollama-stream-${Date.now()}`,
          text: accumulated,
          usage: { promptTokens: 0, completionTokens: totalTokens, totalTokens },
          model: this.config.defaultModel || 'llama2',
          provider: this.provider,
          finishReason: FinishReason.STOP,
          latency: 0,
          cost: 0
        });
      }
    } catch (error) {
      if (onError) onError(error as Error);
      throw error;
    }
  }
  
  async chat(request: ChatRequest): Promise<ChatResponse> {
    const startTime = Date.now();
    
    const response = await fetch(`${this.baseUrl}/api/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model: this.config.defaultModel || 'llama2',
        messages: request.messages.map(m => ({
          role: m.role,
          content: typeof m.content === 'string' ? m.content : m.content.map(c => c.text).join('')
        })),
        stream: false,
        options: {
          temperature: request.temperature,
          num_predict: request.maxTokens
        }
      })
    });
    
    const data = await response.json();
    
    return {
      id: `ollama-chat-${Date.now()}`,
      message: {
        role: 'assistant',
        content: data.message.content
      },
      usage: {
        promptTokens: data.prompt_eval_count || 0,
        completionTokens: data.eval_count || 0,
        totalTokens: (data.prompt_eval_count || 0) + (data.eval_count || 0)
      },
      model: data.model,
      provider: this.provider,
      finishReason: data.done ? FinishReason.STOP : FinishReason.LENGTH,
      latency: Date.now() - startTime,
      cost: 0
    };
  }
  
  async embed(request: EmbedRequest): Promise<EmbedResponse> {
    const input = Array.isArray(request.input) ? request.input : [request.input];
    
    const response = await fetch(`${this.baseUrl}/api/embeddings`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model: request.model || 'nomic-embed-text',
        input
      })
    });
    
    const data = await response.json();
    
    return {
      embeddings: data.embeddings || [data.embedding],
      usage: {
        promptTokens: input.length * 10,
        completionTokens: 0,
        totalTokens: input.length * 10
      },
      model: request.model || 'nomic-embed-text'
    };
  }
  
  async countTokens(text: string): Promise<number> {
    // Ollama没有直接的token计数API，使用估算
    return Math.ceil(text.length / 4);
  }
  
  async healthCheck(): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/api/tags`);
      return response.ok;
    } catch {
      return false;
    }
  }
  
  async listModels(): Promise<string[]> {
    const response = await fetch(`${this.baseUrl}/api/tags`);
    const data = await response.json();
    return data.models?.map((m: any) => m.name) || [];
  }
  
  async pullModel(modelName: string): Promise<void> {
    await fetch(`${this.baseUrl}/api/pull`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name: modelName })
    });
  }
}
```

### 3.4 国内模型适配器

```typescript
// 文心一言适配器
class WenxinAdapter implements ModelAdapter {
  readonly id = 'wenxin';
  readonly name = '文心一言';
  readonly provider = 'baidu';
  readonly type = ModelType.CHAT;
  
  private config: WenxinConfig;
  private accessToken: string | null = null;
  private tokenExpireTime: number = 0;
  
  constructor(config: WenxinConfig) {
    this.config = config;
  }
  
  private async getAccessToken(): Promise<string> {
    if (this.accessToken && Date.now() < this.tokenExpireTime) {
      return this.accessToken;
    }
    
    const response = await fetch(
      `https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id=${this.config.apiKey}&client_secret=${this.config.secretKey}`,
      { method: 'POST' }
    );
    
    const data = await response.json();
    this.accessToken = data.access_token;
    this.tokenExpireTime = Date.now() + (data.expires_in - 300) * 1000;
    
    return this.accessToken!;
  }
  
  async chat(request: ChatRequest): Promise<ChatResponse> {
    const startTime = Date.now();
    const token = await this.getAccessToken();
    
    const response = await fetch(
      `https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions?access_token=${token}`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: this.config.defaultModel || 'ernie-bot-4',
          messages: request.messages.map(m => ({
            role: m.role,
            content: typeof m.content === 'string' ? m.content : m.content[0].text
          })),
          temperature: request.temperature,
          max_output_tokens: request.maxTokens
        })
      }
    );
    
    const data = await response.json();
    
    return {
      id: data.id || `wenxin-${Date.now()}`,
      message: {
        role: 'assistant',
        content: data.result
      },
      usage: {
        promptTokens: data.usage?.prompt_tokens || 0,
        completionTokens: data.usage?.completion_tokens || 0,
        totalTokens: data.usage?.total_tokens || 0
      },
      model: this.config.defaultModel || 'ernie-bot-4',
      provider: this.provider,
      finishReason: this.mapFinishReason(data.finish_reason),
      latency: Date.now() - startTime,
      cost: this.calculateCost(data.usage)
    };
  }
  
  // ... 其他方法实现
}

// 通义千问适配器
class QwenAdapter implements ModelAdapter {
  readonly id = 'qwen';
  readonly name = '通义千问';
  readonly provider = 'alibaba';
  readonly type = ModelType.CHAT;
  
  private config: QwenConfig;
  
  constructor(config: QwenConfig) {
    this.config = config;
  }
  
  async chat(request: ChatRequest): Promise<ChatResponse> {
    const startTime = Date.now();
    
    const response = await fetch(
      'https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation',
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.config.apiKey}`
        },
        body: JSON.stringify({
          model: this.config.defaultModel || 'qwen-max',
          input: {
            messages: request.messages.map(m => ({
              role: m.role,
              content: typeof m.content === 'string' ? m.content : m.content[0].text
            }))
          },
          parameters: {
            temperature: request.temperature,
            max_tokens: request.maxTokens,
            result_format: 'message'
          }
        })
      }
    );
    
    const data = await response.json();
    
    return {
      id: data.request_id || `qwen-${Date.now()}`,
      message: {
        role: 'assistant',
        content: data.output.choices[0].message.content
      },
      usage: {
        promptTokens: data.usage.input_tokens,
        completionTokens: data.usage.output_tokens,
        totalTokens: data.usage.total_tokens
      },
      model: this.config.defaultModel || 'qwen-max',
      provider: this.provider,
      finishReason: this.mapFinishReason(data.output.choices[0].finish_reason),
      latency: Date.now() - startTime,
      cost: this.calculateCost(data.usage)
    };
  }
  
  // ... 其他方法实现
}

// 智谱GLM适配器
class GLMAdapter implements ModelAdapter {
  readonly id = 'glm';
  readonly name = '智谱GLM';
  readonly provider = 'zhipu';
  readonly type = ModelType.CHAT;
  
  private config: GLMConfig;
  
  constructor(config: GLMConfig) {
    this.config = config;
  }
  
  async chat(request: ChatRequest): Promise<ChatResponse> {
    const startTime = Date.now();
    
    const response = await fetch(
      'https://open.bigmodel.cn/api/paas/v4/chat/completions',
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${this.config.apiKey}`
        },
        body: JSON.stringify({
          model: this.config.defaultModel || 'glm-4',
          messages: request.messages.map(m => ({
            role: m.role,
            content: typeof m.content === 'string' ? m.content : m.content[0].text
          })),
          temperature: request.temperature,
          max_tokens: request.maxTokens,
          stream: false
        })
      }
    );
    
    const data = await response.json();
    
    return {
      id: data.id,
      message: data.choices[0].message,
      usage: {
        promptTokens: data.usage.prompt_tokens,
        completionTokens: data.usage.completion_tokens,
        totalTokens: data.usage.total_tokens
      },
      model: data.model,
      provider: this.provider,
      finishReason: this.mapFinishReason(data.choices[0].finish_reason),
      latency: Date.now() - startTime,
      cost: this.calculateCost(data.usage)
    };
  }
  
  // ... 其他方法实现
}
```

---

## 四、模型路由器

### 4.1 智能路由策略

```typescript
// 模型路由器
class ModelRouter {
  private adapters: Map<string, ModelAdapter> = new Map();
  private strategies: RoutingStrategy[] = [];
  private config: RouterConfig;
  
  constructor(config: RouterConfig) {
    this.config = config;
    this.initializeStrategies();
  }
  
  registerAdapter(adapter: ModelAdapter): void {
    this.adapters.set(adapter.id, adapter);
  }
  
  // 智能选择模型
  async selectModel(request: AIRequest): Promise<ModelAdapter> {
    for (const strategy of this.strategies) {
      const adapterId = await strategy.select(request, this.adapters);
      if (adapterId && this.adapters.has(adapterId)) {
        return this.adapters.get(adapterId)!;
      }
    }
    
    // 默认模型
    return this.adapters.get(this.config.defaultAdapter)!;
  }
  
  private initializeStrategies(): void {
    this.strategies = [
      new ExplicitModelStrategy(),
      new TaskBasedStrategy(),
      new CostOptimizationStrategy(),
      new PerformanceStrategy(),
      new FallbackStrategy()
    ];
  }
}

// 路由策略接口
interface RoutingStrategy {
  name: string;
  priority: number;
  select(request: AIRequest, adapters: Map<string, ModelAdapter>): Promise<string | null>;
}

// 显式指定模型策略
class ExplicitModelStrategy implements RoutingStrategy {
  name = 'explicit';
  priority = 100;
  
  async select(request: AIRequest): Promise<string | null> {
    return request.modelId || null;
  }
}

// 基于任务的策略
class TaskBasedStrategy implements RoutingStrategy {
  name = 'task-based';
  priority = 80;
  
  private taskModelMapping: Record<string, string> = {
    'creative_writing': 'gpt-4',
    'plot_generation': 'claude-3',
    'dialogue': 'gpt-3.5-turbo',
    'translation': 'qwen-max',
    'analysis': 'glm-4',
    'embedding': 'text-embedding-3-small'
  };
  
  async select(request: AIRequest): Promise<string | null> {
    const task = request.metadata?.task;
    return this.taskModelMapping[task] || null;
  }
}

// 成本优化策略
class CostOptimizationStrategy implements RoutingStrategy {
  name = 'cost-optimization';
  priority = 60;
  
  async select(
    request: AIRequest,
    adapters: Map<string, ModelAdapter>
  ): Promise<string | null> {
    // 根据任务复杂度选择成本合适的模型
    const complexity = this.estimateComplexity(request);
    
    if (complexity === 'low') {
      return 'ollama'; // 本地免费模型
    } else if (complexity === 'medium') {
      return 'gpt-3.5-turbo';
    } else {
      return 'gpt-4';
    }
  }
  
  private estimateComplexity(request: AIRequest): 'low' | 'medium' | 'high' {
    const promptLength = request.prompt?.length || 0;
    
    if (promptLength < 500) return 'low';
    if (promptLength < 2000) return 'medium';
    return 'high';
  }
}

// 性能优先策略
class PerformanceStrategy implements RoutingStrategy {
  name = 'performance';
  priority = 50;
  
  async select(): Promise<string | null> {
    // 选择响应最快的模型
    return 'gpt-3.5-turbo';
  }
}

// 降级策略
class FallbackStrategy implements RoutingStrategy {
  name = 'fallback';
  priority = 0;
  
  async select(): Promise<string | null> {
    return 'ollama'; // 最后降级到本地模型
  }
}
```

### 4.2 负载均衡

```typescript
// 负载均衡器
class LoadBalancer {
  private healthChecker: HealthChecker;
  private metrics: Map<string, AdapterMetrics> = new Map();
  
  async selectAdapter(
    candidates: string[],
    strategy: BalanceStrategy
  ): Promise<string> {
    switch (strategy) {
      case 'round-robin':
        return this.roundRobin(candidates);
      case 'least-connections':
        return this.leastConnections(candidates);
      case 'weighted':
        return this.weighted(candidates);
      case 'random':
        return this.random(candidates);
      default:
        return candidates[0];
    }
  }
  
  private roundRobin(candidates: string[]): string {
    // 轮询实现
  }
  
  private leastConnections(candidates: string[]): string {
    // 最少连接实现
  }
  
  private weighted(candidates: string[]): string {
    // 加权选择实现
  }
  
  private random(candidates: string[]): string {
    return candidates[Math.floor(Math.random() * candidates.length)];
  }
}

type BalanceStrategy = 'round-robin' | 'least-connections' | 'weighted' | 'random';
```

---

## 五、Prompt管理系统

### 5.1 Prompt模板

```typescript
// Prompt模板管理
class PromptManager {
  private templates: Map<string, PromptTemplate> = new Map();
  private versionControl: PromptVersionControl;
  
  // 注册模板
  registerTemplate(template: PromptTemplate): void {
    this.templates.set(template.id, template);
  }
  
  // 渲染模板
  render(templateId: string, variables: Record<string, any>): string {
    const template = this.templates.get(templateId);
    if (!template) {
      throw new Error(`模板未找到: ${templateId}`);
    }
    
    return this.interpolate(template.template, variables);
  }
  
  private interpolate(template: string, variables: Record<string, any>): string {
    return template.replace(/\{(\w+)\}/g, (match, key) => {
      return variables[key] !== undefined ? String(variables[key]) : match;
    });
  }
}

// Prompt模板定义
interface PromptTemplate {
  id: string;
  name: string;
  category: PromptCategory;
  description: string;
  template: string;
  variables: PromptVariable[];
  examples?: PromptExample[];
  metadata?: Record<string, any>;
}

enum PromptCategory {
  CREATIVE_WRITING = 'creative_writing',
  ANALYSIS = 'analysis',
  TRANSLATION = 'translation',
  EDITING = 'editing',
  STORYBOARD = 'storyboard',
  SCRIPT = 'script',
  COMIC = 'comic',
  ILLUSTRATION = 'illustration'
}

interface PromptVariable {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'array' | 'object';
  required: boolean;
  default?: any;
  description: string;
}

// 预定义模板
const NOVEL_CONTINUATION_TEMPLATE: PromptTemplate = {
  id: 'novel_continuation',
  name: '小说续写',
  category: PromptCategory.CREATIVE_WRITING,
  description: '根据前文内容智能续写小说',
  template: `你是一位专业的小说创作助手。请根据以下信息继续创作：

【前文内容】
{previous_content}

【角色信息】
{character_info}

【世界观设定】
{world_setting}

【情节走向】
{plot_direction}

【创作要求】
- 风格：{style}
- 字数：约{word_count}字
- 保持角色一致性
- 推进情节发展
- 注重细节描写

请继续创作：`,
  variables: [
    { name: 'previous_content', type: 'string', required: true, description: '前文内容' },
    { name: 'character_info', type: 'string', required: false, default: '', description: '角色信息' },
    { name: 'world_setting', type: 'string', required: false, default: '', description: '世界观设定' },
    { name: 'plot_direction', type: 'string', required: false, default: '自然发展', description: '情节走向提示' },
    { name: 'style', type: 'string', required: false, default: '自然流畅', description: '写作风格' },
    { name: 'word_count', type: 'number', required: false, default: 500, description: '目标字数' }
  ]
};

const STORYBOARD_GENERATION_TEMPLATE: PromptTemplate = {
  id: 'storyboard_generation',
  name: '分镜脚本生成',
  category: PromptCategory.STORYBOARD,
  description: '从小说文本生成分镜脚本',
  template: `你是一位专业的影视分镜师。请将以下小说场景转换为专业的分镜脚本：

【原文场景】
{scene_content}

【角色信息】
{characters}

【分镜要求】
- 格式：{format}
- 风格：{visual_style}
- 时长：约{duration}秒

请按以下格式输出分镜脚本：

场景{scene_number}：{location}
时间：{time_of_day}

镜头1：
- 景别：{shot_type}
- 画面描述：{description}
- 角色：{characters_in_shot}
- 动作：{action}
- 对白：{dialogue}
- 时长：{shot_duration}秒
- 备注：{notes}

...`,
  variables: [
    { name: 'scene_content', type: 'string', required: true, description: '场景内容' },
    { name: 'characters', type: 'string', required: false, default: '', description: '角色信息' },
    { name: 'format', type: 'string', required: false, default: '电影标准', description: '分镜格式' },
    { name: 'visual_style', type: 'string', required: false, default: '写实', description: '视觉风格' },
    { name: 'duration', type: 'number', required: false, default: 60, description: '预计时长' },
    { name: 'scene_number', type: 'number', required: true, description: '场景编号' }
  ]
};
```

---

## 六、成本控制

### 6.1 成本追踪

```typescript
// 成本追踪器
class CostTracker {
  private usageLog: UsageLog;
  private budgetManager: BudgetManager;
  
  async recordUsage(usage: UsageRecord): Promise<void> {
    await this.usageLog.record(usage);
    
    // 检查预算
    const currentSpend = await this.getCurrentSpend(usage.period);
    const budget = await this.budgetManager.getBudget(usage.period);
    
    if (currentSpend >= budget.warningThreshold) {
      this.emitWarning(currentSpend, budget);
    }
    
    if (currentSpend >= budget.limit) {
      this.emitLimitReached(currentSpend, budget);
    }
  }
  
  async getCurrentSpend(period: Period): Promise<number> {
    const records = await this.usageLog.getByPeriod(period);
    return records.reduce((sum, r) => sum + r.cost, 0);
  }
  
  async getUsageReport(period: Period): Promise<UsageReport> {
    const records = await this.usageLog.getByPeriod(period);
    
    return {
      totalCost: records.reduce((sum, r) => sum + r.cost, 0),
      totalTokens: records.reduce((sum, r) => sum + r.usage.totalTokens, 0),
      byModel: this.groupByModel(records),
      byDate: this.groupByDate(records),
      records
    };
  }
}

interface UsageRecord {
  id: string;
  timestamp: Date;
  model: string;
  provider: string;
  usage: TokenUsage;
  cost: number;
  requestType: string;
  period: Period;
}

interface Budget {
  period: Period;
  limit: number;
  warningThreshold: number;
  actions: BudgetAction[];
}

type Period = 'daily' | 'weekly' | 'monthly';
```

### 6.2 预算管理

```typescript
// 预算管理器
class BudgetManager {
  private budgets: Map<Period, Budget> = new Map();
  
  setBudget(budget: Budget): void {
    this.budgets.set(budget.period, budget);
  }
  
  getBudget(period: Period): Budget {
    return this.budgets.get(period) || {
      period,
      limit: Infinity,
      warningThreshold: Infinity,
      actions: []
    };
  }
  
  async checkBudget(period: Period, estimatedCost: number): Promise<boolean> {
    const budget = this.getBudget(period);
    const currentSpend = await this.getCurrentSpend(period);
    
    return (currentSpend + estimatedCost) <= budget.limit;
  }
}
```

---

## 七、缓存层

### 7.1 响应缓存

```typescript
// 响应缓存
class ResponseCache {
  private cache: CacheStore;
  private config: CacheConfig;
  
  constructor(config: CacheConfig) {
    this.config = config;
    this.cache = new CacheStore(config.maxSize);
  }
  
  async get<T>(request: AIRequest): Promise<T | null> {
    if (!this.config.enabled) return null;
    
    const key = this.generateKey(request);
    const cached = await this.cache.get(key);
    
    if (cached && !this.isExpired(cached)) {
      return cached.response;
    }
    
    return null;
  }
  
  async set<T>(request: AIRequest, response: T): Promise<void> {
    if (!this.config.enabled) return;
    
    const key = this.generateKey(request);
    await this.cache.set(key, {
      response,
      timestamp: Date.now(),
      ttl: this.config.ttl
    });
  }
  
  private generateKey(request: AIRequest): string {
    const content = JSON.stringify({
      prompt: request.prompt,
      systemPrompt: request.systemPrompt,
      temperature: request.temperature,
      maxTokens: request.maxTokens
    });
    
    return crypto.createHash('sha256').update(content).digest('hex');
  }
  
  private isExpired(cached: CachedItem): boolean {
    return Date.now() - cached.timestamp > cached.ttl;
  }
}

interface CacheConfig {
  enabled: boolean;
  maxSize: number;
  ttl: number;
}
```

---

## 八、重试与容错

### 8.1 重试机制

```typescript
// 重试策略
class RetryStrategy {
  private config: RetryConfig;
  
  constructor(config: RetryConfig) {
    this.config = config;
  }
  
  async execute<T>(
    operation: () => Promise<T>,
    isRetryable: (error: Error) => boolean
  ): Promise<T> {
    let lastError: Error | null = null;
    
    for (let attempt = 0; attempt <= this.config.maxRetries; attempt++) {
      try {
        return await operation();
      } catch (error) {
        lastError = error as Error;
        
        if (!isRetryable(error as Error)) {
          throw error;
        }
        
        if (attempt < this.config.maxRetries) {
          const delay = this.calculateDelay(attempt);
          await this.sleep(delay);
        }
      }
    }
    
    throw lastError;
  }
  
  private calculateDelay(attempt: number): number {
    // 指数退避
    const baseDelay = this.config.baseDelay;
    const maxDelay = this.config.maxDelay;
    const delay = Math.min(baseDelay * Math.pow(2, attempt), maxDelay);
    return delay + Math.random() * 1000; // 添加抖动
  }
  
  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

interface RetryConfig {
  maxRetries: number;
  baseDelay: number;
  maxDelay: number;
  retryableErrors: string[];
}
```

### 8.2 故障转移

```typescript
// 故障转移管理
class FailoverManager {
  private adapters: Map<string, ModelAdapter> = new Map();
  private healthStatus: Map<string, boolean> = new Map();
  
  async executeWithFailover<T>(
    primaryAdapter: string,
    operation: (adapter: ModelAdapter) => Promise<T>,
    fallbacks: string[] = []
  ): Promise<T> {
    const candidates = [primaryAdapter, ...fallbacks];
    
    for (const adapterId of candidates) {
      if (!this.isHealthy(adapterId)) {
        continue;
      }
      
      const adapter = this.adapters.get(adapterId);
      if (!adapter) continue;
      
      try {
        const result = await operation(adapter);
        return result;
      } catch (error) {
        this.markUnhealthy(adapterId);
        console.warn(`适配器 ${adapterId} 失败，尝试下一个`);
      }
    }
    
    throw new Error('所有模型适配器均不可用');
  }
  
  private isHealthy(adapterId: string): boolean {
    return this.healthStatus.get(adapterId) !== false;
  }
  
  private markUnhealthy(adapterId: string): void {
    this.healthStatus.set(adapterId, false);
    
    // 30秒后重试
    setTimeout(() => {
      this.healthStatus.set(adapterId, true);
    }, 30000);
  }
}
```

---

## 九、模型配置管理

### 9.1 配置界面

```typescript
// 模型配置
interface ModelConfig {
  id: string;
  name: string;
  enabled: boolean;
  adapter: string;
  
  // API配置
  apiKey?: string;
  baseUrl?: string;
  
  // 模型参数
  defaultModel: string;
  temperature: number;
  maxTokens: number;
  
  // 高级设置
  timeout: number;
  maxRetries: number;
  
  // 成本设置
  costTracking: boolean;
  budgetLimit?: number;
}

// 配置管理器
class ModelConfigManager {
  private configs: Map<string, ModelConfig> = new Map();
  
  loadConfigs(): void {
    // 从存储加载配置
  }
  
  saveConfig(config: ModelConfig): void {
    this.configs.set(config.id, config);
    // 持久化
  }
  
  getConfig(id: string): ModelConfig | undefined {
    return this.configs.get(id);
  }
  
  getAllConfigs(): ModelConfig[] {
    return Array.from(this.configs.values());
  }
}
```

---

**文档版本**: v1.0  
**最后更新**: 2026-02-19  
**维护者**: AI小说创作工作室开发团队
