import { View, Text } from '@tarojs/components';
import Taro, { useRouter } from '@tarojs/taro';
import { useEffect, useState } from 'react';
import { fetchAnalysis } from '../../utils/api';
import './index.scss';

export default function Ocr() {
  const router = useRouter();
  const id = router.params?.id;
  const [status, setStatus] = useState('ocr_pending');
  const [ocrText, setOcrText] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let timer: number | undefined;
    const poll = async () => {
      if (!id) {
        setLoading(false);
        return;
      }
      try {
        const res = await fetchAnalysis(id);
        setStatus(res.status || res.ocr_status || 'ocr_pending');
        if (res.ocr_text) {
          setOcrText(res.ocr_text);
        }
        if (res.status === 'ocr_completed' || res.ocr_status === 'completed') {
          setLoading(false);
          return;
        }
      } catch {
        Taro.showToast({ title: '识别失败', icon: 'none' });
        setLoading(false);
        return;
      }
      timer = window.setTimeout(poll, 1200);
    };
    poll();
    return () => {
      if (timer) {
        clearTimeout(timer);
      }
    };
  }, [id]);

  return (
    <View className='container'>
      <View className='card'>
        <Text className='section-title'>OCR 识别结果</Text>
        {loading ? (
          <Text className='subtle'>识别中，请稍候…</Text>
        ) : (
          <Text className='ocr-text'>{ocrText || '未识别到文本'}</Text>
        )}
        {!loading && (
          <Text className='subtle'>状态：{status}</Text>
        )}
      </View>

      <View className='actions'>
        <View className='secondary-btn' onClick={() => Taro.navigateBack()}>
          重新拍照
        </View>
      </View>
    </View>
  );
}
