import { useParams, Link } from 'react-router-dom';
import { useScope, useLessons, useProgress } from '@/hooks/useApi';
import { ProgressBar } from '@/components/ProgressBar';
import { SubjectGroup } from '@/components/SubjectGroup';
import { programIcons } from '@/components/ProgramIcons';
import type { LessonRow } from '@/api/client';

/** Fallback generic icon for unknown programs */
const fallbackIcon = (
  <svg viewBox="0 0 48 48" fill="none" className="size-full" aria-hidden="true">
    <rect x="8" y="6" width="32" height="36" rx="4" stroke="var(--lk-accent)" strokeWidth="2" />
    <path d="M16 16h16M16 22h12M16 28h8" stroke="var(--lk-accent)" strokeWidth="1.5" strokeLinecap="round" />
  </svg>
);

function ErrorBanner({ message }: { message: string }) {
  return (
    <div
      className="mb-4 rounded-lg px-4 py-3 text-sm"
      role="alert"
      style={{
        background: 'rgba(234,179,8,0.1)',
        border: '1px solid rgba(234,179,8,0.2)',
        color: 'var(--lk-in-progress)',
        animation: 'fadeIn 0.25s cubic-bezier(0.22,1,0.36,1)',
      }}
    >
      {message}
    </div>
  );
}

function EmptySubjectsState() {
  return (
    <div
      className="flex flex-col items-center justify-center rounded-2xl px-8 py-12"
      style={{
        background: 'var(--lk-card)',
        border: '1px solid var(--lk-border)',
      }}
    >
      <p
        className="mb-1 text-sm font-semibold"
        style={{ color: 'var(--lk-text-secondary)', letterSpacing: '-0.3px' }}
      >
        暂无课程内容
      </p>
      <p
        className="text-xs"
        style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.6', opacity: 0.7 }}
      >
        课程大纲尚未配置科目与课时
      </p>
    </div>
  );
}

function BackLink() {
  return (
    <Link
      to="/"
      className="mb-4 inline-flex items-center gap-2 text-[13px] no-underline"
      style={{ color: 'var(--lk-text-secondary)', transition: 'color 0.2s ease' }}
      aria-label="返回教程列表"
      onMouseEnter={(e) => {
        (e.currentTarget as HTMLElement).style.color = 'var(--lk-accent)';
      }}
      onMouseLeave={(e) => {
        (e.currentTarget as HTMLElement).style.color = 'var(--lk-text-secondary)';
      }}
    >
      <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" aria-hidden="true">
        <path d="M15 18l-6-6 6-6" />
      </svg>
      返回列表
    </Link>
  );
}

function DetailSkeleton() {
  return (
    <div className="mx-auto max-w-[1200px] px-6 py-8 sm:px-10" role="status" aria-busy="true" aria-label="加载中">
      <div className="mb-4">
        <BackLink />
      </div>
      {/* Title skeleton */}
      <div className="mb-3 flex items-center gap-3">
        <div
          className="size-9 shrink-0 animate-pulse rounded-lg"
          style={{ background: 'var(--lk-border)' }}
        />
        <div
          className="h-7 w-48 animate-pulse rounded-lg"
          style={{ background: 'var(--lk-border)' }}
        />
      </div>
      {/* Progress bar skeleton */}
      <div
        className="mb-7 h-2 w-full animate-pulse rounded-full"
        style={{ background: 'var(--lk-border)' }}
      />
      {/* Subject group skeletons */}
      <div className="flex flex-col gap-3">
        {[1, 2, 3].map((i) => (
          <div
            key={i}
            className="animate-pulse rounded-xl"
            style={{
              background: 'var(--lk-card)',
              border: '1px solid var(--lk-border)',
              height: 64,
            }}
          />
        ))}
      </div>
    </div>
  );
}

