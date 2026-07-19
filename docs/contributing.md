# Contributing to Ancora documentation

Thank you for helping improve the Ancora documentation. This page covers
the conventions for writing and reviewing pages in the `docs/` directory.
For the general contribution process (branching, commits, CI, and pull
requests), see the repository-level
[contributing guide](../CONTRIBUTING.md).

## File structure

```
docs/
  concepts/       # Language-neutral concept pages
  guides/         # How-to guides
  quickstarts/    # Per-language quickstart walkthroughs
  sdk/            # Per-language SDK index pages
  spec/           # Architecture specs and ADRs
```

Place new pages in the directory that matches their purpose, and keep
each page focused on a single concept.

## Writing conventions

- Use sentence case for headings, not title case.
- Use plain hyphens, never em dashes. A single hyphen (`-`) marks a
  parenthetical aside; two hyphens (`--`) mark a range.
- Do not leave trailing whitespace at the end of lines.
- Write links in `[text](path)` form; never use bare URLs.
- Prefer concrete examples over abstract descriptions.
- Give every page exactly one H1 heading, matching its entry in the
  `mkdocs.yml` navigation.

## Internal links

Always use relative paths from the linking file:

```markdown
[Agents](../concepts/agents.md)
```

Do not use absolute paths (`/concepts/agents.md`) or bare filenames
(`agents.md`) when linking from a different directory.

## Previewing locally

The site is built with MkDocs. To preview your changes with live reload:

```bash
mkdocs serve
```

Then open the local address it prints (by default `http://127.0.0.1:8000`).

## Validating your changes

Run the link checker:

```bash
bash scripts/check-links.sh
```

Run the style checker:

```bash
bash scripts/check-style.sh
```

Both scripts exit with status 0 on success and 1 on violations. CI runs
the same checks, so passing them locally means your pull request will
pass the documentation checks.

## Pull request checklist

- [ ] All new internal links resolve to real files (`check-links.sh` passes)
- [ ] No em dashes and no trailing whitespace (`check-style.sh` passes)
- [ ] New pages are added to the `mkdocs.yml` navigation
- [ ] Each page has a single H1 heading matching its nav entry
