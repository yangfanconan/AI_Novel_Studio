import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect, useCallback } from "react";

export interface ApiError {
  message: string;
  code?: string;
  details?: any;
}

export interface ApiResponse<T> {
  data: T;
  error?: ApiError;
  success: boolean;
}

export type ApiMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

export interface RequestConfig {
  method?: ApiMethod;
  params?: Record<string, any>;
  body?: any;
  timeout?: number;
}

export class ApiClientError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
    public code?: string,
    public details?: any
  ) {
    super(message);
    this.name = "ApiClientError";
  }
}

export class ApiNetworkError extends ApiClientError {
  constructor(message: string, details?: any) {
    super(message, undefined, "NETWORK_ERROR", details);
    this.name = "ApiNetworkError";
  }
}

export class ApiTimeoutError extends ApiClientError {
  constructor(message: string = "请求超时", details?: any) {
    super(message, 408, "TIMEOUT", details);
    this.name = "ApiTimeoutError";
  }
}

export class ApiValidationError extends ApiClientError {
  constructor(message: string, details?: any) {
    super(message, 400, "VALIDATION_ERROR", details);
    this.name = "ApiValidationError";
  }
}

export class ApiNotFoundError extends ApiClientError {
  constructor(message: string = "资源未找到", details?: any) {
    super(message, 404, "NOT_FOUND", details);
    this.name = "ApiNotFoundError";
  }
}

export class ApiServerError extends ApiClientError {
  constructor(message: string = "服务器错误", details?: any) {
    super(message, 500, "SERVER_ERROR", details);
    this.name = "ApiServerError";
  }
}

export interface LoadingState {
  isLoading: boolean;
  error: ApiError | null;
  data: any;
}

export interface LoadingStates {
  [key: string]: LoadingState;
}

export class ApiClient {
  private baseUrl: string = "";
  private timeout: number = 30000;
  private defaultHeaders: Record<string, string> = {};
  private loadingStates: LoadingStates = {};
  private listeners: Map<string, Set<(state: LoadingState) => void>> = new Map();

  constructor(config?: { baseUrl?: string; timeout?: number; headers?: Record<string, string> }) {
    if (config?.baseUrl) this.baseUrl = config.baseUrl;
    if (config?.timeout) this.timeout = config.timeout;
    if (config?.headers) this.defaultHeaders = config.headers;
  }

  private async invokeTauri<T>(command: string, args?: any): Promise<T> {
    return await invoke<T>(command, args);
  }

  private handleError(error: any): ApiClientError {
    if (error instanceof ApiClientError) {
      return error;
    }

    if (error.name === "TimeoutError" || error.message?.includes("timeout")) {
      return new ApiTimeoutError(error.message);
    }

    if (error.message?.includes("Network") || error.message?.includes("fetch")) {
      return new ApiNetworkError(error.message, error);
    }

    if (error.code === 404 || error.message?.includes("not found")) {
      return new ApiNotFoundError(error.message);
    }

    if (error.code === 400 || error.message?.includes("validation")) {
      return new ApiValidationError(error.message, error.details);
    }

    if (error.code && error.code >= 500) {
      return new ApiServerError(error.message, error);
    }

    return new ApiClientError(error.message || "未知错误", error.code, error.details);
  }

  private updateLoadingState(key: string, state: Partial<LoadingState>) {
    this.loadingStates[key] = {
      ...this.loadingStates[key],
      ...state,
    };
    const listeners = this.listeners.get(key);
    if (listeners) {
      listeners.forEach((listener) => listener(this.loadingStates[key]));
    }
  }

  private subscribeToLoading(key: string, callback: (state: LoadingState) => void) {
    if (!this.listeners.has(key)) {
      this.listeners.set(key, new Set());
    }
    this.listeners.get(key)!.add(callback);
    return () => {
      this.listeners.get(key)?.delete(callback);
    };
  }

  private getLoadingState(key: string): LoadingState {
    return this.loadingStates[key] || { isLoading: false, error: null, data: null };
  }

  async request<T>(
    key: string,
    command: string,
    args?: any,
    config?: RequestConfig
  ): Promise<T> {
    const timeout = config?.timeout || this.timeout;

    try {
      this.updateLoadingState(key, { isLoading: true, error: null });

      const timeoutPromise = new Promise<never>((_, reject) => {
        setTimeout(() => reject(new Error("Timeout")), timeout);
      });

      const result = await Promise.race([
        this.invokeTauri<T>(command, args),
        timeoutPromise,
      ]);

      this.updateLoadingState(key, { isLoading: false, data: result });
      return result;
    } catch (error) {
      const apiError = this.handleError(error);
      this.updateLoadingState(key, { isLoading: false, error: apiError });
      throw apiError;
    }
  }

  async get<T>(key: string, command: string, args?: any, config?: RequestConfig): Promise<T> {
    return this.request<T>(key, command, args, { ...config, method: "GET" });
  }

  async post<T>(key: string, command: string, args?: any, config?: RequestConfig): Promise<T> {
    return this.request<T>(key, command, args, { ...config, method: "POST" });
  }

  async put<T>(key: string, command: string, args?: any, config?: RequestConfig): Promise<T> {
    return this.request<T>(key, command, args, { ...config, method: "PUT" });
  }

  async delete<T>(key: string, command: string, args?: any, config?: RequestConfig): Promise<T> {
    return this.request<T>(key, command, args, { ...config, method: "DELETE" });
  }

  async patch<T>(key: string, command: string, args?: any, config?: RequestConfig): Promise<T> {
    return this.request<T>(key, command, args, { ...config, method: "PATCH" });
  }

  isLoading(key: string): boolean {
    return this.getLoadingState(key).isLoading;
  }

  getError(key: string): ApiError | null {
    return this.getLoadingState(key).error;
  }

  getData(key: string): any {
    return this.getLoadingState(key).data;
  }

  onLoadingChange(key: string, callback: (state: LoadingState) => void) {
    return this.subscribeToLoading(key, callback);
  }

  clearLoadingState(key: string) {
    delete this.loadingStates[key];
    this.listeners.delete(key);
  }

  clearAllLoadingStates() {
    this.loadingStates = {};
    this.listeners.clear();
  }
}

export const apiClient = new ApiClient();

export const createApiHook = <T, Args = any>(
  key: string,
  command: string,
  method: ApiMethod = "GET"
) => {
  return {
    useQuery: (args?: Args) => {
      const [data, setData] = useState<T | null>(null);
      const [loading, setLoading] = useState(false);
      const [error, setError] = useState<ApiError | null>(null);

      const execute = useCallback(
        async (executeArgs?: Args) => {
          try {
            setLoading(true);
            setError(null);
            const result = await apiClient.request<T>(
              key,
              command,
              executeArgs || args
            );
            setData(result);
            return result;
          } catch (err) {
            setError(err as ApiError);
            throw err;
          } finally {
            setLoading(false);
          }
        },
        [args]
      );

      useEffect(() => {
        execute();
      }, [execute]);

      return { data, loading, error, refetch: execute };
    },

    useMutation: () => {
      const [loading, setLoading] = useState(false);
      const [error, setError] = useState<ApiError | null>(null);

      const mutate = useCallback(
        async (mutationArgs: Args) => {
          try {
            setLoading(true);
            setError(null);
            const result = await apiClient.request<T>(
              key,
              command,
              mutationArgs,
              { method }
            );
            return { data: result, error: null };
          } catch (err) {
            setError(err as ApiError);
            return { data: null, error: err as ApiError };
          } finally {
            setLoading(false);
          }
        },
        []
      );

      return { mutate, loading, error };
    },
  };
};
