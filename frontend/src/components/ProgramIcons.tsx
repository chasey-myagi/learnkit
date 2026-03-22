/** SVG icons per program slug, fallback to a generic icon */
export const programIcons: Record<string, React.ReactNode> = {
  'game-dev': (
    <svg viewBox="0 0 48 48" fill="none" className="size-full" aria-hidden="true">
      <rect x="6" y="15" width="36" height="21" rx="10" stroke="var(--lk-accent)" strokeWidth="2.5" />
      <circle cx="17" cy="25" r="3" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="31" cy="23" r="1.8" fill="var(--lk-accent)" />
      <circle cx="35" cy="27" r="1.8" fill="var(--lk-accent)" />
      <rect x="21" y="18" width="6" height="4" rx="2" fill="var(--lk-accent)" opacity=".3" />
    </svg>
  ),
  'reinforcement-learning': (
    <svg viewBox="0 0 48 48" fill="none" className="size-full" aria-hidden="true">
      <circle cx="12" cy="16" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="12" cy="32" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="24" cy="12" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="24" cy="24" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="24" cy="36" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="36" cy="20" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="36" cy="32" r="3.5" stroke="var(--lk-accent)" strokeWidth="2" />
      <path d="M15.5 16l5-2.5M15.5 16l5 6.5M15.5 32l5-6.5M15.5 32l5 2.5M27.5 12l5 6M27.5 24l5-2.5M27.5 24l5 6.5M27.5 36l5-2.5" stroke="var(--lk-accent)" strokeWidth="1" opacity=".4" />
    </svg>
  ),
  'rust-systems': (
    <svg viewBox="0 0 48 48" fill="none" className="size-full" aria-hidden="true">
      <circle cx="24" cy="24" r="9" stroke="var(--lk-completed)" strokeWidth="2.5" />
      <circle cx="24" cy="24" r="4" fill="var(--lk-completed)" opacity=".2" />
      <path d="M24 12v-3M24 39v-3M12 24H9M39 24h-3M16 16l-2-2M34 34l-2-2M16 32l-2 2M34 16l-2-2" stroke="var(--lk-completed)" strokeWidth="2.5" strokeLinecap="round" />
    </svg>
  ),
};

export const bgIcons: Record<string, React.ReactNode> = {
  'game-dev': (
    <svg viewBox="0 0 80 80" fill="none" className="size-full" aria-hidden="true">
      <rect x="10" y="25" width="60" height="35" rx="17" stroke="currentColor" strokeWidth="2.5" />
      <circle cx="28" cy="42" r="4" stroke="currentColor" strokeWidth="2" />
      <circle cx="52" cy="38" r="2.5" fill="currentColor" />
      <circle cx="58" cy="44" r="2.5" fill="currentColor" />
      <rect x="35" y="30" width="10" height="6" rx="3" fill="currentColor" opacity=".3" />
    </svg>
  ),
  'reinforcement-learning': (
    <svg viewBox="0 0 80 80" fill="none" className="size-full" aria-hidden="true">
      <circle cx="20" cy="25" r="5" stroke="currentColor" strokeWidth="2" />
      <circle cx="20" cy="55" r="5" stroke="currentColor" strokeWidth="2" />
      <circle cx="40" cy="20" r="5" stroke="currentColor" strokeWidth="2" />
      <circle cx="40" cy="40" r="5" stroke="currentColor" strokeWidth="2" />
      <circle cx="40" cy="60" r="5" stroke="currentColor" strokeWidth="2" />
      <circle cx="60" cy="35" r="5" stroke="currentColor" strokeWidth="2" />
      <circle cx="60" cy="55" r="5" stroke="currentColor" strokeWidth="2" />
      <path d="M25 25l10-3M25 25l10 13M25 55l10-13M25 55l10 3M45 20l10 13M45 40l10-3M45 40l10 13M45 60l10-3M45 60l10-7" stroke="currentColor" strokeWidth="1.2" opacity=".5" />
    </svg>
  ),
  'rust-systems': (
    <svg viewBox="0 0 80 80" fill="none" className="size-full" aria-hidden="true">
      <circle cx="40" cy="40" r="14" stroke="currentColor" strokeWidth="2.5" />
      <circle cx="40" cy="40" r="6" fill="currentColor" opacity=".2" />
      <path d="M40 22v-4M40 62v-4M22 40h-4M62 40h-4M28.3 28.3l-2.8-2.8M54.5 54.5l-2.8-2.8M28.3 51.7l-2.8 2.8M54.5 25.5l-2.8 2.8" stroke="currentColor" strokeWidth="3" strokeLinecap="round" />
    </svg>
  ),
};
