interface ProgressRingProps {
  percent: number;
  size?: number;
  label?: string;
}

const CIRCUMFERENCE = 2 * Math.PI * 42; // r=42 in viewBox 100x100

export function ProgressRing({ percent, size = 56, label }: ProgressRingProps) {
  const safePercent = isFinite(percent) ? percent : 0;
  const clampedPercent = Math.min(100, Math.max(0, safePercent));
  const isComplete = clampedPercent >= 100;
  const offset = CIRCUMFERENCE * (1 - clampedPercent / 100);

  return (
    <div
      className="relative shrink-0"
      style={{ width: size, height: size }}
      role="progressbar"
      aria-valuenow={Math.round(clampedPercent)}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-label={label ?? `进度 ${Math.round(clampedPercent)}%`}
    >
      <svg
        viewBox="0 0 100 100"
        width={size}
        height={size}
        style={{ transform: 'rotate(-90deg)' }}
        aria-hidden="true"
      >
        <circle
          cx="50"
          cy="50"
          r="42"
          fill="none"
          stroke="var(--lk-border)"
          strokeWidth="5"
          style={{ transition: 'stroke 0.2s ease' }}
        />
        <circle
          cx="50"
          cy="50"
          r="42"
          fill="none"
          stroke={isComplete ? 'var(--lk-completed)' : 'var(--lk-accent)'}
          strokeWidth="5"
          strokeLinecap="round"
          strokeDasharray={CIRCUMFERENCE}
          strokeDashoffset={offset}
          style={{ transition: 'stroke-dashoffset 0.6s cubic-bezier(0.22,1,0.36,1)' }}
        />
      </svg>

      <div
        className="absolute inset-0 flex items-center justify-center text-sm font-bold"
        aria-hidden="true"
        style={{
          fontVariantNumeric: 'tabular-nums',
          color: isComplete
            ? 'var(--lk-completed)'
            : clampedPercent === 0
              ? 'var(--lk-text-secondary)'
              : 'var(--lk-text)',
        }}
      >
        {Math.round(clampedPercent)}
        <span
          className="text-[10px] font-normal"
          style={{ color: 'var(--lk-text-secondary)', transition: 'color 0.2s ease' }}
        >
          %
        </span>
      </div>
    </div>
  );
}
