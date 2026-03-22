type Status = 'pending' | 'prepared' | 'in_progress' | 'completed';

const config: Record<Status, { bg: string; color: string; label: string }> = {
  completed: {
    bg: 'rgba(16,172,132,0.15)',
    color: 'var(--lk-completed)',
    label: '✓ 已完成',
  },
  in_progress: {
    bg: 'rgba(254,202,87,0.15)',
    color: 'var(--lk-in-progress)',
    label: '📖 学习中',
  },
  prepared: {
    bg: 'rgba(72,219,251,0.15)',
    color: 'var(--lk-prepared)',
    label: '已备课',
  },
  pending: {
    bg: 'rgba(120,120,160,0.12)',
    color: 'var(--lk-pending)',
    label: '待备课',
  },
};

export function StatusBadge({ status }: { status: Status }) {
  const c = config[status];
  return (
    <span
      className="inline-flex items-center gap-1 whitespace-nowrap rounded-md px-3 py-1 text-[11px] font-semibold"
      style={{
        background: c.bg,
        color: c.color,
        letterSpacing: '0.2px',
        transition: 'color 0.2s ease, background 0.2s ease',
        animation: status === 'in_progress' ? 'badge-breathe 2.5s ease-in-out infinite' : undefined,
      }}
    >
      {c.label}
    </span>
  );
}
