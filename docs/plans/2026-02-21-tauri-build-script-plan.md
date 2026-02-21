# Tauri Build Script Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 修复 `cargo test` 中 `tauri::generate_context!()` 的 `OUT_DIR` 错误。

**Architecture:** 在 `frontend/src-tauri` 添加 build script，使用 `tauri-build` 生成 context，并监听 `tauri.conf.json` 变更。

**Tech Stack:** Rust, Tauri 2.x, tauri-build 2.x

---

### Task 1: 复现失败用例（RED）

**Files:**
- Test: none (使用现有构建失败作为 RED)

**Step 1: Run test to verify it fails**

Run: `cargo test -p smart-ingredients-tauri`
Expected: FAIL with `OUT_DIR env var is not set` from `tauri::generate_context!()`

### Task 2: 添加 build script（GREEN）

**Files:**
- Create: `frontend/src-tauri/build.rs`

**Step 1: Write minimal implementation**

```rust
fn main() {
    println!("cargo:rerun-if-changed=tauri.conf.json");
    tauri_build::build();
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-tauri`
Expected: PASS (不再出现 `OUT_DIR` 错误)

### Task 3: 补齐缺失图标文件（GREEN）

**Files:**
- Create: `frontend/src-tauri/icons/32x32.png`
- Create: `frontend/src-tauri/icons/128x128.png`
- Create: `frontend/src-tauri/icons/128x128@2x.png`

**Step 1: Write minimal implementation**

```bash
node - <<'NODE'
const fs = require('fs');
const zlib = require('zlib');

const crcTable = new Uint32Array(256);
for (let i = 0; i < 256; i++) {
  let c = i;
  for (let k = 0; k < 8; k++) {
    c = (c & 1) ? (0xEDB88320 ^ (c >>> 1)) : (c >>> 1);
  }
  crcTable[i] = c >>> 0;
}

function crc32(buf) {
  let c = 0xFFFFFFFF;
  for (const b of buf) {
    c = crcTable[(c ^ b) & 0xFF] ^ (c >>> 8);
  }
  return (c ^ 0xFFFFFFFF) >>> 0;
}

function chunk(type, data) {
  const typeBuf = Buffer.from(type);
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length, 0);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(Buffer.concat([typeBuf, data])), 0);
  return Buffer.concat([len, typeBuf, data, crc]);
}

function png(width, height, rgba) {
  const [r, g, b, a] = rgba;
  const row = width * 4 + 1;
  const raw = Buffer.alloc(row * height);
  for (let y = 0; y < height; y++) {
    const offset = y * row;
    raw[offset] = 0;
    for (let x = 0; x < width; x++) {
      const i = offset + 1 + x * 4;
      raw[i] = r;
      raw[i + 1] = g;
      raw[i + 2] = b;
      raw[i + 3] = a;
    }
  }
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  ihdr[10] = 0;
  ihdr[11] = 0;
  ihdr[12] = 0;
  const idat = zlib.deflateSync(raw);
  const signature = Buffer.from([0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);
  return Buffer.concat([signature, chunk('IHDR', ihdr), chunk('IDAT', idat), chunk('IEND', Buffer.alloc(0))]);
}

const color = [255, 255, 255, 255];
fs.writeFileSync('frontend/src-tauri/icons/32x32.png', png(32, 32, color));
fs.writeFileSync('frontend/src-tauri/icons/128x128.png', png(128, 128, color));
fs.writeFileSync('frontend/src-tauri/icons/128x128@2x.png', png(256, 256, color));
NODE
```

**Step 2: Run test to verify it passes**

Run: `cargo test -p smart-ingredients-tauri`
Expected: PASS (不再出现 icon 缺失错误)

### Task 4: 更新忽略规则（GREEN）

**Files:**
- Modify: `.gitignore`

**Step 1: Update ignore rules**

Add exceptions for Tauri build assets:

```gitignore
!frontend/src-tauri/build.rs
!frontend/src-tauri/icons/32x32.png
!frontend/src-tauri/icons/128x128.png
!frontend/src-tauri/icons/128x128@2x.png
```

### Task 5: 运行全量测试并提交

**Files:**
- Modify: none

**Step 1: Run full tests**

Run: `cargo test`
Expected: PASS

**Step 2: Commit**

```bash
git add .gitignore frontend/src-tauri/build.rs frontend/src-tauri/icons/32x32.png frontend/src-tauri/icons/128x128.png frontend/src-tauri/icons/128x128@2x.png docs/requirements/026-tauri-build-script-requirements.md docs/design/026-tauri-build-script-technical-plan.md docs/plans/2026-02-21-tauri-build-script-plan.md
git commit -m "fix(build): add tauri build script"
```
