import { useState, useId } from 'react';
import { LessonItem } from './LessonItem';
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
    <svg className="size-[22px] shrink-0" viewBox="0 0 24 24" fill="none" stroke="var(--lk-accent)" strokeWidth="1.8" aria-hidden="true">
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
      aria-hidden="true"
      style={{
        color: 'var(--lk-text-secondary)',
        transform: open ? 'rotate(180deg)' : 'rotate(0deg)',
        transition: 'transform 250ms cubic-bezier(0.22,1,0.36,1), color 0.2s ease',
      }}
    >
      <polyline points="6 9 12 15 18 9" />
    </svg>
  );
}

export function SubjectGroup({ programSlug, title, lessons, totalLessons, defaultOpen = false }: SubjectGroupProps) {
  const [open, setOpen] = useState(defaultOpen);
  const panelId = useId();
  const headerId = useId();
  const completedCount = lessons.filter((l) => l.status === 'completed').length;
  const rawPercent = totalLessons > 0 ? (completedCount / totalLessons) * 100 : 0;
  const percent = Math.min(100, Math.max(0, isFinite(rawPercent) ? rawPercent : 0));

  // Each lesson item is roughly 50px, with 8px margin
  const completedLineHeight = completedCount > 0 ? completedCount * 50 + (completedCount - 1) * 8 : 0;

  return (
    <div
      className="overflow-hidden"
      style={{
        border: '1px solid var(--lk-border)',
        borderRadius: 'var(--radius)',
        background: 'var(--lk-card)',
        transition: 'border-color 0.2s ease, background 0.2s ease',
        borderLeft: open ? '3px solid var(--lk-accent)' : '3px solid transparent',
      }}
      onMouseEnter={(e) => {
        if (!open) e.currentTarget.style.borderColor = 'rgba(59,130,246,0.3)';
      }}
      onMouseLeave={(e) => {
        if (!open) {
          e.currentTarget.style.borderColor = 'var(--lk-border)';
        }
      }}
    >
      {/* Header */}
      <div
        className="flex cursor-pointer select-none items-center gap-3 px-5 py-4 transition-colors hover:bg-[var(--lk-accent-hover)]"
        role="button"
        tabIndex={0}
        id={headerId}
        aria-expanded={open}
        aria-controls={panelId}
        aria-label={`展开/折叠${title}`}
        onClick={() => setOpen(!open)}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            setOpen(!open);
          }
        }}
      >
        <h3
          className="flex min-w-0 flex-1 items-center gap-2 text-base font-bold"
          style={{ letterSpacing: '-0.3px', lineHeight: '1.3' }}
        >
          <SubjectIcon />
          <span className="min-w-0 truncate" title={title}>
            {title || '未命名科目'}
          </span>
        </h3>

        {/* Progress bar + text */}
        <div className="flex shrink-0 items-center gap-2" style={{ maxWidth: 160 }}>
          <div
            className="flex-1 overflow-hidden"
            role="progressbar"
            aria-valuenow={Math.round(percent)}
            aria-valuemin={0}
            aria-valuemax={100}
            aria-label={`${title} 进度 ${completedCount}/${totalLessons}`}
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
                transition: 'width 0.3s cubic-bezier(0.22,1,0.36,1)',
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
          id={panelId}
          role="region"
          aria-labelledby={headerId}
          className="px-6 pb-6 pt-4"
          style={{
            background: 'var(--lk-bg)',
            borderTop: '1px solid var(--lk-border)',
            transition: 'background 0.2s ease, border-color 0.2s ease',
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
                  transition: 'height 0.3s cubic-bezier(0.22,1,0.36,1)',
                }}
              />
            )}

            {/* Lesson items */}
            {lessons.length === 0 ? (
              <div
                className="py-4 text-center text-xs"
                style={{ color: 'var(--lk-text-secondary)', opacity: 0.7 }}
              >
                暂无课时数据
              </div>
            ) : (
              lessons.map((lesson, i) => (
                <LessonItem
                  key={lesson.id}
                  lesson={lesson}
                  programSlug={programSlug}
                  isLast={i === lessons.length - 1}
                />
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}
