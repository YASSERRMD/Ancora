# Contributing to Ancora Documentation

Thank you for contributing. This page describes the conventions for writing
and reviewing docs in the `docs/` directory.

## File structure

```
docs/
  concepts/       # Language-neutral concept pages
  guides/         # How-to guides
  quickstarts/    # Per-language quickstart walkthroughs
  sdk/            # Per-language SDK index pages
  spec/           # Architecture specs and ADRs
```

## Writing conventions

- Use sentence case for headings (not title case).
- Use plain hyphens, not em dashes. A single hyphen (`-`) for
  parenthetical asides; two hyphens (`--`) for ranges.
- Do not leave trailing whitespace at the end of lines.
- Wrap links in `[text](path)` form; never use bare URLs.
- Prefer concrete examples over abstract descriptions.
- Keep pages focused: one concept per file.

## Internal links

Always use relative paths from the linking file:

```markdown
[Agents](../concepts/agents.md)
```

Not absolute paths (`/concepts/agents.md`) and not bare filenames
(`agents.md` from a different directory).

## Validating your changes

Run the link checker:

```bash
bash scripts/check-links.sh
```

Run the style checker:

```bash
bash scripts/check-style.sh
```

Both scripts exit with status 0 on success and 1 on violations.

## Pull request checklist

- [ ] All new internal links resolve to real files (`check-links.sh` passes)
- [ ] No em dashes, no trailing whitespace (`check-style.sh` passes)
- [ ] New concept pages are added to `mkdocs.yml` nav
- [ ] Each page has a single H1 heading matching its nav entry
