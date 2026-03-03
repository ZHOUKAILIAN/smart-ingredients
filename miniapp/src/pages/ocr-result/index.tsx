import { View, Text, Textarea } from '@tarojs/components';
import Taro, { useRouter } from '@tarojs/taro';
import { useEffect, useState } from 'react';
import { confirmAnalysis, fetchAnalysis } from '../../utils/api';
import './index.scss';

export default function OcrResultPage() {
  const router = useRouter();
  const id = router.params?.id;
  const isDemo = router.params?.demo === '1';

  const [status, setStatus] = useState('ocr_pending');
  const [ocrText, setOcrText] = useState('');
  const [loading, setLoading] = useState(true);
  const [confirming, setConfirming] = useState(false);

  useEffect(() => {
    if (isDemo) {
      setStatus('demo');
      setOcrText('示例配料：饮用水、白砂糖、柠檬酸、食用香精。');
      setLoading(false);
      return;
    }

    if (!id) {
      Taro.showToast({ title: '缺少记录 ID', icon: 'none' });
      setLoading(false);
      return;
    }

    let timer: number | undefined;
    const poll = async () => {
      try {
        const res: any = await fetchAnalysis(id);
        setStatus(res.status || res.ocr_status || 'ocr_pending');
        if (res.ocr_text) {
          setOcrText(res.ocr_text);
        }
        if (res.status === 'ocr_completed' || res.ocr_status === 'completed' || res.ocr_text) {
          setLoading(false);
          return;
        }
      } catch {
        Taro.showToast({ title: '识别失败', icon: 'none' });
        setLoading(false);
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

  const onConfirm = async () => {
    if (confirming || loading) {
      return;
    }

    try {
      setConfirming(true);
      Taro.showLoading({
        title: '确认中…',
        mask: true
      });

      if (isDemo) {
        Taro.navigateTo({ url: '/pages/analysis/index?demo=1' });
        return;
      }

      if (!id) {
        Taro.showToast({ title: '缺少记录 ID', icon: 'none' });
        return;
      }

      await confirmAnalysis(id, ocrText || undefined);
      Taro.navigateTo({ url: `/pages/analysis/index?id=${id}` });
    } catch (err: any) {
      const errMsg = String(err?.errMsg || err?.message || '确认失败');
      Taro.showToast({ title: errMsg, icon: 'none' });
    } finally {
      Taro.hideLoading();
      setConfirming(false);
    }
  };

  return (
    <View className='container ocr-result-page'>
      <View className='card result-card'>
        <Text className='section-title'>识别结果</Text>
        {loading ? (
          <Text className='subtle'>识别中，请稍候…</Text>
        ) : (
          <View className='result-scroll-wrap'>
            <Textarea
              className='ocr-textarea'
              value={ocrText}
              maxlength={-1}
              placeholder='未识别到文本，可手动输入或修改后再确认'
              onInput={(e) => setOcrText(e.detail.value)}
            />
          </View>
        )}
        {!loading && <Text className='ocr-status'>状态：{status}</Text>}
      </View>

      <View className='actions action-row'>
        <View className='secondary-btn' onClick={() => Taro.redirectTo({ url: '/pages/capture/index' })}>
          重新拍照
        </View>
        <View className={`primary-btn ${loading || confirming ? 'disabled-btn' : ''}`} onClick={onConfirm}>
          {confirming ? '确认中…' : '确认结果'}
        </View>
      </View>
    </View>
  );
}
