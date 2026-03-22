import { useState } from 'react';
import { StatusBadge } from './StatusBadge';
import type { LessonRow } from '@/api/client';

interface SubjectGroupProps {
  programSlug: string;
  title: string;
  lessons: LessonRow[];
  totalLessons: number;
  defaultOpen?: boolean;
}

function SubjectIcon() {
  return (
    <svg className="size-[22px] shrink-0" viewBox="0 0 24 24" fill="none" stroke="var(--lk-accent)" strokeWidth="1.8">
      <path d="M4 6h10v14H4z" rx="2" />
      <path d="M7 10h4M7 13h4M7 16h2" strokeLinecap="round" />
      <path d="M10 8h10v10H10" rx="2" opacity=".5" />
    </svg>
  );
}

function ChevronIcon({ open }: { open: boolean }) {
  return (
    <svg
      className="size-5 shrink-0"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      style={{
        color: 'var(--lk-text-secondary)',
        transform: open ? 'rotate(180deg)' : 'rotate(0deg)',
        transition: 'transform 0.25s cubic-bezier(.4,0,.2,1), color 0.2s ease',
      }}
    >
      <polyline points="6 9 12 15 18 9" />
    </svg>
  );
}

export function SubjectGroup({ programSlug, title, lessons, totalLessons, defaultOpen = false }: SubjectGroupProps) {
  const [open, setOpen] = useState(defaultOpen);
  const completedCount = lessons.filter((l) => l.status === 'completed').length;
  const percent = totalLessons > 0 ? (completedCount / totalLessons) * 100 : 0;

  // Calculate completed line height for timeline
  const completedLessons = lessons.filter((l) => l.status === 'completed').length;
  // Each lesson item is roughly 50px, with 8px margin
  const completedLineHeight = completedLessons > 0 ? completedLessons * 50 + (completedLessons - 1) * 8 : 0;

  return (
    <div
      className="overflow-hidden"
      style={{
        border: '1px solid var(--lk-border)',
        borderRadius: 'var(--radius)',
        background: 'var(--lk-card)',
        transition: 'border-color 0.2s ease, background 0.2s ease',
        borderLeft: open ? '3px solid var(--lk-accent)' : '1px solid var(--lk-border)',
      }}
      onMouseEnter={(e) => {
        if (!open) e.currentTarget.style.borderColor = 'rgba(108,92,231,0.3)';
      }}
      onMouseLeave={(e) => {
        if (!open) e.currentTarget.style.borderColor = 'var(--lk-border)';
      }}
    >
      {/* Header */}
      <div
        className="flex cursor-pointer select-none items-center gap-3 px-5 py-4"
        role="button"
        tabIndex={0}
        aria-expanded={open}
        aria-label={`展开/折叠${title}`}
        onClick={() => setOpen(!open)}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            setOpen(!open);
          }
        }}
        style={{ transition: 'background 0.2s ease' }}
        onMouseEnter={(e) => {
          e.currentTarget.style.background = 'var(--lk-accent-glow)';
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.background = 'transparent';
        }}
      >
        <h3
          className="flex flex-1 items-center gap-2 text-base font-bold"
          style={{ letterSpacing: '-0.3px', lineHeight: '1.3' }}
        >
          <SubjectIcon />
          {title}
        </h3>

        {/* Progress bar + text */}
        <div className="flex shrink-0 items-center gap-2" style={{ maxWidth: 160 }}>
          <div
            className="flex-1 overflow-hidden"
            style={{
              height: 4,
              background: 'var(--lk-border)',
              borderRadius: 2,
              minWidth: 80,
              transition: 'background 0.2s ease',
            }}
          >
            <div
              style={{
                height: '100%',
                background: 'var(--lk-accent)',
                borderRadius: 2,
                width: `${percent}%`,
                transition: 'width 0.3s cubic-bezier(.4,0,.2,1)',
              }}
            />
          </div>
          <span
            className="whitespace-nowrap text-xs font-medium"
            style={{ color: 'var(--lk-text-secondary)', fontVariantNumeric: 'tabular-nums' }}
          >
            {completedCount}/{totalLessons}
          </span>
        </div>

        <ChevronIcon open={open} />
      </div>

      {/* Body */}
      {open && (
        <div
          className="px-6 pb-6 pt-4"
          style={{
            background: 'var(--lk-bg)',
            borderTop: '1px solid var(--lk-border)',
            transition: 'background 0.3s ease, border-color 0.2s ease',
          }}
        >
          <div className="relative" style={{ paddingLeft: 28 }}>
            {/* Timeline line */}
            <div
              className="absolute"
              style={{
                left: 7,
                top: 8,
                bottom: 8,
                width: 2,
                background: 'var(--lk-border)',
                transition: 'background 0.2s ease',
              }}
            />

            {/* Completed segment overlay */}
            {completedLineHeight > 0 && (
              <div
                className="absolute"
                style={{
                  left: 7,
                  top: 8,
                  width: 2,
                  height: completedLineHeight,
                  background: 'var(--lk-completed)',
                  borderRadius: 1,
                  transition: 'height 0.3s cubic-bezier(.4,0,.2,1)',
                }}
              />
            )}

            {/* Lesson items */}
            {lessons.map((lesson, i) => (
              <LessonItem
                key={lesson.id}
                lesson={lesson}
                programSlug={programSlug}
                isLast={i === lessons.length - 1}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function LessonItem({
  lesson,
  programSlug,
  isLast,
}: {
  lesson: LessonRow;
  programSlug: string;
  isLast: boolean;
}) {
  const statusColors: Record<string, string> = {
    completed: 'var(--lk-completed)',
    in_progress: 'var(--lk-in-progress)',
    prepared: 'var(--lk-prepared)',
    pending: 'var(--lk-pending)',
  };

  const statusShadows: Record<string, string> = {
    completed: '0 0 0 3px rgba(16,172,132,0.2)',
    in_progress: '0 0 0 3px rgba(254,202,87,0.2)',
    prepared: '0 0 0 3px rgba(72,219,251,0.2)',
    pending: 'none',
  };

  const handleClick = () => {
    if (lesson.file_path) {
      window.location.href = `/lessons/${programSlug}/${lesson.subject}/${lesson.lesson}.html`;
    }
  };

  return (
    <div
      className="group relative cursor-pointer"
      style={{
        padding: '8px 12px',
        marginBottom: isLast ? 0 : 8,
        borderRadius: 'var(--radius-sm)',
        transition: 'background 0.2s ease',
      }}
      role="button"
      tabIndex={0}
      onClick={handleClick}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          handleClick();
        }
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.background = 'var(--lk-accent-glow)';
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.background = 'transparent';
      }}
    >
      {/* Timeline node dot */}
      <div
        className="absolute"
        style={{
          left: -23,
          top: 12,
          width: 14,
          height: 14,
          borderRadius: '50%',
          border: '2.5px solid var(--lk-card)',
          background: statusColors[lesson.status],
          boxShadow: statusShadows[lesson.status],
          opacity: lesson.status === 'pending' ? 0.6 : 1,
          zIndex: 2,
          transition: 'background 0.2s ease, box-shadow 0.2s ease, border-color 0.2s ease',
          animation: lesson.status === 'in_progress' ? 'node-pulse 2s infinite' : undefined,
        }}
      />

      {/* Hover arrow */}
      <div
        className="pointer-events-none absolute text-sm opacity-0 transition-all"
        style={{
          right: 12,
          top: '50%',
          transform: 'translateX(4px) translateY(-50%)',
          color: 'var(--lk-accent)',
        }}
      >
        <span className="block transition-all duration-200 group-hover:translate-x-0 group-hover:opacity-100"
          style={{ opacity: 0, transform: 'translateX(4px)' }}
        >
          →
        </span>
      </div>

      {/* Content */}
      <div className="flex items-center justify-between gap-3">
        <span className="text-sm font-semibold" style={{ lineHeight: '1.5' }}>
          {lesson.title}
        </span>
        <StatusBadge status={lesson.status} />
      </div>

      {lesson.status !== 'pending' && (
        <div
          className="mt-1 flex flex-wrap items-center gap-2 text-xs"
          style={{ color: 'var(--lk-text-secondary)' }}
        >
          {lesson.status === 'in_progress' && (
            <span>学习中</span>
          )}
          {lesson.status === 'completed' && lesson.completed_at && (
            <span>完成于 {lesson.completed_at}</span>
          )}
          {lesson.status === 'prepared' && (
            <span>已就绪</span>
          )}
        </div>
      )}
    </div>
  );
}
