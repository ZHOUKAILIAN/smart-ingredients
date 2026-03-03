import { View, Text } from '@tarojs/components';
import Taro, { useRouter } from '@tarojs/taro';
import { useEffect } from 'react';
import { fetchAnalysis } from '../../utils/api';
import './index.scss';

const DONE_STATUSES = ['completed', 'done', 'success', 'llm_completed', 'ready'];
const FAILED_STATUSES = ['failed', 'error'];

export default function AnalysisPage() {
  const router = useRouter();
  const id = router.params?.id;
  const isDemo = router.params?.demo === '1';

  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | undefined;

    if (isDemo) {
      timer = setTimeout(() => {
        Taro.redirectTo({ url: '/pages/analysis-result/index?demo=1' });
      }, 900);
      return;
    }

    if (!id) {
      Taro.showToast({ title: '缺少分析记录 ID', icon: 'none' });
      return;
    }

    const poll = async () => {
      try {
        const res: any = await fetchAnalysis(id);
        const llmStatus = String(res?.llm_status || res?.status || 'pending');

        if (FAILED_STATUSES.includes(llmStatus)) {
          Taro.redirectTo({ url: `/pages/analysis-result/index?id=${id}` });
          return;
        }

        if (res?.result != null) {
          Taro.redirectTo({ url: `/pages/analysis-result/index?id=${id}` });
          return;
        }

        if (DONE_STATUSES.includes(llmStatus)) {
          Taro.redirectTo({ url: `/pages/analysis-result/index?id=${id}` });
          return;
        }
      } catch {
        Taro.showToast({ title: '分析失败', icon: 'none' });
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
    <View className='container analysis-page'>
      <View className='card status-card'>
        <Text className='status-title'>结果分析中…</Text>
        <Text className='status-subtle'>正在生成分析结果，请稍候</Text>
        <View className='status-track'>
          <View className='status-bar' />
        </View>
      </View>

      <View className='actions'>
        <View className='secondary-btn' onClick={() => Taro.redirectTo({ url: '/pages/capture/index' })}>
          重新拍照
        </View>
      </View>
    </View>
  );
}
