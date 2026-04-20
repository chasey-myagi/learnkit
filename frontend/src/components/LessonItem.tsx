import { StatusBadge } from './StatusBadge';
import type { LessonRow } from '@/api/client';

interface LessonItemProps {
  lesson: LessonRow;
  programSlug: string;
  isLast: boolean;
}

const statusColors: Record<string, string> = {
  completed: 'var(--lk-completed)',
  in_progress: 'var(--lk-in-progress)',
  prepared: 'var(--lk-prepared)',
  pending: 'var(--lk-pending)',
};

const statusLabels: Record<string, string> = {
  completed: '已完成',
  in_progress: '学习中',
  prepared: '已备课',
  pending: '待备课',
};

export function LessonItem({ lesson, programSlug, isLast }: LessonItemProps) {
  const isClickable = lesson.status === 'prepared' || lesson.status === 'in_progress' || lesson.status === 'completed';
  const handleClick = () => {
    if (isClickable) {
      window.location.href = `/lessons/${programSlug}/${lesson.subject}/${lesson.lesson}`;
    }
  };

  return (
    <div
      className={`group relative hover:bg-[var(--lk-accent-hover)] ${isClickable ? 'cursor-pointer' : 'cursor-default'}`}
      style={{
        padding: '10px 12px',
        minHeight: 44,
        marginBottom: isLast ? 0 : 8,
        borderRadius: 'var(--radius-sm)',
        transition: 'background 0.15s ease',
      }}
      role="button"
      tabIndex={0}
      aria-label={`${lesson.title || '未命名课时'} — ${statusLabels[lesson.status] ?? '未知状态'}`}
      onClick={handleClick}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          handleClick();
        }
      }}
    >
      {/* Timeline node dot */}
      <div
        className="absolute"
        style={{
          left: -23,
          top: 14,
          width: 10,
          height: 10,
          borderRadius: '50%',
          border: '2px solid var(--lk-card)',
          background: statusColors[lesson.status] ?? 'var(--lk-pending)',
          opacity: lesson.status === 'pending' ? 0.45 : 1,
          zIndex: 2,
          transition: 'background 0.2s ease, border-color 0.2s ease',
        }}
      />

      {/* Content */}
      <div className="flex items-center justify-between gap-3">
        <span
          className="min-w-0 truncate text-sm"
          style={{
            lineHeight: '1.6',
            fontWeight: lesson.status === 'in_progress' ? 600 : 400,
            color: lesson.status === 'pending' ? 'var(--lk-text-secondary)' : 'var(--lk-text)',
          }}
          title={lesson.title}
        >
          {lesson.title || '未命名课时'}
        </span>
        <div className="flex items-center gap-2 shrink-0">
          <StatusBadge status={lesson.status} />
          {isClickable && (
            <span
              className="text-xs opacity-0 -translate-x-0.5 transition-all group-hover:opacity-100 group-hover:translate-x-0"
              aria-hidden="true"
              style={{ color: 'var(--lk-text-secondary)' }}
            >
              &rsaquo;
            </span>
          )}
        </div>
      </div>

      {lesson.status === 'completed' && lesson.completed_at && (
        <div
          className="mt-1 text-xs"
          style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.6' }}
        >
          完成于 {lesson.completed_at}
        </div>
      )}
    </div>
  );
}
