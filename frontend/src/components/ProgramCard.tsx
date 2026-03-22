import { useNavigate } from 'react-router-dom';
import { ProgressBar } from './ProgressBar';
import { ProgressRing } from './ProgressRing';
import type { ProgressData, ScopeData } from '@/api/client';

interface ProgramCardProps {
  slug: string;
  scope: ScopeData;
  progress: ProgressData;
}

/** SVG icons per program slug, fallback to a generic icon */
const programIcons: Record<string, React.ReactNode> = {
  'game-dev': (
    <svg viewBox="0 0 48 48" fill="none" className="size-full">
      <rect x="6" y="15" width="36" height="21" rx="10" stroke="var(--lk-accent)" strokeWidth="2.5" />
      <circle cx="17" cy="25" r="3" stroke="var(--lk-accent)" strokeWidth="2" />
      <circle cx="31" cy="23" r="1.8" fill="var(--lk-accent)" />
      <circle cx="35" cy="27" r="1.8" fill="var(--lk-accent)" />
      <rect x="21" y="18" width="6" height="4" rx="2" fill="var(--lk-accent)" opacity=".3" />
    </svg>
  ),
  'reinforcement-learning': (
    <svg viewBox="0 0 48 48" fill="none" className="size-full">
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
    <svg viewBox="0 0 48 48" fill="none" className="size-full">
      <circle cx="24" cy="24" r="9" stroke="var(--lk-completed)" strokeWidth="2.5" />
      <circle cx="24" cy="24" r="4" fill="var(--lk-completed)" opacity=".2" />
      <path d="M24 12v-3M24 39v-3M12 24H9M39 24h-3M16 16l-2-2M34 34l-2-2M16 32l-2 2M34 16l-2-2" stroke="var(--lk-completed)" strokeWidth="2.5" strokeLinecap="round" />
    </svg>
  ),
};

const bgIcons: Record<string, React.ReactNode> = {
  'game-dev': (
    <svg viewBox="0 0 80 80" fill="none" className="size-full">
      <rect x="10" y="25" width="60" height="35" rx="17" stroke="currentColor" strokeWidth="2.5" />
      <circle cx="28" cy="42" r="4" stroke="currentColor" strokeWidth="2" />
      <circle cx="52" cy="38" r="2.5" fill="currentColor" />
      <circle cx="58" cy="44" r="2.5" fill="currentColor" />
      <rect x="35" y="30" width="10" height="6" rx="3" fill="currentColor" opacity=".3" />
    </svg>
  ),
  'reinforcement-learning': (
    <svg viewBox="0 0 80 80" fill="none" className="size-full">
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
    <svg viewBox="0 0 80 80" fill="none" className="size-full">
      <circle cx="40" cy="40" r="14" stroke="currentColor" strokeWidth="2.5" />
      <circle cx="40" cy="40" r="6" fill="currentColor" opacity=".2" />
      <path d="M40 22v-4M40 62v-4M22 40h-4M62 40h-4M28.3 28.3l-2.8-2.8M54.5 54.5l-2.8-2.8M28.3 51.7l-2.8 2.8M54.5 25.5l-2.8 2.8" stroke="currentColor" strokeWidth="3" strokeLinecap="round" />
    </svg>
  ),
};

function getStatusTag(progress: ProgressData) {
  if (progress.completed === progress.total) {
    return { label: '已完成', className: 'tag-done', bg: 'rgba(16,172,132,0.12)', color: 'var(--lk-completed)' };
  }
  if (progress.in_progress > 0 || progress.completed > 0) {
    return { label: '进行中', className: 'tag-progress', bg: 'rgba(108,92,231,0.12)', color: 'var(--lk-accent)' };
  }
  return { label: '未开始', className: 'tag-new', bg: 'rgba(120,120,160,0.12)', color: 'var(--lk-pending)' };
}

function getDescription(slug: string): string {
  const descs: Record<string, string> = {
    'game-dev': '从零构建游戏引擎与玩法逻辑',
    'reinforcement-learning': '策略梯度、Q-learning 与环境建模',
    'rust-systems': '所有权、生命周期与高性能系统设计',
  };
  return descs[slug] ?? '';
}

