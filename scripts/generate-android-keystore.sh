#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
android_dir="${root_dir}/frontend/src-tauri/gen/android"
keystore_file="${android_dir}/release-key.jks"
keystore_props="${android_dir}/keystore.properties"

if [[ ! -d "${android_dir}" ]]; then
  echo "Android project not initialized. Run: cargo tauri android init" >&2
  exit 1
fi

if [[ -f "${keystore_file}" ]]; then
  echo "Keystore already exists: ${keystore_file}"
  exit 0
fi

keystore_password="${KEYSTORE_PASSWORD:-smart_ingredients_store_pass_2024}"
key_password="${KEY_PASSWORD:-${keystore_password}}"
key_alias="${KEY_ALIAS:-smart_ingredients_release}"

keytool -genkeypair \
  -v \
  -storetype PKCS12 \
  -keystore "${keystore_file}" \
  -alias "${key_alias}" \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -storepass "${keystore_password}" \
  -keypass "${key_password}" \
  -dname "CN=Smart Ingredients, OU=Development, O=Smart Ingredients, L=Beijing, ST=Beijing, C=CN"

cat >"${keystore_props}" <<EOF
# 签名配置文件
# 请将此文件添加到 .gitignore 中，不要提交到版本控制

# 密钥库文件路径（相对于 gen/android 目录）
storeFile=release-key.jks

# 密钥库密码
storePassword=${keystore_password}

# 密钥别名
keyAlias=${key_alias}

# 密钥密码
keyPassword=${key_password}
EOF

echo "Keystore generated at: ${keystore_file}"
echo "Keystore config updated: ${keystore_props}"
