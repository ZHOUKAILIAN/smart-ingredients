#!/usr/bin/env node
/* eslint-disable no-console */
const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

const mode = process.argv[2] || 'build';
const isDevMode = mode === 'dev';
const isBuildMode = mode === 'build';

if (!isDevMode && !isBuildMode) {
  console.error('Usage: node scripts/run-weapp-with-orbstack.js <build|dev>');
  process.exit(1);
}

const CONTAINERS = [
  'smart-ingredients-db',
  'smart-ingredients-storage',
  'smart-ingredients-redis',
  'smart-ingredients-ocr',
  'smart-ingredients-backend'
];

const isIPv4 = (value) => /^\d+\.\d+\.\d+\.\d+$/.test(value);

const pickHostIp = () => {
  const interfaces = os.networkInterfaces();
  const preferredOrder = ['en0', 'en1', 'eth0', 'wlan0'];

  const scan = (names) => {
    for (const name of names) {
      const entries = interfaces[name] || [];
      for (const info of entries) {
        const family = typeof info.family === 'string' ? info.family : String(info.family);
        if (family !== 'IPv4' || info.internal) {
          continue;
        }
        if (!isIPv4(info.address)) {
          continue;
        }
        return { ip: info.address, iface: name };
      }
    }
    return null;
  };

  return scan(preferredOrder) || scan(Object.keys(interfaces));
};

const run = (cmd, args, options = {}) => spawnSync(cmd, args, { encoding: 'utf8', ...options });

const ensureContainers = () => {
  const result = run('docker', ['start', ...CONTAINERS], { stdio: 'pipe' });
  if (result.status !== 0) {
    console.warn('[warn] Docker containers were not fully started.');
    const stderr = (result.stderr || '').trim();
    if (stderr) {
      console.warn(stderr);
    }
  } else {
    console.log('[ok] OrbStack containers are running.');
  }
};

const checkBackendReachable = (apiBase) => {
  const result = run('curl', ['-sS', '-m', '4', '-o', '/dev/null', '-w', '%{http_code}', `${apiBase}/api/v1/analysis/upload`], {
    stdio: 'pipe'
  });

  const code = (result.stdout || '').trim();
  if (result.status !== 0 || !['200', '401', '403', '405'].includes(code)) {
    console.warn(`[warn] Backend health check unexpected (status=${code || 'n/a'}).`);
  } else {
    console.log(`[ok] Backend reachable at ${apiBase} (HTTP ${code}).`);
  }
};

const main = () => {
  ensureContainers();

  const host = pickHostIp();
  if (!host) {
    console.error('Unable to detect a host IPv4 address.');
    process.exit(1);
  }

  const apiBase = `http://${host.ip}:3000`;
  console.log(`[info] Using interface ${host.iface}, API_BASE=${apiBase}`);
  checkBackendReachable(apiBase);

  const taroBin = path.resolve(
    __dirname,
    '..',
    'node_modules',
    '.bin',
    process.platform === 'win32' ? 'taro.cmd' : 'taro'
  );
  const taroArgs = ['build', '--type', 'weapp'];
  if (isDevMode) {
    taroArgs.push('--watch');
  }

  const buildResult = spawnSync(taroBin, taroArgs, {
    stdio: 'inherit',
    env: { ...process.env, API_BASE: apiBase }
  });

  process.exit(buildResult.status || 0);
};

main();
