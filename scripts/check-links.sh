#!/usr/bin/env bash
# Checks that all internal markdown links in docs/ resolve to real files.
# Does not follow external HTTP links.
set -euo pipefail

DOCS_DIR="${1:-docs}"
ERRORS=0

while IFS= read -r -d '' file; do
    dir="$(dirname "$file")"
    while IFS= read -r link; do
        # Strip fragment
        target="${link%%\#*}"
        [ -z "$target" ] && continue
        # Build absolute path relative to the linking file
        if [[ "$target" == /* ]]; then
            full="$DOCS_DIR${target}"
        else
            full="$dir/$target"
        fi
        # Add .md if no extension
        if [[ "$full" != *.md ]] && [[ "$full" != *.png ]] && [[ "$full" != *.svg ]]; then
            full="${full}.md"
        fi
        if [ ! -f "$full" ]; then
            echo "BROKEN: $file -> $link (resolved: $full)"
            ERRORS=$((ERRORS + 1))
        fi
    done < <(grep -oP '\]\(\K[^)]+' "$file" | grep -v '^http')
done < <(find "$DOCS_DIR" -name '*.md' -print0)

if [ "$ERRORS" -gt 0 ]; then
    echo "$ERRORS broken link(s) found."
    exit 1
else
    echo "All internal links OK."
fi
