interface ProgressBarProps {
  percent: number;
  height?: number;
  label?: string;
}

export function ProgressBar({ percent, height = 8, label }: ProgressBarProps) {
  const safePercent = isFinite(percent) ? percent : 0;
  const clampedPercent = Math.min(100, Math.max(0, safePercent));
  const isComplete = clampedPercent >= 100;
  const isZero = clampedPercent <= 0;

  return (
    <div
      className="overflow-hidden"
      role="progressbar"
      aria-valuenow={Math.round(clampedPercent)}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-label={label ?? `进度 ${Math.round(clampedPercent)}%`}
      style={{
        height,
        background: 'var(--lk-border)',
        borderRadius: height / 2,
        transition: 'background 0.2s ease',
      }}
    >
      <div
        style={{
          height: '100%',
          borderRadius: height / 2,
          width: `${clampedPercent}%`,
          background: isComplete
            ? 'var(--lk-completed)'
            : 'var(--lk-accent)',
          transition: 'width 0.3s cubic-bezier(0.22,1,0.36,1)',
          ...(isZero ? { opacity: 0 } : {}),
        }}
      />
    </div>
  );
}
