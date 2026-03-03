#!/usr/bin/env node
/* eslint-disable no-console */
const fs = require('fs');
const path = require('path');
const automator = require('miniprogram-automator');

const CLI_PATH = '/Applications/wechatwebdevtools.app/Contents/MacOS/cli';
const PROJECT_PATH = path.resolve(__dirname, '..');
const ARTIFACTS_DIR = path.resolve(PROJECT_PATH, 'artifacts', 'flow-check');

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function ensureArtifactsDir() {
  fs.mkdirSync(ARTIFACTS_DIR, { recursive: true });
}

async function expectElement(page, selector, label) {
  const element = await page.$(selector);
  if (!element) {
    throw new Error(`Element not found: ${label} (${selector})`);
  }
  return element;
}

function pagePath(page) {
  return String(page?.path || '');
}

async function currentPageOrThrow(miniProgram, label) {
  const page = await miniProgram.currentPage();
  if (!page) {
    throw new Error(`No current page after: ${label}`);
  }
  return page;
}

async function assertPathIncludes(miniProgram, expected, label) {
  const page = await currentPageOrThrow(miniProgram, label);
  const pathValue = pagePath(page);
  if (!pathValue.includes(expected)) {
    throw new Error(`Unexpected path after ${label}. expected~=${expected}, got=${pathValue}`);
  }
  return page;
}

async function saveShot(miniProgram, name) {
  const output = path.resolve(ARTIFACTS_DIR, `${Date.now()}-${name}.png`);
  await miniProgram.screenshot({ path: output });
  return output;
}

async function runScenario(miniProgram, buttonSelector, scenarioName) {
  console.log(`\n[Scenario] ${scenarioName}`);

  let page = await miniProgram.reLaunch('/pages/onboarding/index');
  await page.waitFor(1600);
  await expectElement(page, '.option-grid', 'onboarding options grid');
  const onbPrimary = await expectElement(page, '.primary-btn', 'onboarding primary button');
  const onbSecondary = await expectElement(page, '.secondary-btn', 'onboarding secondary button');
  console.log(`[${scenarioName}] onboarding buttons visible`);

  const targetButton = buttonSelector === '.primary-btn' ? onbPrimary : onbSecondary;
  await targetButton.tap();
  await sleep(1000);

  page = await assertPathIncludes(miniProgram, 'pages/capture/index', `${scenarioName}: onboarding click`);
  await page.waitFor(800);
  const captureTitle = await expectElement(page, '.section-title', 'capture title');
  const captureTitleText = await captureTitle.text();
  if (!String(captureTitleText).includes('开始分析')) {
    throw new Error(`[${scenarioName}] capture title mismatch: ${captureTitleText}`);
  }
  await expectElement(page, '.demo-btn', 'capture demo button');
  console.log(`[${scenarioName}] capture page visible with expected elements`);

  const shotCapture = await saveShot(miniProgram, `${scenarioName}-capture`);
  console.log(`[${scenarioName}] screenshot: ${shotCapture}`);

  const demoBtn = await expectElement(page, '.demo-btn', 'capture demo button');
  await demoBtn.tap();
  await sleep(1200);

  page = await assertPathIncludes(miniProgram, 'pages/ocr-result/index', `${scenarioName}: capture demo click`);
  await page.waitFor(800);
  const ocrTitle = await expectElement(page, '.section-title', 'ocr section title');
  const ocrTitleText = await ocrTitle.text();
  if (!String(ocrTitleText).includes('识别结果')) {
    throw new Error(`[${scenarioName}] ocr title mismatch: ${ocrTitleText}`);
  }
  const ocrStatus = await expectElement(page, '.ocr-status', 'ocr status');
  const statusText = await ocrStatus.text();
  if (!String(statusText).includes('demo')) {
    throw new Error(`[${scenarioName}] ocr status mismatch: ${statusText}`);
  }
  console.log(`[${scenarioName}] ocr page visible with expected elements`);

  const shotOcr = await saveShot(miniProgram, `${scenarioName}-ocr`);
  console.log(`[${scenarioName}] screenshot: ${shotOcr}`);

  const backBtn = await expectElement(page, '.secondary-btn', 'ocr back button');
  await backBtn.tap();
  await sleep(1000);
  await assertPathIncludes(miniProgram, 'pages/capture/index', `${scenarioName}: ocr back`);
  console.log(`[${scenarioName}] ocr back returns to capture`);
}

async function main() {
  if (!fs.existsSync(CLI_PATH)) {
    throw new Error(`WeChat DevTools CLI not found: ${CLI_PATH}`);
  }

  ensureArtifactsDir();

  console.log(`[Launch] cliPath=${CLI_PATH}`);
  console.log(`[Launch] projectPath=${PROJECT_PATH}`);

  const miniProgram = await automator.launch({
    cliPath: CLI_PATH,
    projectPath: PROJECT_PATH,
    timeout: 120000,
    port: 9420,
    trustProject: true
  });

  try {
    await runScenario(miniProgram, '.primary-btn', 'confirm-flow');
    await runScenario(miniProgram, '.secondary-btn', 'skip-flow');
    console.log('\n[PASS] Flow verification finished successfully.');
  } finally {
    await miniProgram.close();
  }
}

main().catch((err) => {
  console.error('\n[FAIL] Flow verification failed.');
  console.error(err && err.stack ? err.stack : err);
  process.exit(1);
});