export function ProgramDetail() {
  const { slug = '' } = useParams<{ slug: string }>();
  const { scope, loading: scopeLoading, error: scopeError } = useScope(slug);
  const { lessons, loading: lessonsLoading, error: lessonsError } = useLessons(slug);
  const { progress, loading: progressLoading, error: progressError } = useProgress(slug);

  const loading = scopeLoading || lessonsLoading || progressLoading;
  const errorMsg = scopeError || lessonsError || progressError;

  if (loading) {
    return <DetailSkeleton />;
  }

  if (!scope || !progress) {
    return (
      <div className="mx-auto max-w-[1200px] px-6 py-8 sm:px-10">
        <BackLink />
        <div
          className="mt-8 flex flex-col items-center justify-center rounded-2xl px-8 py-16"
          style={{
            background: 'var(--lk-card)',
            border: '1px solid var(--lk-border)',
          }}
        >
          <p
            className="mb-1 text-base font-semibold"
            style={{ color: 'var(--lk-text)', letterSpacing: '-0.3px' }}
          >
            未找到该教程
          </p>
          <p
            className="text-sm"
            style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.6' }}
          >
            请检查链接是否正确，或返回列表页
          </p>
        </div>
      </div>
    );
  }

  const rawPercent = progress.total > 0 ? (progress.completed / progress.total) * 100 : 0;
  const safePercent = Math.min(100, Math.max(0, isFinite(rawPercent) ? rawPercent : 0));

  // Group lessons by subject
  const lessonsBySubject: Record<string, LessonRow[]> = {};
  for (const l of lessons) {
    if (!lessonsBySubject[l.subject]) lessonsBySubject[l.subject] = [];
    lessonsBySubject[l.subject].push(l);
  }

  const subjects = scope.subjects ?? [];

  return (
    <main
      className="mx-auto max-w-[1200px] px-6 py-8 sm:px-10"
      style={{ animation: 'fadeIn 0.25s cubic-bezier(0.22,1,0.36,1)' }}
    >
      {/* Error banner */}
      {errorMsg && <ErrorBanner message={errorMsg} />}

      {/* Header */}
      <div className="mb-7">
        <BackLink />

        <div className="mb-3 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <div
            className="flex min-w-0 items-center gap-3 text-2xl font-bold"
            style={{ letterSpacing: '-0.5px', lineHeight: '1.2' }}
          >
            <span className="size-9 shrink-0">
              {programIcons[slug] ?? fallbackIcon}
            </span>
            <span className="min-w-0 truncate" title={scope.title}>
              {scope.title || '未命名教程'}
            </span>
          </div>
          <div
            className="shrink-0 whitespace-nowrap text-sm font-semibold"
            style={{
              color: 'var(--lk-text-secondary)',
              fontVariantNumeric: 'tabular-nums',
              lineHeight: '1.6',
            }}
          >
            进度{' '}
            <strong
              className="text-base"
              style={{ color: 'var(--lk-accent)', transition: 'color 0.2s ease', fontVariantNumeric: 'tabular-nums' }}
            >
              {progress.completed}
            </strong>
            /{progress.total}
            {scope.difficulty && (
              <span
                className="ml-3 rounded-md px-2 py-0.5 text-[11px] font-medium"
                style={{
                  background: 'rgba(59,130,246,0.1)',
                  color: 'var(--lk-accent)',
                }}
              >
                {scope.difficulty}
              </span>
            )}
          </div>
        </div>

        <div className="mt-1">
          <ProgressBar percent={safePercent} label={`${scope.title || '教程'} 总进度 ${progress.completed}/${progress.total}`} />
        </div>
      </div>

      {/* Subject groups */}
      {subjects.length === 0 ? (
        <EmptySubjectsState />
      ) : (
        <div className="flex flex-col gap-3">
          {subjects.map((subject, i) => (
            <SubjectGroup
              key={subject.slug}
              programSlug={slug}
              title={subject.title}
              lessons={lessonsBySubject[subject.slug] ?? []}
              totalLessons={subject.lessons?.length ?? 0}
              defaultOpen={i === 0}
            />
          ))}
        </div>
      )}
    </main>
  );
}
