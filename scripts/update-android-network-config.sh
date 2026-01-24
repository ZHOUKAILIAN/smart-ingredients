#!/usr/bin/env bash
set -euo pipefail

api_base="${API_BASE:-http://127.0.0.1:3000}"
api_base="$(printf "%s" "$api_base" | tr -d '\r\n')"
host="${api_base#http://}"
host="${host#https://}"
host="${host%%/*}"
host="${host%%:*}"

config_path="frontend/src-tauri/gen/android/app/src/main/res/xml/network_security_config.xml"
if [[ ! -f "$config_path" ]]; then
  exit 0
fi

cat >"$config_path" <<EOF
<network-security-config>
  <domain-config cleartextTrafficPermitted="true">
    <domain includeSubdomains="true">${host}</domain>
  </domain-config>
</network-security-config>
EOF
