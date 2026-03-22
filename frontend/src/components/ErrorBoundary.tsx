import { Component } from 'react';
import type { ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('[LearnKit] Unhandled render error:', error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <div
          className="flex min-h-[60vh] flex-col items-center justify-center px-8"
          style={{ color: 'var(--lk-text)' }}
        >
          <svg
            className="mb-4 size-12"
            viewBox="0 0 48 48"
            fill="none"
            stroke="var(--lk-failed)"
            strokeWidth="2"
            strokeLinecap="round"
            aria-hidden="true"
          >
            <circle cx="24" cy="24" r="20" />
            <path d="M24 14v12" />
            <circle cx="24" cy="32" r="1.5" fill="var(--lk-failed)" />
          </svg>
          <p
            className="mb-2 text-lg font-semibold"
            style={{ letterSpacing: '-0.3px' }}
          >
            页面出错了
          </p>
          <p
            className="mb-6 text-sm"
            style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.6', maxWidth: 400, textAlign: 'center' }}
          >
            应用遇到了意外错误，请尝试刷新页面
          </p>
          <button
            onClick={() => window.location.reload()}
            className="cursor-pointer rounded-lg border-none px-5 py-2.5 text-sm font-semibold text-white"
            style={{
              background: 'var(--lk-accent)',
              transition: 'opacity 0.2s ease',
            }}
            onMouseEnter={(e) => { e.currentTarget.style.opacity = '0.85'; }}
            onMouseLeave={(e) => { e.currentTarget.style.opacity = '1'; }}
          >
            刷新页面
          </button>
          {this.state.error && (
            <pre
              className="mt-6 max-w-[600px] overflow-auto rounded-lg p-4 text-xs"
              style={{
                background: 'var(--lk-card)',
                border: '1px solid var(--lk-border)',
                color: 'var(--lk-text-secondary)',
                whiteSpace: 'pre-wrap',
                wordBreak: 'break-word',
              }}
            >
              {this.state.error.message}
            </pre>
          )}
        </div>
      );
    }

    return this.props.children;
  }
}
