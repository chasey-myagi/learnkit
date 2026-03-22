type Status = 'pending' | 'prepared' | 'in_progress' | 'completed';

const config: Record<Status, { bg: string; color: string; label: string }> = {
  completed: {
    bg: 'rgba(34,197,94,0.12)',
    color: 'var(--lk-completed)',
    label: '已完成',
  },
  in_progress: {
    bg: 'rgba(234,179,8,0.12)',
    color: 'var(--lk-in-progress)',
    label: '学习中',
  },
  prepared: {
    bg: 'rgba(6,182,212,0.12)',
    color: 'var(--lk-prepared)',
    label: '已备课',
  },
  pending: {
    bg: 'rgba(113,113,122,0.1)',
    color: 'var(--lk-pending)',
    label: '待备课',
  },
};

export function StatusBadge({ status }: { status: Status }) {
  const c = config[status] ?? config.pending;
  return (
    <span
      className="inline-flex items-center gap-1 whitespace-nowrap rounded-md px-3 py-1 text-[12px] font-medium"
      style={{
        background: c.bg,
        color: c.color,
        letterSpacing: '0.2px',
      }}
    >
      {c.label}
    </span>
  );
}
