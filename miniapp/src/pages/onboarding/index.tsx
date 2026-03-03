import { View, Text } from '@tarojs/components';
import Taro from '@tarojs/taro';
import { useEffect, useState } from 'react';
import { getPreference, hasPreferenceConfigured, setPreference } from '../../utils/storage';
import './index.scss';

const OPTIONS = [
  { value: 'normal', label: '普通人群', desc: '综合查看风险和建议' },
  { value: 'allergy', label: '过敏体质', desc: '重点关注过敏原' },
  { value: 'kids', label: '儿童/婴幼儿', desc: '关注高糖和刺激性' },
  { value: 'pregnancy', label: '孕期/哺乳', desc: '关注安全性成分' },
  { value: 'weight_loss', label: '控糖/控重', desc: '关注糖分与热量' },
  { value: 'low_sodium', label: '低钠关注', desc: '关注钠盐与调味剂' }
];

export default function Onboarding() {
  const [current, setCurrent] = useState(getPreference());
  const currentLabel = OPTIONS.find((opt) => opt.value === current)?.label || '普通人群';

  useEffect(() => {
    if (hasPreferenceConfigured()) {
      Taro.redirectTo({ url: '/pages/capture/index' });
    }
  }, []);

  const onConfirm = () => {
    setPreference(current);
    Taro.redirectTo({ url: '/pages/capture/index' });
  };

  return (
    <View className='container onboarding-page'>
      <View className='card intro-card'>
        <Text className='section-title'>Smart Ingredients</Text>
        <Text className='subtle'>先选择你的关注人群，分析结果会优先展示你更关心的风险。</Text>
        <Text className='current-tip'>当前选择：{currentLabel}</Text>
      </View>

      <View className='card option-card'>
        <Text className='section-title'>人群定位</Text>
        <View className='option-grid'>
          {OPTIONS.map((opt) => (
            <View
              key={opt.value}
              className={`option-item ${current === opt.value ? 'selected' : ''}`}
              onClick={() => setCurrent(opt.value)}
            >
              <View className='option-head'>
                <Text className='option-label'>{opt.label}</Text>
                <View className={`option-check ${current === opt.value ? 'checked' : ''}`}>
                  {current === opt.value ? '✓' : ''}
                </View>
              </View>
              <Text className='option-desc'>{opt.desc}</Text>
            </View>
          ))}
        </View>
      </View>

      <View className='actions'>
        <View className='primary-btn' onClick={onConfirm}>
          确认并开始
        </View>
      </View>
    </View>
  );
}
