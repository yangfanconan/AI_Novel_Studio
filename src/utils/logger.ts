export enum LogLevel {
  DEBUG = "DEBUG",
  INFO = "INFO",
  WARN = "WARN",
  ERROR = "ERROR",
}

export interface LogContext {
  feature?: string;
  action?: string;
  userId?: string;
  requestId?: string;
  component?: string;
  [key: string]: any;
}

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  message: string;
  context?: LogContext;
  error?: Error;
  stack?: string;
}

class Logger {
  private context: LogContext = {};
  private requestId: string;

  constructor(context: LogContext = {}) {
    this.context = context;
    this.requestId = this.generateRequestId();
    this.info("Logger initialized", { feature: "logger" });
  }

  private generateRequestId(): string {
    return `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
  }

  private formatMessage(level: LogLevel, message: string, error?: Error): string {
    const timestamp = new Date().toISOString();
    const contextStr =
      Object.keys(this.context).length > 0 ? ` | Context: ${JSON.stringify(this.context)}` : "";
    const errorStr = error ? ` | Error: ${error.message}` : "";
    return `[${timestamp}] [${level}] [${this.requestId}] ${message}${contextStr}${errorStr}`;
  }

  private log(level: LogLevel, message: string, error?: Error): void {
    const entry: LogEntry = {
      timestamp: new Date().toISOString(),
      level,
      message,
      context: { ...this.context, requestId: this.requestId },
      error,
      stack: error?.stack,
    };

    const formatted = this.formatMessage(level, message, error);

    switch (level) {
      case LogLevel.DEBUG:
        console.debug(formatted);
        break;
      case LogLevel.INFO:
        console.info(formatted);
        break;
      case LogLevel.WARN:
        console.warn(formatted);
        break;
      case LogLevel.ERROR:
        console.error(formatted);
        if (error?.stack) {
          console.error("Stack trace:", error.stack);
        }
        break;
    }

    if (typeof window !== "undefined" && (window as any).logEntries) {
      (window as any).logEntries.push(entry);
    }
  }

  debug(message: string, context?: LogContext): void {
    if (context) {
      this.context = { ...this.context, ...context };
    }
    this.log(LogLevel.DEBUG, message);
  }

  info(message: string, context?: LogContext): void {
    if (context) {
      this.context = { ...this.context, ...context };
    }
    this.log(LogLevel.INFO, message);
  }

  warn(message: string, context?: LogContext): void {
    if (context) {
      this.context = { ...this.context, ...context };
    }
    this.log(LogLevel.WARN, message);
  }

  error(message: string, error?: Error, context?: LogContext): void {
    if (context) {
      this.context = { ...this.context, ...context };
    }
    this.log(LogLevel.ERROR, message, error);
  }

  withContext(context: LogContext): Logger {
    return new Logger({ ...this.context, ...context });
  }

  setContext(context: LogContext): void {
    this.context = { ...this.context, ...context };
  }

  clearContext(): void {
    this.context = { requestId: this.requestId };
  }

  trackAction(action: string, startTime?: number): () => void {
    const start = startTime || performance.now();
    this.info(`Action started: ${action}`, { action });

    return () => {
      const duration = performance.now() - start;
      this.info(`Action completed: ${action}`, { action, duration: `${duration.toFixed(2)}ms` });
    };
  }
}

export const logger = new Logger({ feature: "app" });
export const createLogger = (context: LogContext) => new Logger(context);
