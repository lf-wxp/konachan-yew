#!/bin/sh
# ---------------------------------------------------------------------------
# nginx entrypoint hook (run from /docker-entrypoint.d before nginx starts).
#
# Regenerates the runtime configuration consumed by the WASM bundle so a single
# image can serve both the regular and the safe (content-filtered) mode based
# on environment variables:
#
#   KONACHAN_SAFE  "true" / "1" / "yes" / "on"  -> safe mode
#                  anything else (or unset)     -> regular mode
# ---------------------------------------------------------------------------
set -eu

HTML_ROOT="${HTML_ROOT:-/usr/share/nginx/html}"

case "$(printf '%s' "${KONACHAN_SAFE:-}" | tr '[:upper:]' '[:lower:]')" in
  1 | true | yes | on) safe="true" ;;
  *) safe="false" ;;
esac

cat >"${HTML_ROOT}/config.js" <<EOF
window.__KONACHAN_CONFIG__ = {
  safe: ${safe}
};
EOF

echo "konachan: wrote runtime config to ${HTML_ROOT}/config.js (safe=${safe})"
