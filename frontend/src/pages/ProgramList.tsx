import { usePrograms } from '@/hooks/useApi';
import { ProgramCard } from '@/components/ProgramCard';
import { mockScopes, mockProgress } from '@/api/mock';

function SkeletonCard() {
  return (
    <div
      className="flex flex-col gap-4 rounded-2xl p-6"
      style={{
        background: 'var(--lk-card)',
        border: '1px solid var(--lk-border)',
        height: 220,
      }}
    >
      <div className="flex items-center gap-3">
        <div
          className="size-11 shrink-0 animate-pulse rounded-xl"
          style={{ background: 'var(--lk-border)' }}
        />
        <div className="flex flex-1 flex-col gap-2">
          <div
            className="h-4 w-32 animate-pulse rounded"
            style={{ background: 'var(--lk-border)' }}
          />
          <div
            className="h-3 w-48 animate-pulse rounded"
            style={{ background: 'var(--lk-border)', opacity: 0.6 }}
          />
        </div>
      </div>
      <div className="flex gap-4">
        <div
          className="h-3 w-16 animate-pulse rounded"
          style={{ background: 'var(--lk-border)', opacity: 0.5 }}
        />
        <div
          className="h-3 w-16 animate-pulse rounded"
          style={{ background: 'var(--lk-border)', opacity: 0.5 }}
        />
      </div>
      <div className="mt-auto flex items-center gap-3">
        <div
          className="h-2 flex-1 animate-pulse rounded-full"
          style={{ background: 'var(--lk-border)' }}
        />
        <div
          className="size-14 shrink-0 animate-pulse rounded-full"
          style={{ background: 'var(--lk-border)', opacity: 0.4 }}
        />
      </div>
    </div>
  );
}

function EmptyState() {
  return (
    <div
      className="flex flex-col items-center justify-center rounded-2xl px-8 py-16"
      style={{
        background: 'var(--lk-card)',
        border: '1px solid var(--lk-border)',
      }}
    >
      <svg
        className="mb-4 size-12"
        viewBox="0 0 48 48"
        fill="none"
        stroke="var(--lk-text-secondary)"
        strokeWidth="1.5"
        strokeLinecap="round"
      >
        <rect x="8" y="6" width="32" height="36" rx="4" />
        <path d="M16 16h16M16 22h12M16 28h8" />
      </svg>
      <p
        className="mb-1 text-base font-semibold"
        style={{ color: 'var(--lk-text)', letterSpacing: '-0.3px' }}
      >
        还没有学习计划
      </p>
      <p
        className="text-sm"
        style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.6' }}
      >
        通过 CLI 创建你的第一个教程，开始学习之旅
      </p>
    </div>
  );
}

export function ProgramList() {
  const { programs, loading } = usePrograms();

  if (loading) {
    return (
      <div className="mx-auto max-w-[1200px] px-6 py-8 sm:px-10">
        <div className="flex items-baseline justify-between" style={{ marginBottom: 24 }}>
          <h1
            className="text-[22px] font-bold"
            style={{ letterSpacing: '-0.5px', lineHeight: '1.2' }}
          >
            我的学习
          </h1>
        </div>
        <div className="grid grid-cols-1 gap-5 md:grid-cols-2">
          {[1, 2, 3].map((i) => (
            <SkeletonCard key={i} />
          ))}
        </div>
      </div>
    );
  }

  return (
    <main
      className="mx-auto max-w-[1200px] px-6 py-8 sm:px-10"
      style={{ animation: 'fadeIn 0.25s cubic-bezier(0.22,1,0.36,1)' }}
    >
      <div className="mb-6 flex items-baseline justify-between">
        <h1
          className="text-[22px] font-bold"
          style={{ letterSpacing: '-0.5px', lineHeight: '1.2' }}
        >
          我的学习
        </h1>
        {programs.length > 0 && (
          <span
            className="text-sm"
            style={{
              color: 'var(--lk-text-secondary)',
              lineHeight: '1.6',
              fontVariantNumeric: 'tabular-nums',
            }}
          >
            {programs.length} 个教程
          </span>
        )}
      </div>

      {programs.length === 0 ? (
        <EmptyState />
      ) : (
        <div className="grid grid-cols-1 gap-5 md:grid-cols-2">
          {programs.map((p) => {
            const scope = mockScopes[p.slug];
            const progress = mockProgress[p.slug];
            if (!scope || !progress) return null;
            return (
              <ProgramCard key={p.slug} slug={p.slug} scope={scope} progress={progress} />
            );
          })}
        </div>
      )}
    </main>
  );
}
