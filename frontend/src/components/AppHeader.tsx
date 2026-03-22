import { useTheme } from '@/hooks/useTheme';

export function AppHeader() {
  const { theme, toggle } = useTheme();

  return (
    <header
      className="sticky top-0 z-50 flex h-14 items-center justify-between px-6"
      style={{
        background: 'var(--lk-bg)',
        borderBottom: '1px solid var(--lk-border)',
        transition: 'background 0.2s ease, border-color 0.2s ease',
      }}
    >
      {/* Logo: plain text, no accent coloring */}
      <div className="flex items-center gap-2">
        <span
          className="text-[15px] font-semibold tracking-tight"
          style={{
            color: 'var(--lk-text)',
            letterSpacing: '-0.3px',
          }}
        >
          LearnKit
        </span>
      </div>

      {/* Theme toggle capsule */}
      <button
        onClick={toggle}
        className="flex min-h-[44px] min-w-[44px] items-center justify-center rounded-full p-2 cursor-pointer border-none"
        style={{
          background:
            theme === 'dark'
              ? 'rgba(255,255,255,0.06)'
              : 'rgba(0,0,0,0.06)',
          transition: 'background 0.2s ease',
        }}
        aria-label={theme === 'dark' ? '切换到日间模式' : '切换到夜间模式'}
        title={theme === 'dark' ? '切换到日间模式' : '切换到夜间模式'}
      >
        <span
          className="flex size-7 items-center justify-center rounded-full text-sm leading-none"
          aria-hidden="true"
          style={{
            background:
              theme === 'light' ? 'var(--lk-accent)' : 'transparent',
            transition: 'background 0.2s ease',
          }}
        >
          ☀️
        </span>
        <span
          className="flex size-7 items-center justify-center rounded-full text-sm leading-none"
          aria-hidden="true"
          style={{
            background:
              theme === 'dark' ? 'var(--lk-accent)' : 'transparent',
            transition: 'background 0.2s ease',
          }}
        >
          🌙
        </span>
      </button>
    </header>
  );
}
