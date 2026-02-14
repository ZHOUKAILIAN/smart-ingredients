import { View, Text } from '@tarojs/components';
import Taro from '@tarojs/taro';
import { useState } from 'react';
import { getPreference, setPreference } from '../../utils/storage';
import './index.scss';

const OPTIONS = [
  { value: 'normal', label: '普通人群', desc: '适合大多数人，综合查看风险与建议' },
  { value: 'allergy', label: '过敏体质', desc: '重点关注过敏原与交叉污染提示' },
  { value: 'kids', label: '儿童/婴幼儿', desc: '关注高糖、刺激性与儿童敏感成分' },
  { value: 'pregnancy', label: '孕期/哺乳', desc: '关注刺激性成分与不明确添加剂' },
  { value: 'weight_loss', label: '控糖/控重', desc: '关注糖分、脂肪与热量负担' },
  { value: 'low_sodium', label: '低钠/心血管关注', desc: '关注钠盐、调味剂与血压负担' },
  { value: 'fitness', label: '健身增肌', desc: '关注蛋白质与整体营养结构' },
  { value: 'gut_sensitive', label: '肠胃敏感', desc: '关注刺激性成分与肠胃负担' },
  { value: 'lactose_intolerant', label: '乳糖不耐/乳制品敏感', desc: '关注乳制品相关成分' }
];

export default function Onboarding() {
  const [current, setCurrent] = useState(getPreference());

  const onConfirm = () => {
    setPreference(current);
    Taro.navigateTo({ url: '/pages/capture/index' });
  };

  const onSkip = () => {
    setPreference('normal');
    Taro.navigateTo({ url: '/pages/capture/index' });
  };

  return (
    <View className='container'>
      <View className='card intro-card'>
        <Text className='section-title'>欢迎使用 Smart Ingredients</Text>
        <Text className='subtle'>先选人群定位，分析结果会更贴近你的关注点</Text>
      </View>

      <View className='card steps-card'>
        <Text className='section-title'>快速上手</Text>
        <View className='step'>
          <Text className='step-title'>1. 选人群</Text>
          <Text className='step-desc'>确定你的关注重点</Text>
        </View>
        <View className='step'>
          <Text className='step-title'>2. 拍配料表</Text>
          <Text className='step-desc'>上传清晰配料表照片</Text>
        </View>
        <View className='step'>
          <Text className='step-title'>3. 看识别结果</Text>
          <Text className='step-desc'>先获取配料文本，再决定下一步</Text>
        </View>
      </View>

      <View className='card options-card'>
        <Text className='section-title'>人群定位</Text>
        {OPTIONS.map((opt) => (
          <View
            key={opt.value}
            className={current === opt.value ? 'option selected' : 'option'}
            onClick={() => setCurrent(opt.value)}
          >
            <Text className='option-label'>{opt.label}</Text>
            <Text className='option-desc'>{opt.desc}</Text>
          </View>
        ))}
      </View>

      <View className='actions'>
        <View className='secondary-btn' onClick={onSkip}>先用普通人群</View>
        <View className='primary-btn' onClick={onConfirm}>确认并开始</View>
        <Text className='subtle tips'>之后可在个人中心修改</Text>
      </View>
    </View>
  );
}
