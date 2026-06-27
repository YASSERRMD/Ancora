#!/usr/bin/env bash
# Checks docs/ markdown files for style violations.
# Rules enforced:
#   1. No em dash (-- or Unicode U+2014) -- use a hyphen or two hyphens instead.
#   2. No trailing whitespace on any line.
#   3. No bare HTTP links (must be in [text](url) form).
set -euo pipefail

DOCS_DIR="${1:-docs}"
ERRORS=0

while IFS= read -r -d '' file; do
    # 1. Em dash check (both ASCII -- and Unicode em dash)
    if grep -qP '—|(?<![!])\-\-(?![->\|])' "$file" 2>/dev/null; then
        echo "EM-DASH: $file"
        ERRORS=$((ERRORS + 1))
    fi

    # 2. Trailing whitespace
    if grep -qP '[ \t]+$' "$file"; then
        echo "TRAILING-WS: $file"
        ERRORS=$((ERRORS + 1))
    fi

    # 3. Bare HTTP links (http(s):// not inside markdown link syntax)
    if grep -qP '(?<!\(|"|\')https?://[^\s")\x27]+(?!\))' "$file" 2>/dev/null; then
        echo "BARE-LINK: $file"
        ERRORS=$((ERRORS + 1))
    fi
done < <(find "$DOCS_DIR" -name '*.md' -print0)

if [ "$ERRORS" -gt 0 ]; then
    echo "$ERRORS style violation(s) found."
    exit 1
else
    echo "Style check passed."
fi
