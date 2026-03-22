import { useParams, Link } from 'react-router-dom';
import { useScope, useLessons, useProgress } from '@/hooks/useApi';
import { ProgressBar } from '@/components/ProgressBar';
import { SubjectGroup } from '@/components/SubjectGroup';
import type { LessonRow } from '@/api/client';

/** SVG icons per program slug */
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

export function ProgramDetail() {
  const { slug = '' } = useParams<{ slug: string }>();
  const { scope, loading: scopeLoading } = useScope(slug);
  const { lessons, loading: lessonsLoading } = useLessons(slug);
  const { progress, loading: progressLoading } = useProgress(slug);

  const loading = scopeLoading || lessonsLoading || progressLoading;

  if (loading) {
    return (
      <div className="mx-auto max-w-[1200px] px-10 py-8">
        <div className="mb-4">
          <Link
            to="/"
            className="inline-flex items-center gap-2 text-[13px] no-underline"
            style={{ color: 'var(--lk-text-secondary)', transition: 'color 0.2s ease' }}
          >
            <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
              <path d="M15 18l-6-6 6-6" />
            </svg>
            返回列表
          </Link>
        </div>
        <div
          className="h-16 animate-pulse rounded-xl"
          style={{ background: 'var(--lk-card)', border: '1px solid var(--lk-border)' }}
        />
        <div className="mt-4 flex flex-col gap-3">
          {[1, 2, 3].map((i) => (
            <div
              key={i}
              className="h-16 animate-pulse rounded-xl"
              style={{ background: 'var(--lk-card)', border: '1px solid var(--lk-border)' }}
            />
          ))}
        </div>
      </div>
    );
  }

  if (!scope || !progress) {
    return (
      <div className="mx-auto max-w-[1200px] px-10 py-8">
        <Link
          to="/"
          className="inline-flex items-center gap-2 text-[13px] no-underline"
          style={{ color: 'var(--lk-text-secondary)' }}
        >
          <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
            <path d="M15 18l-6-6 6-6" />
          </svg>
          返回列表
        </Link>
        <p className="mt-8 text-center" style={{ color: 'var(--lk-text-secondary)' }}>
          未找到该教程
        </p>
      </div>
    );
  }

  const percent = progress.total > 0 ? (progress.completed / progress.total) * 100 : 0;

  // Group lessons by subject
  const lessonsBySubject: Record<string, LessonRow[]> = {};
  for (const l of lessons) {
    if (!lessonsBySubject[l.subject]) lessonsBySubject[l.subject] = [];
    lessonsBySubject[l.subject].push(l);
  }

  return (
    <main
      className="mx-auto max-w-[1200px] px-10 py-8"
      style={{ animation: 'fadeIn 0.25s cubic-bezier(.4,0,.2,1)' }}
    >
      {/* Header */}
      <div className="mb-7">
        <Link
          to="/"
          className="mb-4 inline-flex items-center gap-2 text-[13px] no-underline"
          style={{ color: 'var(--lk-text-secondary)', transition: 'color 0.2s ease' }}
          onMouseEnter={(e) => {
            (e.currentTarget as HTMLElement).style.color = 'var(--lk-accent)';
          }}
          onMouseLeave={(e) => {
            (e.currentTarget as HTMLElement).style.color = 'var(--lk-text-secondary)';
          }}
        >
          <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round">
            <path d="M15 18l-6-6 6-6" />
          </svg>
          返回列表
        </Link>

        <div className="mb-3 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <div
            className="flex items-center gap-3 text-2xl font-bold"
            style={{ letterSpacing: '-0.5px', lineHeight: '1.2' }}
          >
            <span className="size-9">
              {programIcons[slug] ?? programIcons['game-dev']}
            </span>
            {scope.title}
          </div>
          <div
            className="shrink-0 whitespace-nowrap text-sm font-semibold"
            style={{ color: 'var(--lk-text-secondary)', fontVariantNumeric: 'tabular-nums' }}
          >
            进度{' '}
            <strong
              className="text-base"
              style={{ color: 'var(--lk-accent)', transition: 'color 0.2s ease', fontVariantNumeric: 'tabular-nums' }}
            >
              {progress.completed}
            </strong>
            /{progress.total}
          </div>
        </div>

        <div className="mt-1">
          <ProgressBar percent={percent} />
        </div>
      </div>

      {/* Subject groups */}
      <div className="flex flex-col gap-3">
        {scope.subjects.map((subject, i) => (
          <SubjectGroup
            key={subject.slug}
            programSlug={slug}
            title={subject.title}
            lessons={lessonsBySubject[subject.slug] ?? []}
            totalLessons={subject.lessons.length}
            defaultOpen={i === 0}
          />
        ))}
      </div>
    </main>
  );
}
