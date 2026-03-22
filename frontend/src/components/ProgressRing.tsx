interface ProgressRingProps {
  percent: number;
  size?: number;
}

const CIRCUMFERENCE = 2 * Math.PI * 42; // r=42 in viewBox 100x100

export function ProgressRing({ percent, size = 56 }: ProgressRingProps) {
  const isComplete = percent >= 100;
  const offset = CIRCUMFERENCE * (1 - Math.min(100, Math.max(0, percent)) / 100);

  return (
    <div className="relative shrink-0" style={{ width: size, height: size }}>
      {/* SVG gradient definition */}
      <svg width="0" height="0" className="absolute">
        <defs>
          <linearGradient id="ringGradient" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="#6c5ce7" />
            <stop offset="100%" stopColor="#a29bfe" />
          </linearGradient>
        </defs>
      </svg>

      <svg
        viewBox="0 0 100 100"
        width={size}
        height={size}
        style={{ transform: 'rotate(-90deg)' }}
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
          stroke={isComplete ? 'var(--lk-completed)' : 'url(#ringGradient)'}
          strokeWidth="5"
          strokeLinecap="round"
          strokeDasharray={CIRCUMFERENCE}
          strokeDashoffset={offset}
          style={{ transition: 'stroke-dashoffset 0.6s cubic-bezier(0.22,1,0.36,1)' }}
        />
      </svg>

      <div
        className="absolute inset-0 flex items-center justify-center text-sm font-bold"
        style={{
          fontVariantNumeric: 'tabular-nums',
          color: isComplete
            ? 'var(--lk-completed)'
            : percent === 0
              ? 'var(--lk-text-secondary)'
              : 'var(--lk-text)',
        }}
      >
        {Math.round(percent)}
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
