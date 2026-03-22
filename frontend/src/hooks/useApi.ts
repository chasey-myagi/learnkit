import { useState, useEffect } from 'react';
import { api } from '@/api/client';
import type { Program, ScopeData, LessonRow, ProgressData } from '@/api/client';
import { mockPrograms, mockScopes, mockLessons, mockProgress } from '@/api/mock';

export function usePrograms() {
  const [programs, setPrograms] = useState<Program[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    api
      .getPrograms()
      .then(setPrograms)
      .catch(() => {
        setPrograms(mockPrograms);
        setError(null); // fallback to mock data silently
      })
      .finally(() => setLoading(false));
  }, []);

  return { programs, loading, error };
}

export function useScope(slug: string) {
  const [scope, setScope] = useState<ScopeData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    api
      .getScope(slug)
      .then(setScope)
      .catch(() => {
        setScope(mockScopes[slug] ?? null);
      })
      .finally(() => setLoading(false));
  }, [slug]);

  return { scope, loading, error };
}

export function useLessons(slug: string) {
  const [lessons, setLessons] = useState<LessonRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    api
      .getLessons(slug)
      .then(setLessons)
      .catch(() => {
        setLessons(mockLessons[slug] ?? []);
      })
      .finally(() => setLoading(false));
  }, [slug]);

  return { lessons, loading, error };
}

export function useProgress(slug: string) {
  const [progress, setProgress] = useState<ProgressData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    api
      .getProgress(slug)
      .then(setProgress)
      .catch(() => {
        setProgress(mockProgress[slug] ?? null);
      })
      .finally(() => setLoading(false));
  }, [slug]);

  return { progress, loading, error };
}
