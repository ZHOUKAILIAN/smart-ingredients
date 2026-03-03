import { View, Text } from '@tarojs/components';
import Taro from '@tarojs/taro';
import { useState } from 'react';
import { API_BASE, isLocalApiBase, uploadImage } from '../../utils/api';
import './index.scss';

type ImageSource = 'camera' | 'album';

const CAPTURE_TIPS = [
  '确保配料表文字清晰，光线充足',
  '尽量平行拍摄，避免文字倾斜',
  '避免反光和阴影遮挡文字'
];

const CONNECTION_ERROR_MARKERS = [
  'uploadFile:fail',
  'request:fail',
  'Failed to connect',
  'ERR_CONNECTION_REFUSED',
  'ECONNREFUSED',
  'timeout'
];

const isConnectionError = (errMsg: string) => CONNECTION_ERROR_MARKERS.some((marker) => errMsg.includes(marker));

const showUploadError = (errMsg: string) => {
  if (isConnectionError(errMsg)) {
    const content = isLocalApiBase()
      ? `无法连接上传服务：${API_BASE}\n请先启动后端，或把 API_BASE 改为可访问地址。`
      : `无法连接上传服务：${API_BASE}\n请检查网络或后端服务状态。`;

    Taro.showModal({
      title: '上传失败',
      content,
      showCancel: false,
      confirmText: '知道了'
    });
    return;
  }

  Taro.showModal({
    title: '上传失败',
    content: errMsg || '上传失败，请稍后重试',
    showCancel: false,
    confirmText: '知道了'
  });
};

export default function CaptureScan() {
  const [loading, setLoading] = useState(false);

  const pickAndUpload = async (sourceType: ImageSource[]) => {
    if (loading) {
      return;
    }

    if (isLocalApiBase()) {
      try {
        const systemInfo = Taro.getSystemInfoSync();
        if (systemInfo.platform !== 'devtools') {
          Taro.showModal({
            title: '上传不可用',
            content: `当前接口地址是 ${API_BASE}，真机无法访问本机地址。\n请改为局域网或公网地址后再试。`,
            showCancel: false,
            confirmText: '知道了'
          });
          return;
        }
      } catch (_err) {
        // ignore
      }
    }

    try {
      setLoading(true);
      Taro.showLoading({
        title: '上传中…',
        mask: true
      });

      let res;
      try {
        res = await Taro.chooseImage({ count: 1, sourceType });
      } catch (pickError: any) {
        const errMsg = String(pickError?.errMsg || '');
        if (errMsg.includes('webapi_getwxaasyncsecinfo') && sourceType.includes('camera')) {
          Taro.showToast({ title: '相机不可用，改用相册', icon: 'none' });
          res = await Taro.chooseImage({ count: 1, sourceType: ['album'] });
        } else {
          throw pickError;
        }
      }

      const filePath = res.tempFilePaths[0];
      const uploadRes = await uploadImage(filePath);
      Taro.navigateTo({ url: `/pages/ocr/index?id=${uploadRes.id}` });
    } catch (err: any) {
      const errMsg = String(err?.errMsg || err?.message || '');
      if (errMsg.includes('webapi_getwxaasyncsecinfo')) {
        Taro.showToast({ title: '当前环境权限异常，请使用真实 AppID', icon: 'none' });
      } else {
        showUploadError(errMsg);
      }
    } finally {
      Taro.hideLoading();
      setLoading(false);
    }
  };

  return (
    <View className='container capture-scan-page'>
      <View className='card upload-card'>
        <View className='upload-hero'>
          <View className='hero-icon-box'>
            <View className='hero-camera-icon' />
          </View>
        </View>

        <Text className='section-title'>开始分析 · 上传配料表照片</Text>
        <Text className='subtle'>请确保配料表文字清晰可见</Text>
        <View className='middle-gap' />

        <View className='actions capture-actions'>
          <View className={`primary-btn action-btn ${loading ? 'disabled-btn' : ''}`} onClick={() => pickAndUpload(['camera'])}>
            <View className='btn-icon camera-mini' />
            <Text>{loading ? '上传中…' : '拍照'}</Text>
          </View>
          <View className={`secondary-btn action-btn ${loading ? 'disabled-btn' : ''}`} onClick={() => pickAndUpload(['album'])}>
            <View className='btn-icon album-mini' />
            <Text>{loading ? '上传中…' : '从相册选择'}</Text>
          </View>
        </View>

        <Text className='demo-btn' onClick={() => Taro.navigateTo({ url: '/pages/ocr/index?demo=1' })}>
          跳过上传（演示）
        </Text>
      </View>

      <View className='card tips-card'>
        <View className='tips-title-row'>
          <View className='tips-mark' />
          <Text className='tips-title'>拍摄小贴士</Text>
        </View>
        {CAPTURE_TIPS.map((item) => (
          <View className='tip-item' key={item}>
            <View className='tip-dot' />
            <Text className='tip-text'>{item}</Text>
          </View>
        ))}
      </View>
    </View>
  );
}
