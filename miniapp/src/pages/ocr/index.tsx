import { View, Text } from '@tarojs/components';
import Taro, { useRouter } from '@tarojs/taro';
import { useEffect } from 'react';
import { fetchAnalysis } from '../../utils/api';
import './index.scss';

export default function Ocr() {
  const router = useRouter();
  const id = router.params?.id;
  const isDemo = router.params?.demo === '1';

  useEffect(() => {
    let timer: number | undefined;

    if (isDemo) {
      timer = setTimeout(() => {
        Taro.redirectTo({ url: '/pages/ocr-result/index?demo=1' });
      }, 900);
      return;
    }

    if (!id) {
      Taro.showToast({ title: '缺少记录 ID', icon: 'none' });
      return;
    }

    const poll = async () => {
      try {
        const res = await fetchAnalysis(id);
        if (res.status === 'ocr_completed' || res.ocr_status === 'completed') {
          Taro.redirectTo({ url: `/pages/ocr-result/index?id=${id}` });
          return;
        }
      } catch {
        Taro.showToast({ title: '识别失败', icon: 'none' });
        return;
      }
      timer = setTimeout(poll, 1200);
    };
    poll();

    return () => {
      if (timer) {
        clearTimeout(timer);
      }
    };
  }, [id, isDemo]);

  return (
    <View className='container ocr-page'>
      <View className='status-card'>
        <View className='ocr-badge'>
          <Text>OCR</Text>
        </View>
        <Text className='status-title'>正在识别配料表…</Text>
        <Text className='status-subtle'>请稍候，通常需要 3-5 秒</Text>
        <View className='status-track'>
          <View className='status-bar' />
        </View>
      </View>

      <View className='actions'>
        <View className='secondary-btn' onClick={() => Taro.navigateBack()}>
          返回重拍
        </View>
      </View>
    </View>
  );
}
