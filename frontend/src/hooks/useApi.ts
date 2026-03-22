import { useState, useEffect, useRef } from 'react';
import { api } from '@/api/client';
import type { Program, ScopeData, LessonRow, ProgressData } from '@/api/client';
import { mockPrograms, mockScopes, mockLessons, mockProgress } from '@/api/mock';

/** Whether to show a "using offline data" toast — set by hooks on fallback */
let _offlineNotified = false;

function notifyOfflineFallback(setError: (msg: string | null) => void) {
  if (!_offlineNotified) {
    _offlineNotified = true;
    setError('无法连接服务器，正在使用离线数据');
    // Auto-clear after 4 seconds
    setTimeout(() => setError(null), 4000);
  }
}

export function usePrograms() {
  const [programs, setPrograms] = useState<Program[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    api
      .getPrograms()
      .then((data) => {
        if (!cancelled) {
          setPrograms(Array.isArray(data) ? data : []);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setPrograms(mockPrograms);
          notifyOfflineFallback(setError);
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, []);

  return { programs, loading, error };
}

export function useScope(slug: string) {
  const [scope, setScope] = useState<ScopeData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const prevSlug = useRef(slug);

  useEffect(() => {
    let cancelled = false;
    prevSlug.current = slug;

    setLoading(true);
    setError(null);

    api
      .getScope(slug)
      .then((data) => {
        if (!cancelled) setScope(data ?? null);
      })
      .catch(() => {
        if (!cancelled) {
          const fallback = mockScopes[slug] ?? null;
          setScope(fallback);
          if (!fallback) {
            setError('无法加载课程大纲');
          } else {
            notifyOfflineFallback(setError);
          }
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [slug]);

  return { scope, loading, error };
}

export function useLessons(slug: string) {
  const [lessons, setLessons] = useState<LessonRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const prevSlug = useRef(slug);

  useEffect(() => {
    let cancelled = false;
    prevSlug.current = slug;

    setLoading(true);
    setError(null);

    api
      .getLessons(slug)
      .then((data) => {
        if (!cancelled) setLessons(Array.isArray(data) ? data : []);
      })
      .catch(() => {
        if (!cancelled) {
          setLessons(mockLessons[slug] ?? []);
          notifyOfflineFallback(setError);
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [slug]);

  return { lessons, loading, error };
}

export function useProgress(slug: string) {
  const [progress, setProgress] = useState<ProgressData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const prevSlug = useRef(slug);

  useEffect(() => {
    let cancelled = false;
    prevSlug.current = slug;

    setLoading(true);
    setError(null);

    api
      .getProgress(slug)
      .then((data) => {
        if (!cancelled) setProgress(data ?? null);
      })
      .catch(() => {
        if (!cancelled) {
          setProgress(mockProgress[slug] ?? null);
          notifyOfflineFallback(setError);
        }
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [slug]);

  return { progress, loading, error };
}
