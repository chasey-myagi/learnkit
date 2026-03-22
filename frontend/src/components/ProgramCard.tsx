import { useNavigate } from 'react-router-dom';
import { ProgressBar } from './ProgressBar';
import { ProgressRing } from './ProgressRing';
import { programIcons, bgIcons } from './ProgramIcons';
import type { ProgressData, ScopeData } from '@/api/client';

interface ProgramCardProps {
  slug: string;
  scope: ScopeData;
  progress: ProgressData;
}

function getStatusTag(progress: ProgressData) {
  const completed = progress?.lessons?.completed ?? 0;
  const in_progress = progress?.lessons?.in_progress ?? 0;
  const prepared = progress?.lessons?.prepared ?? 0;
  const pending = progress?.lessons?.pending ?? 0;
  const total = completed + in_progress + prepared + pending;

  if (total > 0 && completed === total) {
    return { label: '已完成', bg: 'rgba(34,197,94,0.1)', color: 'var(--lk-completed)' };
  }
  if (in_progress > 0 || completed > 0) {
    return { label: '学习中', bg: 'rgba(59,130,246,0.1)', color: 'var(--lk-accent)' };
  }
  if (prepared > 0) {
    return { label: '已备课', bg: 'rgba(6,182,212,0.1)', color: 'var(--lk-prepared)' };
  }
  return { label: '未开始', bg: 'rgba(113,113,122,0.1)', color: 'var(--lk-pending)' };
}

// Description derived from scope subjects
function getDescription(scope: ScopeData): string {
  const subjects = scope.subjects ?? [];
  if (subjects.length === 0) return '';
  return subjects.map(s => s.title).join(' · ');
}

export function ProgramCard({ slug, scope, progress }: ProgramCardProps) {
  const navigate = useNavigate();
  const completed = progress?.lessons?.completed ?? 0;
  const total = completed + (progress?.lessons?.in_progress ?? 0) + (progress?.lessons?.prepared ?? 0) + (progress?.lessons?.pending ?? 0);
  const rawPercent = total > 0 ? (completed / total) * 100 : 0;
  const percent = Math.min(100, Math.max(0, isFinite(rawPercent) ? rawPercent : 0));
  const isComplete = percent >= 100;
  const tag = getStatusTag(progress);
  const subjects = scope.subjects ?? [];
  const totalSubjects = subjects.length;
  const totalLessons = subjects.reduce((sum, s) => sum + (s.lessons?.length ?? 0), 0);

  return (
    <article
      className="relative flex cursor-pointer flex-col gap-4 overflow-hidden rounded-2xl border p-6 transition-all hover:-translate-y-0.5 hover:shadow-lg"
      style={{
        background: 'var(--lk-card)',
        borderColor: isComplete ? 'rgba(34,197,94,0.3)' : 'var(--lk-border)',
        '--hover-border': isComplete ? 'var(--lk-completed)' : 'var(--lk-accent)',
      } as React.CSSProperties}
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
        e.currentTarget.style.borderColor = isComplete ? 'var(--lk-completed)' : 'var(--lk-accent)';
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.borderColor = isComplete ? 'rgba(34,197,94,0.3)' : 'var(--lk-border)';
      }}
    >
      {/* Background decorative icon */}
      <div
        className="pointer-events-none absolute select-none"
        aria-hidden="true"
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
            className="truncate text-[17px] font-bold"
            style={{ letterSpacing: '-0.3px', lineHeight: '1.3', color: 'var(--lk-text)' }}
            title={scope.title}
          >
            {scope.title || '未命名教程'}
          </div>
          <div
            className="truncate text-[13px]"
            style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.6' }}
            title={getDescription(scope)}
          >
            {getDescription(scope)}
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
        style={{ color: 'var(--lk-text-secondary)', fontVariantNumeric: 'tabular-nums', lineHeight: '1.6' }}
      >
        <span>
          <strong className="font-semibold" style={{ color: 'var(--lk-text)', transition: 'color 0.2s ease' }}>
            {totalSubjects}
          </strong>{' '}
          个科目
        </span>
        <span>
          <strong className="font-semibold" style={{ color: 'var(--lk-text)', transition: 'color 0.2s ease' }}>
            {totalLessons}
          </strong>{' '}
          节课
        </span>
        <span>{scope.created}</span>
      </div>

      {/* Progress row */}
      <div className="mt-auto flex items-center gap-3">
        <div className="flex-1">
          <ProgressBar percent={percent} label={`${scope.title} 进度 ${Math.round(percent)}%`} />
        </div>
        <ProgressRing percent={percent} label={`${scope.title} 进度 ${Math.round(percent)}%`} />
      </div>
    </article>
  );
}
