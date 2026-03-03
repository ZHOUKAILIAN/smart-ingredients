export default defineAppConfig({
  pages: [
    'pages/capture/index',
    'pages/capture-scan/index',
    'pages/onboarding/index',
    'pages/ocr/index',
    'pages/ocr-result/index',
    'pages/analysis/index',
    'pages/analysis-result/index',
    'pages/analysis-detail/index'
  ],
  window: {
    navigationBarTitleText: 'Smart Ingredients',
    backgroundTextStyle: 'light',
    navigationBarBackgroundColor: '#ffffff',
    navigationBarTextStyle: 'black'
  }
});