export function ProgramCard({ slug, scope, progress }: ProgramCardProps) {
  const navigate = useNavigate();
  const percent = progress.total > 0 ? (progress.completed / progress.total) * 100 : 0;
  const isComplete = percent >= 100;
  const tag = getStatusTag(progress);
  const totalSubjects = scope.subjects.length;
  const totalLessons = scope.subjects.reduce((sum, s) => sum + s.lessons.length, 0);

  return (
    <article
      className="relative flex cursor-pointer flex-col gap-4 overflow-hidden rounded-2xl border p-6"
      style={{
        background: 'var(--lk-card)',
        borderColor: isComplete ? 'rgba(16,172,132,0.3)' : 'var(--lk-border)',
        transition: 'border-color 0.2s ease, transform 0.25s cubic-bezier(.4,0,.2,1), box-shadow 0.25s cubic-bezier(.4,0,.2,1), background 0.2s ease',
      }}
      role="button"
      tabIndex={0}
      aria-label={`查看${scope.title}课程详情`}
      onClick={() => navigate(`/program/${slug}`)}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          navigate(`/program/${slug}`);
        }
      }}
      onMouseEnter={(e) => {
        const el = e.currentTarget;
        el.style.borderColor = isComplete ? 'var(--lk-completed)' : 'var(--lk-accent)';
        el.style.transform = 'translateY(-2px)';
        el.style.boxShadow = isComplete
          ? '0 4px 20px rgba(0,0,0,0.15), 0 0 0 1px rgba(16,172,132,0.15)'
          : '0 4px 20px rgba(0,0,0,0.15), 0 0 0 1px rgba(108,92,231,0.15)';
      }}
      onMouseLeave={(e) => {
        const el = e.currentTarget;
        el.style.borderColor = isComplete ? 'rgba(16,172,132,0.3)' : 'var(--lk-border)';
        el.style.transform = 'translateY(0)';
        el.style.boxShadow = 'none';
      }}
    >
      {/* Background decorative icon */}
      <div
        className="pointer-events-none absolute select-none"
        style={{
          right: -12,
          bottom: -12,
          width: 120,
          height: 120,
          opacity: 0.05,
          color: 'var(--lk-text)',
          transition: 'color 0.2s ease',
        }}
      >
        {bgIcons[slug] ?? bgIcons['game-dev']}
      </div>

      {/* Top row: icon + title + status tag */}
      <div className="flex items-center gap-3">
        <div className="size-11 shrink-0">
          {programIcons[slug] ?? programIcons['game-dev']}
        </div>
        <div className="min-w-0 flex-1">
          <div
            className="text-[17px] font-bold leading-tight"
            style={{ letterSpacing: '-0.3px', color: 'var(--lk-text)' }}
          >
            {scope.title}
          </div>
          <div className="text-[13px] leading-snug" style={{ color: 'var(--lk-text-secondary)' }}>
            {getDescription(slug)}
          </div>
        </div>
        <span
          className="inline-block shrink-0 rounded-full px-3 py-1 text-[11px] font-semibold"
          style={{ background: tag.bg, color: tag.color }}
        >
          {tag.label}
        </span>
      </div>

      {/* Meta */}
      <div
        className="flex gap-4 text-xs"
        style={{ color: 'var(--lk-text-secondary)', fontVariantNumeric: 'tabular-nums', lineHeight: '1.4' }}
      >
        <span>
          <strong className="font-semibold" style={{ color: 'var(--lk-text)', transition: 'color 0.2s ease' }}>
            {totalSubjects}
          </strong>{' '}
          subjects
        </span>
        <span>
          <strong className="font-semibold" style={{ color: 'var(--lk-text)', transition: 'color 0.2s ease' }}>
            {totalLessons}
          </strong>{' '}
          lessons
        </span>
        <span>{scope.created}</span>
      </div>

      {/* Progress row */}
      <div className="mt-auto flex items-center gap-3">
        <div className="flex-1">
          <ProgressBar percent={percent} />
        </div>
        <ProgressRing percent={percent} />
      </div>
    </article>
  );
}
