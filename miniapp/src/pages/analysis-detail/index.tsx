import { View, Text, ScrollView } from '@tarojs/components';
import Taro, { useRouter } from '@tarojs/taro';
import { useEffect, useState } from 'react';
import { fetchAnalysis } from '../../utils/api';
import {
  buildIngredientDetailItems,
  buildSections,
  formatRiskLevel,
  splitSectionsByTab,
  type IngredientDetailItem
} from '../../utils/analysis-view';
import './index.scss';

const DONE_STATUSES = ['completed', 'done', 'success', 'llm_completed', 'ready'];
const FAILED_STATUSES = ['failed', 'error'];

const DEMO_RESULT = {
  综合评分: '72 / 100',
  核心结论: '属于高甜度调味饮料，建议控制饮用频次。',
  建议频次: '每周 2-3 次',
  风险提示: '糖分和香精占比较高，长期高频饮用会增加代谢负担。',
  配料分析: {
    原始配料: '饮用水、白砂糖、果葡糖浆、柠檬酸、食用香精',
    甜味来源: '白砂糖、果葡糖浆',
    酸度调节: '柠檬酸',
    风味添加: '食用香精'
  }
};

export default function AnalysisDetailPage() {
  const router = useRouter();
  const id = router.params?.id;
  const isDemo = router.params?.demo === '1';

  const [loading, setLoading] = useState(true);
  const [errorText, setErrorText] = useState('');
  const [ingredientItems, setIngredientItems] = useState<IngredientDetailItem[]>([]);

  const applyResult = (source: any) => {
    const grouped = splitSectionsByTab(buildSections(source));
    setIngredientItems(buildIngredientDetailItems(grouped.ingredients));
  };

  useEffect(() => {
    if (isDemo) {
      applyResult(DEMO_RESULT);
      setLoading(false);
      return;
    }

    if (!id) {
      setErrorText('缺少分析记录 ID');
      setLoading(false);
      return;
    }

    let timer: ReturnType<typeof setTimeout> | undefined;

    const poll = async () => {
      try {
        const res: any = await fetchAnalysis(id);
        const llmStatus = String(res?.llm_status || res?.status || 'pending');

        if (FAILED_STATUSES.includes(llmStatus)) {
          setErrorText(String(res?.error_message || '分析失败'));
          setLoading(false);
          return;
        }

        if (res?.result != null) {
          applyResult(res.result);
          setLoading(false);
          return;
        }

        if (DONE_STATUSES.includes(llmStatus)) {
          applyResult(res);
          setLoading(false);
          return;
        }
      } catch (err: any) {
        const errMsg = String(err?.errMsg || err?.message || '分析失败');
        setErrorText(errMsg);
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

  const goSummary = () => {
    const pages = Taro.getCurrentPages();
    if (pages.length > 1) {
      Taro.navigateBack();
      return;
    }

    if (isDemo) {
      Taro.redirectTo({ url: '/pages/analysis-result/index?demo=1' });
      return;
    }

    if (id) {
      Taro.redirectTo({ url: `/pages/analysis-result/index?id=${id}` });
      return;
    }

    Taro.redirectTo({ url: '/pages/capture/index' });
  };

  return (
    <View className='container analysis-detail-page'>
      <View className='card detail-card'>
        <View className='detail-head'>
          <Text className='section-title'>详细配料表</Text>
          <Text className='subtle'>按风险优先级排序展示，便于快速查看重点配料</Text>
        </View>

        {loading && <Text className='subtle'>加载中…</Text>}
        {!loading && !!errorText && <Text className='error-text'>{errorText}</Text>}

        {!loading && !errorText && (
          <ScrollView scrollY className='detail-scroll'>
            {ingredientItems.length ? (
              ingredientItems.map((item, idx) => (
                <View className='ingredient-card' key={`${item.name}-${item.category}-${idx}`}>
                  <View className='ingredient-head'>
                    <Text className='ingredient-name'>{item.name}</Text>
                    <Text className={`risk-badge risk-${item.riskLevel}`}>{formatRiskLevel(item.riskLevel)}</Text>
                  </View>

                  {!!item.category && <Text className='ingredient-category'>{item.category}</Text>}
                  <Text className='ingredient-function'>{item.functionText || '暂无说明'}</Text>
                  {!!item.note && <Text className='ingredient-note'>备注：{item.note}</Text>}
                </View>
              ))
            ) : (
              <Text className='subtle'>暂无配料数据</Text>
            )}
          </ScrollView>
        )}
      </View>

      <View className='actions action-row'>
        <View className='secondary-btn compact-btn' onClick={goSummary}>
          返回概要
        </View>
        <View className='primary-btn compact-btn' onClick={() => Taro.redirectTo({ url: '/pages/capture/index' })}>
          重新拍照
        </View>
      </View>
    </View>
  );
}
