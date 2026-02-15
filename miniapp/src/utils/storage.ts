const KEY = 'analysis_preference';

export const getPreference = (): string => {
  try {
    const value = wx.getStorageSync(KEY);
    return value ? String(value) : 'normal';
  } catch {
    return 'normal';
  }
};

export const setPreference = (value: string) => {
  try {
    wx.setStorageSync(KEY, value);
  } catch {
    // ignore
  }
};
