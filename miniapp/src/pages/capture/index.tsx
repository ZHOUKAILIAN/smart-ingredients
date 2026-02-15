import { View, Text } from '@tarojs/components';
import Taro from '@tarojs/taro';
import { useEffect, useState } from 'react';
import { getPreference } from '../../utils/storage';
import { uploadImage } from '../../utils/api';
import './index.scss';

export default function Capture() {
  const [loading, setLoading] = useState(false);
  const [preference, setPreference] = useState('normal');

  useEffect(() => {
    const value = getPreference();
    setPreference(value || 'normal');
  }, []);

  const chooseImage = async () => {
    try {
      setLoading(true);
      const res = await Taro.chooseImage({ count: 1, sourceType: ['camera', 'album'] });
      const filePath = res.tempFilePaths[0];
      const uploadRes = await uploadImage(filePath);
      if (!uploadRes?.id) {
        throw new Error('上传失败');
      }
      Taro.navigateTo({ url: `/pages/ocr/index?id=${uploadRes.id}` });
    } catch (err) {
      Taro.showToast({ title: '上传失败', icon: 'none' });
    } finally {
      setLoading(false);
    }
  };

  return (
    <View className='container'>
      <View className='card'>
        <Text className='section-title'>开始拍照上传</Text>
        <Text className='subtle'>当前人群定位：{preference}</Text>
      </View>

      <View className='card tips-card'>
        <Text className='section-title'>拍摄建议</Text>
        <Text className='subtle'>对准配料表、避免反光、尽量保持清晰</Text>
      </View>

      <View className='actions'>
        <View className='primary-btn' onClick={chooseImage}>
          {loading ? '上传中…' : '拍照/上传配料表'}
        </View>
      </View>
    </View>
  );
}
