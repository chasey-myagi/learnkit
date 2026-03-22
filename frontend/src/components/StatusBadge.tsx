type Status = 'pending' | 'prepared' | 'in_progress' | 'completed';

const config: Record<Status, { bg: string; color: string; label: string; dot: string }> = {
  completed: {
    bg: 'rgba(34,197,94,0.08)',
    color: 'var(--lk-completed)',
    label: '已完成',
    dot: 'var(--lk-completed)',
  },
  in_progress: {
    bg: 'rgba(234,179,8,0.08)',
    color: 'var(--lk-in-progress)',
    label: '学习中',
    dot: 'var(--lk-in-progress)',
  },
  prepared: {
    bg: 'rgba(59,130,246,0.08)',
    color: 'var(--lk-accent)',
    label: '已备课',
    dot: 'var(--lk-accent)',
  },
  pending: {
    bg: 'transparent',
    color: 'var(--lk-text-secondary)',
    label: '待备课',
    dot: 'var(--lk-text-secondary)',
  },
};

export function StatusBadge({ status }: { status: Status }) {
  const c = config[status] ?? config.pending;
  return (
    <span
      className="inline-flex items-center gap-1.5 whitespace-nowrap text-[11px] font-medium"
      style={{
        color: c.color,
        opacity: status === 'pending' ? 0.6 : 1,
      }}
    >
      <span
        style={{
          width: 6,
          height: 6,
          borderRadius: '50%',
          background: c.dot,
          flexShrink: 0,
        }}
      />
      {c.label}
    </span>
  );
}
