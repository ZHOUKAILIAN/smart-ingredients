import Taro from '@tarojs/taro';

declare const __API_BASE__: string | undefined;

const LOCAL_API_BASE = 'http://127.0.0.1:3000';

const normalizeBase = (value: unknown): string => {
  if (typeof value !== 'string') {
    return '';
  }
  const trimmed = value.trim();
  if (!trimmed) {
    return '';
  }
  return trimmed.replace(/\/+$/, '');
};

const resolveApiBase = () => {
  const defineBase = typeof __API_BASE__ === 'string' ? __API_BASE__ : '';

  const envBase =
    typeof process !== 'undefined' &&
    process?.env &&
    typeof process.env.API_BASE === 'string' &&
    process.env.API_BASE
      ? process.env.API_BASE
      : '';

  return normalizeBase(defineBase) || normalizeBase(envBase) || LOCAL_API_BASE;
};

export const API_BASE = resolveApiBase();

export const isLocalApiBase = () => API_BASE.includes('://127.0.0.1') || API_BASE.includes('://localhost');

const readServerMessage = (payload: any): string => {
  const message = payload?.message || payload?.msg || payload?.error;
  return typeof message === 'string' ? message : '';
};

const parseUploadPayload = (rawData: unknown): any => {
  if (typeof rawData === 'string') {
    const text = rawData.trim();
    if (!text) {
      return {};
    }
    try {
      return JSON.parse(text);
    } catch (err) {
      throw new Error(`服务返回格式异常: ${text.slice(0, 80)}`);
    }
  }

  if (rawData && typeof rawData === 'object') {
    return rawData;
  }

  return {};
};

export const uploadImage = async (filePath: string) => {
  const res = await Taro.uploadFile({
    url: `${API_BASE}/api/v1/analysis/upload`,
    filePath,
    name: 'file'
  });

  const payload = parseUploadPayload(res.data);

  if (typeof res.statusCode === 'number' && (res.statusCode < 200 || res.statusCode >= 300)) {
    throw new Error(readServerMessage(payload) || `服务异常(${res.statusCode})`);
  }

  if (!payload?.id) {
    throw new Error(readServerMessage(payload) || '上传失败');
  }

  return payload;
};

export const fetchAnalysis = async (id: string) => {
  const res = await Taro.request({
    url: `${API_BASE}/api/v1/analysis/${id}`,
    method: 'GET'
  });
  return res.data as any;
};

export const confirmAnalysis = async (id: string, confirmedText?: string) => {
  const res = await Taro.request({
    url: `${API_BASE}/api/v1/analysis/${id}/confirm`,
    method: 'POST',
    data: confirmedText ? { confirmed_text: confirmedText } : undefined
  });

  const payload = res.data as any;
  if (typeof res.statusCode === 'number' && (res.statusCode < 200 || res.statusCode >= 300)) {
    throw new Error(readServerMessage(payload) || `确认失败(${res.statusCode})`);
  }
  return payload;
};
