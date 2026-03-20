#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOCALES_FILE="$ROOT_DIR/assets/i18n/locales.json"
SOURCE_FILE="$ROOT_DIR/assets/i18n/en.json"
MIN_BATCH_SIZE=500
BATCH_SIZE="${I18N_BATCH_SIZE:-$MIN_BATCH_SIZE}"

log() {
  printf '[i18n] %s\n' "$*"
}

fail() {
  printf '[i18n] error: %s\n' "$*" >&2
  exit 1
}

ensure_batch_size() {
  case "$BATCH_SIZE" in
    ''|*[!0-9]*)
      fail "I18N_BATCH_SIZE must be an integer (got: $BATCH_SIZE)"
      ;;
  esac
  if [ "$BATCH_SIZE" -lt "$MIN_BATCH_SIZE" ]; then
    fail "I18N_BATCH_SIZE must be at least $MIN_BATCH_SIZE (got: $BATCH_SIZE)"
  fi
}

ensure_codex() {
  if command -v codex >/dev/null 2>&1; then
    return
  fi
  if command -v npm >/dev/null 2>&1; then
    log "installing Codex CLI via npm"
    npm i -g @openai/codex || fail "failed to install Codex CLI via npm"
  elif command -v brew >/dev/null 2>&1; then
    log "installing Codex CLI via brew"
    brew install codex || fail "failed to install Codex CLI via brew"
  else
    fail "Codex CLI not found and no supported installer available"
  fi
}

ensure_codex_login() {
  if codex login status >/dev/null 2>&1; then
    return
  fi
  log "Codex login status unavailable or not logged in; starting login flow"
  codex login || fail "Codex login failed"
}

probe_translator() {
  command -v greentic-i18n-translator >/dev/null 2>&1 || fail "greentic-i18n-translator not found"
  greentic-i18n-translator translate --help >/dev/null 2>&1 || fail "translator subcommand 'translate' is required"
}

run_translate() {
  while IFS= read -r locale; do
    [[ -n "$locale" ]] || continue
    log "translating locale: $locale (batch size: $BATCH_SIZE)"
    greentic-i18n-translator translate --langs "$locale" --en "$SOURCE_FILE" --batch-size "$BATCH_SIZE" || fail "translate failed for locale $locale"
  done < <(python3 - "$LOCALES_FILE" <<'PY'
import json
import sys
with open(sys.argv[1], 'r', encoding='utf-8') as f:
    data = json.load(f)
for locale in data:
    if locale != "en":
        print(locale)
PY
)
}

[[ -f "$LOCALES_FILE" ]] || fail "missing locales file: $LOCALES_FILE"
[[ -f "$SOURCE_FILE" ]] || fail "missing source locale file: $SOURCE_FILE"

ensure_codex
ensure_codex_login
ensure_batch_size
probe_translator
run_translate
log "translations updated. Run cargo build to embed translations into WASM"
