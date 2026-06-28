# Maintainer Guidelines

## Responsibilities

Extension maintainers are responsible for:

- Keeping the extension compatible with supported core API versions.
- Responding to security disclosures within 72 hours of notification.
- Updating the `ExtensionOwnership` record when maintainers change.
- Participating in RFC reviews when the RFC touches their extension's API.
- Ensuring the extension passes CI before every release.

## Publishing a New Version

1. Update the extension manifest with the new version and any changed API
   version ranges.
2. Run `cargo test -p <ext-crate>` locally.
3. Open a PR; CI runs the extension stability checks automatically.
4. After approval, tag the release: `git tag ext-name-vX.Y.Z`.

## Stepping Down

If you can no longer maintain an extension:

1. Notify the core team via a GitHub issue.
2. Update the `ExtensionOwnership` record to reflect the new lead.
3. If no replacement is found, the extension is marked `Deprecated` in the
   lifecycle registry and removed after one release cycle.

## Contact

Maintainer records are stored in the `ExtensionOwnership` struct and tracked
in the ecosystem registry.
