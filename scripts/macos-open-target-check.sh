#!/usr/bin/env bash
set -euo pipefail

APP_NAME="${APP_NAME:-}"
APP_NAME_CANDIDATES="${APP_NAME_CANDIDATES:-Float,float-tauri}"
APP_MENU_NAME="${APP_MENU_NAME:-Float}"
EXPECTED_PATH="${1:?usage: macos-open-target-check.sh /absolute/path/to/image}"
EXPECTED_NAME="$(basename "$EXPECTED_PATH")"
STARTUP_TIMEOUT="${STARTUP_TIMEOUT:-90}"
ACTION_TIMEOUT="${ACTION_TIMEOUT:-15}"

osascript_run() {
  osascript "$@"
}

resolve_app_name() {
  if [[ -n "$APP_NAME" ]]; then
    echo "$APP_NAME"
    return 0
  fi

  local candidate
  IFS=',' read -r -a candidates <<<"$APP_NAME_CANDIDATES"
  for candidate in "${candidates[@]}"; do
    candidate="${candidate#"${candidate%%[![:space:]]*}"}"
    candidate="${candidate%"${candidate##*[![:space:]]}"}"
    [[ -z "$candidate" ]] && continue
    if [[ "$(osascript_run -e "tell application \"System Events\" to (exists process \"$candidate\")" | tr -d '\r')" == "true" ]]; then
      APP_NAME="$candidate"
      echo "$APP_NAME"
      return 0
    fi
  done

  return 1
}

activate_app() {
  resolve_app_name >/dev/null
  osascript_run <<OSA >/dev/null
tell application "System Events"
  tell process "$APP_NAME"
    set frontmost to true
  end tell
end tell
OSA
}

send_command_key() {
  local key="$1"
  osascript_run <<OSA >/dev/null
tell application "System Events"
  keystroke "$key" using command down
end tell
OSA
}

click_menu_item() {
  local menu_name="$1"
  local item_name="$2"
  resolve_app_name >/dev/null
  osascript_run <<OSA >/dev/null
tell application "System Events"
  tell process "$APP_NAME"
    click menu item "$item_name" of menu "$menu_name" of menu bar item "$menu_name" of menu bar 1
  end tell
end tell
OSA
}

window_titles() {
  osascript_run <<OSA | tr -d '\r'
tell application "System Events"
  tell process "$APP_NAME"
    if not (exists window 1) then
      return ""
    end if
    set titles to {}
    repeat with currentWindow in windows
      set end of titles to name of currentWindow
    end repeat
    set AppleScript's text item delimiters to linefeed
    return titles as text
  end tell
end tell
OSA
}

front_window_title() {
  osascript_run <<OSA | tr -d '\r'
tell application "System Events"
  tell process "$APP_NAME"
    if not (exists window 1) then
      return ""
    end if
    return name of window 1
  end tell
end tell
OSA
}

window_count() {
  local titles
  titles="$(window_titles)"
  if [[ -z "$titles" ]]; then
    echo 0
  else
    awk 'NF { count += 1 } END { print count + 0 }' <<<"$titles"
  fi
}

wait_for_process() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT))
  while (( SECONDS < deadline )); do
    if resolve_app_name >/dev/null; then
      return 0
    fi
    sleep 1
  done

  echo "Timed out waiting for a usable app process. Tried: ${APP_NAME:-<unset>} ${APP_NAME_CANDIDATES}" >&2
  return 1
}

wait_for_window_count() {
  local expected_count="$1"
  local deadline=$((SECONDS + ACTION_TIMEOUT))
  while (( SECONDS < deadline )); do
    if [[ "$(window_count)" -eq "$expected_count" ]]; then
      return 0
    fi
    sleep 0.25
  done

  echo "Timed out waiting for $expected_count windows." >&2
  return 1
}

wait_for_front_window_title() {
  local expected_fragment="$1"
  local deadline=$((SECONDS + ACTION_TIMEOUT))
  while (( SECONDS < deadline )); do
    local current_title
    current_title="$(front_window_title)"
    if [[ "$current_title" == *"$expected_fragment"* ]]; then
      return 0
    fi
    sleep 0.25
  done

  echo "Timed out waiting for the front window title to contain \"$expected_fragment\"." >&2
  return 1
}

print_summary() {
  local titles
  titles="$(window_titles)"
  echo "Front window: $(front_window_title)"
  echo "All window titles:"
  if [[ -n "$titles" ]]; then
    while IFS= read -r line; do
      [[ -n "$line" ]] && echo "  - $line"
    done <<<"$titles"
  fi
}

main() {
  if [[ "$EXPECTED_PATH" != /* ]]; then
    echo "Expected an absolute image path, got: $EXPECTED_PATH" >&2
    exit 1
  fi

  wait_for_process
  activate_app
  sleep 0.5

  local initial_count
  initial_count="$(window_count)"
  if [[ "$initial_count" -lt 1 ]]; then
    echo "Float is running, but no windows are visible." >&2
    exit 1
  fi

  click_menu_item "$APP_MENU_NAME" "New Window"
  wait_for_window_count "$((initial_count + 1))"
  sleep 0.4

  click_menu_item "$APP_MENU_NAME" "Open…"
  wait_for_front_window_title "$EXPECTED_NAME"

  echo "Open-target check passed."
  print_summary
}

main "$@"
