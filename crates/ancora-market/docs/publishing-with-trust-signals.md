# Publishing with Trust Signals

To publish an extension to the Ancora marketplace, authors must supply a complete
set of trust signals. Missing or invalid signals block publishing.

## Required Steps

### 1. Author Identity

Register a verified author identity with the registry. Once verified, the
registry issues an ed25519 key pair. Sign the release bundle using the
`sign_payload` helper and attach the resulting `Signature` to the manifest.

### 2. Security Scan

Run the bundled scanner against your extension artifact:

```
ancora-scanner --output scan-report.json my-extension.tar.gz
```

Attach the `ScanReport` to your manifest. Extensions with Critical or High
severity findings cannot be published until those findings are resolved.

### 3. License Declaration

Include a valid SPDX expression in your `ExtensionMetadata`. Open-source
licenses receive higher trust scores than proprietary licenses. Enterprise
operators may block proprietary extensions via install policy.

### 4. Residency Declaration

Supply a `ResidencyDeclaration` that lists every region where user data may
be stored or processed. Do not use `Region::Unspecified` - operators enforce
this at install time.

### 5. Version and Changelog

Every release must include a `ChangelogEntry` describing what changed. Yanked
versions are hidden from install clients but remain in the audit log.

## Validation Order

The registry validates signals in this order and returns the first error
encountered:

1. Metadata schema (id, name, version, license format)
2. Author identity (verified flag set)
3. Signature (payload matches declared signature)
4. Security scan (no Critical or High findings)
5. License declaration (SPDX expression present)
6. Residency declaration (no Unspecified regions)

Once all validations pass, the trust score is computed and the extension is
listed on the marketplace.
