import { usePrograms } from '@/hooks/useApi';
import { ProgramCard } from '@/components/ProgramCard';
import { mockScopes, mockProgress } from '@/api/mock';

export function ProgramList() {
  const { programs, loading } = usePrograms();

  if (loading) {
    return (
      <div className="mx-auto max-w-[1200px] px-10 py-8">
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
            <div
              key={i}
              className="h-[220px] animate-pulse rounded-2xl"
              style={{ background: 'var(--lk-card)', border: '1px solid var(--lk-border)' }}
            />
          ))}
        </div>
      </div>
    );
  }

  return (
    <main
      className="mx-auto max-w-[1200px] px-10 py-8"
      style={{ animation: 'fadeIn 0.25s cubic-bezier(.4,0,.2,1)' }}
    >
      <div className="mb-6 flex items-baseline justify-between">
        <h1
          className="text-[22px] font-bold"
          style={{ letterSpacing: '-0.5px', lineHeight: '1.2' }}
        >
          我的学习
        </h1>
        <span className="text-sm" style={{ color: 'var(--lk-text-secondary)', lineHeight: '1.5' }}>
          {programs.length} 个教程
        </span>
      </div>

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
    </main>
  );
}
