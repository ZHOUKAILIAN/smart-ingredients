import { View, Text } from '@tarojs/components';
import Taro, { useRouter } from '@tarojs/taro';
import { useEffect, useState } from 'react';
import { fetchAnalysis } from '../../utils/api';
import {
  buildIngredientDetailItems,
  buildSections,
  buildSummaryView,
  splitSectionsByTab,
  type SummaryViewModel
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

const EMPTY_SUMMARY: SummaryViewModel = {
  headline: '已完成配料分析，请查看详细配料表。',
  warnings: [],
  scoreText: '-'
};

export default function AnalysisResultPage() {
  const router = useRouter();
  const id = router.params?.id;
  const isDemo = router.params?.demo === '1';

  const [loading, setLoading] = useState(true);
  const [errorText, setErrorText] = useState('');
  const [summaryView, setSummaryView] = useState<SummaryViewModel>(EMPTY_SUMMARY);
  const [focusIngredients, setFocusIngredients] = useState<string[]>([]);

  const applyResult = (source: any) => {
    const sections = buildSections(source);
    const grouped = splitSectionsByTab(sections);
    setSummaryView(buildSummaryView(sections));
    const names = buildIngredientDetailItems(grouped.ingredients)
      .map((item) => item.name.trim())
      .filter(Boolean);
    setFocusIngredients(Array.from(new Set(names)).slice(0, 6));
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

  const openDetail = () => {
    if (isDemo) {
      Taro.navigateTo({ url: '/pages/analysis-detail/index?demo=1' });
      return;
    }

    if (!id) {
      Taro.showToast({ title: '缺少分析记录 ID', icon: 'none' });
      return;
    }

    Taro.navigateTo({ url: `/pages/analysis-detail/index?id=${id}` });
  };

  const warningList = summaryView.warnings.length
    ? summaryView.warnings
    : ['保持饮食清淡，注意控制总摄入量。', '建议减少高糖高盐加工食品频次。'];

  return (
    <View className='container analysis-summary-page'>
      <View className='card summary-hero-card'>
        {loading && <Text className='subtle'>处理中…</Text>}
        {!loading && !!errorText && <Text className='error-text'>{errorText}</Text>}
        {!loading && !errorText && (
          <View className='hero-panel'>
            <View className='hero-score'>
              <Text className='score-label'>健康评分</Text>
              <Text className='score-value'>{summaryView.scoreText === '-' ? '—' : summaryView.scoreText}</Text>
            </View>
            <View className='hero-divider' />
            <View className='hero-content'>
              <Text className='section-title'>分析摘要</Text>
              <Text className='headline-text'>{summaryView.headline}</Text>
            </View>
          </View>
        )}
      </View>

      {!loading && !errorText && (
        <View className='card ingredient-focus-card'>
          <View className='card-head'>
            <Text className='section-title'>重点配料</Text>
            <Text className='head-tag'>{focusIngredients.length} 项</Text>
          </View>
          {focusIngredients.length ? (
            <View className='chip-list'>
              {focusIngredients.map((name) => (
                <Text className='focus-chip' key={name}>
                  {name}
                </Text>
              ))}
            </View>
          ) : (
            <Text className='subtle'>暂无重点配料，点击“查看详细配料表”查看更多内容。</Text>
          )}
        </View>
      )}

      {!loading && !errorText && (
        <View className='card warning-card'>
          <View className='card-head'>
            <Text className='section-title'>注意事项</Text>
            <Text className='head-tag'>{warningList.length} 条</Text>
          </View>
          <View className='warning-list'>
            {warningList.map((item, idx) => (
              <View className='warning-item' key={`${item}-${idx}`}>
                <View className='warning-dot' />
                <Text className='warning-text'>{item}</Text>
              </View>
            ))}
          </View>
        </View>
      )}

      <View className='actions action-stack'>
        {!loading && !errorText && (
          <View className='primary-btn compact-btn' onClick={openDetail}>
            查看详细配料表
          </View>
        )}
        <View className='secondary-btn compact-btn' onClick={() => Taro.redirectTo({ url: '/pages/capture/index' })}>
          重新拍照
        </View>
      </View>
    </View>
  );
}
