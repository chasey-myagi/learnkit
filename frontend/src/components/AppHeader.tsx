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
      {/* Gradient bottom line */}
      <div
        className="absolute bottom-[-1px] left-0 right-0 h-[2px]"
        style={{
          background: 'linear-gradient(90deg, var(--lk-accent) 0%, var(--lk-accent-light) 30%, transparent 50%)',
        }}
      />

      {/* Logo */}
      <div className="flex items-center gap-2">
        <span className="text-xl leading-none">📚</span>
        <span
          className="text-[15px] font-bold tracking-tight"
          style={{ color: 'var(--lk-text)', transition: 'color 0.2s ease', letterSpacing: '-0.3px' }}
        >
          <em className="not-italic" style={{ color: 'var(--lk-accent)' }}>
            Learn
          </em>
          Kit
        </span>
      </div>

      {/* Theme toggle capsule */}
      <button
        onClick={toggle}
        className="flex items-center rounded-full p-1 cursor-pointer border-none"
        style={{
          background: theme === 'dark' ? 'rgba(255,255,255,0.06)' : 'rgba(0,0,0,0.06)',
          transition: 'background 0.2s ease',
        }}
        aria-label="切换日夜模式"
      >
        <span
          className="flex size-7 items-center justify-center rounded-full text-sm leading-none"
          style={{
            background: theme === 'light' ? 'var(--lk-accent)' : 'transparent',
            transition: 'background 0.2s ease',
          }}
        >
          ☀️
        </span>
        <span
          className="flex size-7 items-center justify-center rounded-full text-sm leading-none"
          style={{
            background: theme === 'dark' ? 'var(--lk-accent)' : 'transparent',
            transition: 'background 0.2s ease',
          }}
        >
          🌙
        </span>
      </button>
    </header>
  );
}
