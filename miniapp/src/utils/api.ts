import Taro from '@tarojs/taro';

export const API_BASE = (process.env.API_BASE || 'http://127.0.0.1:3000') as string;

export const uploadImage = async (filePath: string) => {
  const res = await Taro.uploadFile({
    url: `${API_BASE}/api/v1/analysis/upload`,
    filePath,
    name: 'file'
  });
  return JSON.parse(res.data);
};

export const fetchAnalysis = async (id: string) => {
  const res = await Taro.request({
    url: `${API_BASE}/api/v1/analysis/${id}`,
    method: 'GET'
  });
  return res.data as any;
};
