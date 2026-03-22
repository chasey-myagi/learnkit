const BASE = '/api';

export interface Program {
  slug: string;
  title: string;
  path: string;
}

export interface LessonRow {
  id: string;
  subject: string;
  lesson: string;
  title: string;
  status: 'pending' | 'prepared' | 'in_progress' | 'completed';
  file_path: string | null;
  prepared_at: string | null;
  started_at: string | null;
  completed_at: string | null;
}

export interface ScopeData {
  program: string;
  title: string;
  created: string;
  difficulty: string | null;
  subjects: {
    slug: string;
    title: string;
    lessons: {
      slug: string;
      title: string;
      sections: string[];
    }[];
  }[];
}

export interface ProgressData {
  lessons: Record<string, number>;
  total: number;
  completed: number;
  in_progress: number;
  prepared: number;
  pending: number;
}

export const api = {
  getPrograms: () =>
    fetch(`${BASE}/programs`).then((r) => r.json()) as Promise<Program[]>,
  getScope: (slug: string) =>
    fetch(`${BASE}/programs/${slug}/scope`).then((r) => r.json()) as Promise<ScopeData>,
  getLessons: (slug: string) =>
    fetch(`${BASE}/programs/${slug}/lessons`).then((r) => r.json()) as Promise<LessonRow[]>,
  getProgress: (slug: string) =>
    fetch(`${BASE}/programs/${slug}/progress`).then((r) => r.json()) as Promise<ProgressData>,
};
