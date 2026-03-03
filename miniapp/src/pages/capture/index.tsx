import { View, Text } from '@tarojs/components';
import Taro from '@tarojs/taro';
import './index.scss';

const STEP_ITEMS = [
  { title: '拍摄配料表', desc: '对准配料表垂直拍照', icon: 'camera-mini' },
  { title: '确认识别文本', desc: 'AI自动识别配料信息', icon: 'album-mini' },
  { title: '查看健康报告', desc: '获取详细的分析和建议', icon: 'report-mini' }
];

export default function Capture() {
  return (
    <View className='container capture-page'>
      <View className='intro-hero'>
        <View className='hero-ai-wrap'>
          <View className='hero-ai-icon'>
            <Text className='hero-spark'>✦</Text>
          </View>
          <View className='hero-ai-dot'>AI</View>
          <View className='hero-ai-glow' />
        </View>
        <Text className='intro-title'>Smart Ingredients</Text>
        <Text className='intro-subtitle'>AI智能配料表分析</Text>
        <Text className='intro-desc'>拍摄识别配料表，AI分析健康风险，让您吃得更安心</Text>
      </View>

      <View className='card steps-card'>
        <View className='steps-title-row'>
          <View className='steps-mark' />
          <Text className='steps-title'>使用步骤</Text>
          <View className='steps-mark' />
        </View>
        {STEP_ITEMS.map((item, idx) => (
          <View className='step-item' key={item.title}>
            <View className='step-left'>
              <View className='step-icon-wrap'>
                <View className={`btn-icon ${item.icon}`} />
              </View>
              <View className='step-index'>{idx + 1}</View>
            </View>
            <View className='step-right'>
              <Text className='step-name'>{item.title}</Text>
              <Text className='step-desc'>{item.desc}</Text>
            </View>
          </View>
        ))}
      </View>

      <View className='actions'>
        <View className='primary-btn start-btn' onClick={() => Taro.navigateTo({ url: '/pages/capture-scan/index' })}>
          开始分析
        </View>
      </View>
    </View>
  );
}
