use crate::whitelabel::{
    BrandIdentity, ColorScheme, WhitelabelConfig, WhitelabelError, WhitelabelTemplate,
};

#[test]
fn test_whitelabel_basic_apply() {
    let brand = BrandIdentity::new("Acme Corp");
    let config = WhitelabelConfig::new(brand, "acme.example.com");
    let tmpl = WhitelabelTemplate::apply(config).expect("should apply");
    assert!(tmpl.contains("Acme Corp"));
    assert!(tmpl.contains("acme.example.com"));
}

#[test]
fn test_whitelabel_security_unchanged() {
    let brand = BrandIdentity::new("Partner Co");
    let config = WhitelabelConfig::new(brand, "partner.co");
    let tmpl = WhitelabelTemplate::apply(config).expect("should apply");
    assert!(
        tmpl.contains("branding_does_not_affect_security: true"),
        "security settings must not be overridden by branding"
    );
}

#[test]
fn test_whitelabel_custom_colors() {
    let brand = BrandIdentity::new("ColorBrand");
    let colors = ColorScheme {
        primary: "#ff0000".to_string(),
        secondary: "#00ff00".to_string(),
        accent: "#0000ff".to_string(),
        background: "#ffffff".to_string(),
    };
    let config = WhitelabelConfig::new(brand, "colorbrand.com").with_colors(colors);
    let tmpl = WhitelabelTemplate::apply(config).expect("should apply");
    assert!(tmpl.contains("#ff0000"));
    assert!(tmpl.contains("#00ff00"));
    assert!(tmpl.contains("#0000ff"));
}

#[test]
fn test_whitelabel_feature_override() {
    let brand = BrandIdentity::new("FeatBrand");
    let config = WhitelabelConfig::new(brand, "feat.brand.com")
        .with_feature_override("advanced_analytics", false)
        .with_feature_override("white_glove_support", true);
    let tmpl = WhitelabelTemplate::apply(config).expect("should apply");
    assert!(tmpl.contains("advanced_analytics: false"));
    assert!(tmpl.contains("white_glove_support: true"));
}

#[test]
fn test_whitelabel_empty_brand_fails() {
    let brand = BrandIdentity::new("");
    let config = WhitelabelConfig::new(brand, "brand.com");
    let err = WhitelabelTemplate::apply(config).unwrap_err();
    assert!(matches!(err, WhitelabelError::ValidationFailed(_)));
}

#[test]
fn test_whitelabel_empty_domain_fails() {
    let brand = BrandIdentity::new("SomeBrand");
    let config = WhitelabelConfig::new(brand, "");
    let err = WhitelabelTemplate::apply(config).unwrap_err();
    assert!(matches!(err, WhitelabelError::ValidationFailed(_)));
}

#[test]
fn test_whitelabel_validation_passes() {
    let brand = BrandIdentity::new("Valid Brand");
    let config = WhitelabelConfig::new(brand, "valid.com");
    let validation = WhitelabelTemplate::validate(&config);
    assert!(validation.valid);
    assert!(validation.errors.is_empty());
}

#[test]
fn test_whitelabel_with_logo_and_support() {
    let brand = BrandIdentity::new("Full Brand")
        .with_logo("https://full.brand/logo.svg")
        .with_support("support@full.brand");
    let config = WhitelabelConfig::new(brand, "full.brand");
    let tmpl = WhitelabelTemplate::apply(config).expect("should apply");
    assert!(tmpl.contains("https://full.brand/logo.svg"));
    assert!(tmpl.contains("support@full.brand"));
}
