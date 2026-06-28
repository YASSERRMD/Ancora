# White-Label Guide

ancora-pkg supports full OEM / partner white-labelling. Partners can rebrand
the product with their own name, logo, colours, and domain.

## What Can Be Customised

- Brand name and logo URL
- Favicon
- Colour scheme (primary, secondary, accent, background)
- Custom domain
- Support email and documentation URL
- Feature flag overrides per partner

## What Cannot Be Customised

Security controls are immutable regardless of white-label configuration:

- TLS requirements
- Audit logging
- Data isolation boundaries
- Access control policies

## Example

```rust
use ancora_pkg::whitelabel::{BrandIdentity, ColorScheme, WhitelabelConfig, WhitelabelTemplate};

let brand = BrandIdentity::new("Acme Agent")
    .with_logo("https://acme.example.com/logo.svg")
    .with_support("support@acme.example.com");

let colors = ColorScheme {
    primary: "#1a2e4a".to_string(),
    secondary: "#2d4a6e".to_string(),
    accent: "#f59e0b".to_string(),
    background: "#ffffff".to_string(),
};

let config = WhitelabelConfig::new(brand, "agent.acme.example.com")
    .with_colors(colors)
    .with_feature_override("advanced_analytics", true);

let template = WhitelabelTemplate::apply(config).unwrap();
println!("{}", template.rendered);
```

## Validation

The `WhitelabelTemplate::validate` function checks that all required fields
are present before applying. Call it before `apply` if you want to surface
errors to the user before committing.
