import { invoke } from '@tauri-apps/api/core';

export enum LogLevel {
  DEBUG = 'DEBUG',
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR',
}

interface DebugLogEntry {
  timestamp: number;
  level: LogLevel;
  source: 'frontend' | 'backend' | 'system';
  feature?: string;
  action?: string;
  component?: string;
  message: string;
  data?: any;
  error?: string;
  stack?: string;
  projectId?: string;
  count?: number;
  duration?: number;
  tauriAvailable?: boolean;
  selectedModel?: string;
  model?: string;
  resultLength?: number;
  errorMessage?: string;
  models?: string[];
  contentLength?: number;
  resultPreview?: string;
  attempt?: number;
  isContinuing?: boolean;
  defaultModel?: string | null;
}

class DebugLogger {
  private logs: DebugLogEntry[] = [];
  private maxLogs = 500;
  private tauriAvailable: boolean = false;
  private initializationPromise: Promise<void>;

  constructor() {
    this.initializationPromise = this.checkTauriAvailable();
  }

  private async checkTauriAvailable(): Promise<void> {
    const maxRetries = 10;
    const retryDelay = 100;

    for (let i = 0; i < maxRetries; i++) {
      try {
        if (typeof window !== 'undefined' && '__TAURI__' in window) {
          if (typeof invoke === 'function') {
            this.tauriAvailable = true;
            this.debug('DebugLogger initialized', { tauriAvailable: true, attempt: i + 1 });
            return;
          }
        }
      } catch (e) {
        console.warn(`Check attempt ${i + 1} failed:`, e);
      }
      
      await new Promise(resolve => setTimeout(resolve, retryDelay));
    }

    console.warn('Tauri invoke not available after retries, logging to console only');
    this.tauriAvailable = false;
  }

  private async waitUntilReady(): Promise<void> {
    await this.initializationPromise;
  }

  private createLog(
    level: LogLevel,
    message: string,
    source: DebugLogEntry['source'] = 'frontend',
    extra: Partial<DebugLogEntry> = {}
  ): void {
    const entry: DebugLogEntry = {
      timestamp: Date.now(),
      level,
      source,
      message,
      ...extra,
    };

    this.logs.push(entry);

    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }

    this.output(entry);
    this.persist(entry);
  }

  private output(entry: DebugLogEntry): void {
    const timestamp = new Date(entry.timestamp).toISOString();
    const prefix = `[${timestamp}] [${entry.level}] [${entry.source}]`;
    const component = entry.component ? ` [${entry.component}]` : '';
    const feature = entry.feature ? ` [${entry.feature}]` : '';
    const action = entry.action ? ` [${entry.action}]` : '';

    let message = `${prefix}${component}${feature}${action} ${entry.message}`;

    if (entry.data) {
      message += ` | Data: ${JSON.stringify(entry.data, null, 2)}`;
    }

    if (entry.error) {
      message += ` | Error: ${entry.error}`;
    }

    if (entry.stack) {
      message += `\nStack: ${entry.stack}`;
    }

    switch (entry.level) {
      case LogLevel.DEBUG:
        console.debug(message);
        break;
      case LogLevel.INFO:
        console.info(message);
        break;
      case LogLevel.WARN:
        console.warn(message);
        break;
      case LogLevel.ERROR:
        console.error(message);
        break;
    }
  }

  private async persist(entry: DebugLogEntry): Promise<void> {
    await this.waitUntilReady();
    
    if (!this.tauriAvailable) {
      return;
    }

    try {
      await invoke('save_debug_log', { entry });
    } catch (e) {
      console.warn('Failed to persist debug log:', e);
    }
  }

  debug(message: string, extra: Partial<DebugLogEntry> = {}): void {
    this.createLog(LogLevel.DEBUG, message, 'frontend', extra);
  }

  info(message: string, extra: Partial<DebugLogEntry> = {}): void {
    this.createLog(LogLevel.INFO, message, 'frontend', extra);
  }

  warn(message: string, extra: Partial<DebugLogEntry> = {}): void {
    this.createLog(LogLevel.WARN, message, 'frontend', extra);
  }

  error(message: string, error?: Error, extra: Partial<DebugLogEntry> = {}): void {
    this.createLog(LogLevel.ERROR, message, 'frontend', {
      ...extra,
      error: error?.message,
      stack: error?.stack,
    });
  }

  trackAction(action: string, extra: Partial<DebugLogEntry> = {}): () => void {
    const startTime = performance.now();
    this.info(`Action started: ${action}`, { action, ...extra });

    return () => {
      const duration = performance.now() - startTime;
      this.info(`Action completed: ${action}`, { action, duration: Math.round(duration * 100) / 100, ...extra });
    };
  }

  getLogs(): DebugLogEntry[] {
    return [...this.logs];
  }

  clearLogs(): void {
    this.logs = [];
    this.debug('Debug logs cleared');
  }

  exportLogs(): string {
    return this.logs
      .map(log => {
        const timestamp = new Date(log.timestamp).toISOString();
        const data = log.data ? ` | Data: ${JSON.stringify(log.data)}` : '';
        const error = log.error ? ` | Error: ${log.error}` : '';
        return `[${timestamp}] [${log.level}] [${log.source}] [${log.feature || 'N/A'}] ${log.message}${data}${error}`;
      })
      .join('\n');
  }

  async exportToFile(): Promise<void> {
    await this.waitUntilReady();
    
    if (!this.tauriAvailable) {
      this.warn('Tauri not available, cannot export to file');
      return;
    }

    try {
      const content = this.exportLogs();
      await invoke('save_debug_log_file', { content });
      this.info('Debug logs exported to file');
    } catch (error) {
      this.error('Failed to export debug logs', error);
    }
  }
}

export const debugLogger = new DebugLogger();

window.addEventListener('error', (event) => {
  debugLogger.error('Uncaught error', event.error, {
    component: 'window',
    feature: 'error-handler',
  });
});

window.addEventListener('unhandledrejection', (event) => {
  debugLogger.error('Unhandled promise rejection', event.reason as Error, {
    component: 'window',
    feature: 'promise-handler',
  });
});

declare global {
  interface Window {
    debugLogger: typeof debugLogger;
    exportDebugLogs: () => Promise<void>;
  }
}

window.debugLogger = debugLogger;
window.exportDebugLogs = () => debugLogger.exportToFile();
