interface ProgressBarProps {
  percent: number;
  height?: number;
}

export function ProgressBar({ percent, height = 8 }: ProgressBarProps) {
  const isComplete = percent >= 100;
  const isZero = percent <= 0;

  return (
    <div
      className="overflow-hidden"
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
          width: `${Math.min(100, Math.max(0, percent))}%`,
          background: isComplete
            ? 'linear-gradient(90deg, #10ac84, #2ecc71)'
            : 'linear-gradient(90deg, #6c5ce7, #a29bfe)',
          transition: 'width 0.3s cubic-bezier(0.22,1,0.36,1)',
          ...(isZero ? { opacity: 0 } : {}),
        }}
      />
    </div>
  );
}
