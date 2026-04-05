#!/usr/bin/env bash
set -euo pipefail

readonly CONVENTIONAL_COMMIT_PATTERN='^((fixup|squash)! )?(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test)(\([a-z0-9._/-]+\))?(!)?: [^ ].*$'

usage() {
  cat <<'EOF' >&2
Usage:
  scripts/check-conventional-commits.sh --message-file <path>
  scripts/check-conventional-commits.sh <git-range>

Examples:
  scripts/check-conventional-commits.sh --message-file .git/COMMIT_EDITMSG
  scripts/check-conventional-commits.sh origin/main..HEAD
EOF
}

print_examples() {
  cat <<'EOF' >&2
Expected Conventional Commit subjects, for example:
  feat: add slideshow controls
  fix(window-size): clamp restored width
  chore!: drop legacy bundle step
  fixup! docs: update release notes
EOF
}

is_valid_subject() {
  local subject="$1"
  [[ "$subject" =~ $CONVENTIONAL_COMMIT_PATTERN ]]
}

validate_subject() {
  local label="$1"
  local subject="$2"

  if is_valid_subject "$subject"; then
    printf 'ok: %s\n' "$label"
    return 0
  fi

  printf 'invalid: %s\n' "$label" >&2
  printf '  subject: %s\n' "$subject" >&2
  return 1
}

validate_message_file() {
  local message_file="$1"
  local subject

  if [[ ! -f "$message_file" ]]; then
    printf 'Commit message file not found: %s\n' "$message_file" >&2
    exit 1
  fi

  subject="$(git stripspace --strip-comments < "$message_file" | sed -n '1p')"

  if [[ -z "$subject" ]]; then
    printf 'Commit message is empty after stripping comments.\n' >&2
    exit 1
  fi

  if ! validate_subject "commit message" "$subject"; then
    print_examples
    exit 1
  fi
}

validate_range() {
  local range="$1"
  local commits
  local invalid=0

  commits="$(git rev-list --reverse --no-merges "$range")"

  if [[ -z "$commits" ]]; then
    printf 'No non-merge commits to validate in %s\n' "$range"
    return 0
  fi

  while IFS= read -r commit; do
    local short_sha
    local subject

    short_sha="$(git rev-parse --short "$commit")"
    subject="$(git log -1 --format=%s "$commit")"

    if ! validate_subject "$short_sha" "$subject"; then
      invalid=1
    fi
  done <<< "$commits"

  if [[ "$invalid" -ne 0 ]]; then
    print_examples
    exit 1
  fi
}

main() {
  if [[ $# -eq 2 && "$1" == "--message-file" ]]; then
    validate_message_file "$2"
    return 0
  fi

  if [[ $# -eq 1 ]]; then
    validate_range "$1"
    return 0
  fi

  usage
  exit 64
}

main "$@"
