const BASE = '/api';
const TIMEOUT_MS = 15_000;

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
  lessons: {
    pending?: number;
    prepared?: number;
    in_progress?: number;
    completed?: number;
  };
  sections: {
    read: number;
    total: number;
  };
}

class ApiError extends Error {
  status?: number;

  constructor(message: string, status?: number) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
  }
}

async function fetchWithTimeout(url: string, timeoutMs = TIMEOUT_MS): Promise<Response> {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);

  try {
    const response = await fetch(url, { signal: controller.signal });
    if (!response.ok) {
      throw new ApiError(
        `HTTP ${response.status}: ${response.statusText}`,
        response.status,
      );
    }
    return response;
  } catch (err) {
    if (err instanceof ApiError) throw err;
    if (err instanceof DOMException && err.name === 'AbortError') {
      throw new ApiError('请求超时，请检查网络连接');
    }
    throw new ApiError('网络连接失败，请稍后重试');
  } finally {
    clearTimeout(timer);
  }
}

async function fetchJson<T>(url: string): Promise<T> {
  const response = await fetchWithTimeout(url);
  try {
    return (await response.json()) as T;
  } catch {
    throw new ApiError('数据解析失败，服务端返回了无效的响应');
  }
}

export const api = {
  getPrograms: () => fetchJson<Program[]>(`${BASE}/programs`),
  getScope: (slug: string) => fetchJson<ScopeData>(`${BASE}/programs/${slug}/scope`),
  getLessons: (slug: string) => fetchJson<LessonRow[]>(`${BASE}/programs/${slug}/lessons`),
  getProgress: (slug: string) => fetchJson<ProgressData>(`${BASE}/programs/${slug}/progress`),
};
