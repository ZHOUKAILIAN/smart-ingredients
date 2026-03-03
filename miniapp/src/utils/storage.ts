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

export const hasPreferenceConfigured = (): boolean => {
  try {
    const value = wx.getStorageSync(KEY);
    return typeof value === 'string' ? value.trim().length > 0 : !!value;
  } catch {
    return false;
  }
};
