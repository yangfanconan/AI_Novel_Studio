export interface AIModelConfig {
  id: string;
  name: string;
  provider: string;
  apiEndpoint: string;
  apiKey?: string;
  supportsStreaming: boolean;
}

export interface PromptTemplate {
  id: string;
  name: string;
  category: string;
  systemPrompt: string;
  userPromptTemplate: string;
  variables: string[];
}

export interface AICompletionRequest {
  model_id: string;
  context: string;
  instruction: string;
  temperature?: number;
  max_tokens?: number;
  stream?: boolean;
  character_context?: string;
  worldview_context?: string;
  project_id?: string;
}

export interface AIRewriteRequest {
  model_id: string;
  content: string;
  instruction: string;
  temperature?: number;
  max_tokens?: number;
}
