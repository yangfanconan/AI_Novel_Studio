import { invoke } from '@tauri-apps/api/core';

export interface UILogEntry {
  component: string;
  action: 'open' | 'close' | 'mount' | 'unmount' | 'click' | 'change' | 'error';
  timestamp: number;
  data?: Record<string, any>;
}

class UILogger {
  private logs: UILogEntry[] = [];
  private flushTimer: NodeJS.Timeout | null = null;
  private enabled: boolean = true;

  constructor() {
    this.setupAutoFlush();
  }

  private setupAutoFlush() {
    this.flushTimer = setInterval(() => {
      this.flush();
    }, 5000);
  }

  log(entry: UILogEntry) {
    if (!this.enabled) return;

    console.log('[UI]', entry.component, entry.action, entry.data || '');
    this.logs.push(entry);

    if (this.logs.length >= 10) {
      this.flush();
    }
  }

  open(component: string, data?: Record<string, any>) {
    return this.log({
      component,
      action: 'open',
      timestamp: Date.now(),
      data,
    });
  }

  close(component: string, data?: Record<string, any>) {
    return this.log({
      component,
      action: 'close',
      timestamp: Date.now(),
      data,
    });
  }

  mount(component: string, data?: Record<string, any>) {
    return this.log({
      component,
      action: 'mount',
      timestamp: Date.now(),
      data,
    });
  }

  unmount(component: string, data?: Record<string, any>) {
    return this.log({
      component,
      action: 'unmount',
      timestamp: Date.now(),
      data,
    });
  }

  click(component: string, target: string, data?: Record<string, any>) {
    return this.log({
      component,
      action: 'click',
      timestamp: Date.now(),
      data: { target, ...data },
    });
  }

  error(component: string, error: any, data?: Record<string, any>) {
    return this.log({
      component,
      action: 'error',
      timestamp: Date.now(),
      data: { error: error.message || String(error), ...data },
    });
  }

  flush() {
    if (this.logs.length === 0) return;

    const logsToSend = [...this.logs];
    this.logs = [];

    if (typeof invoke === 'undefined') {
      console.warn('[UI] Tauri invoke not available, skipping log flush');
      this.logs.unshift(...logsToSend);
      return;
    }

    invoke('save_ui_logs', { logs: logsToSend })
      .then(() => {
        console.log('[UI] Logs flushed:', logsToSend.length);
      })
      .catch((error) => {
        console.error('[UI] Failed to flush logs:', error);
        this.logs.unshift(...logsToSend);
      });
  }

  enable() {
    this.enabled = true;
  }

  disable() {
    this.enabled = false;
  }

  destroy() {
    if (this.flushTimer) {
      clearInterval(this.flushTimer);
    }
    this.flush();
  }
}

export const uiLogger = new UILogger();
