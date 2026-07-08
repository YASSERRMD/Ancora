/// Feature matrix for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Support {
    Full,
    Partial(String),
    Planned(String),
    NotSupported,
}

#[derive(Debug, Clone)]
pub struct FeatureRow {
    pub feature: String,
    pub community: Support,
    pub enterprise: Support,
}

impl FeatureRow {
    pub fn full(feature: impl Into<String>) -> Self {
        Self {
            feature: feature.into(),
            community: Support::Full,
            enterprise: Support::Full,
        }
    }

    pub fn enterprise_only(feature: impl Into<String>, _note: impl Into<String>) -> Self {
        Self {
            feature: feature.into(),
            community: Support::NotSupported,
            enterprise: Support::Full,
        }
    }
}

pub fn feature_matrix() -> Vec<FeatureRow> {
    vec![
        FeatureRow::full("plugin-sdk"),
        FeatureRow::full("catalog-search"),
        FeatureRow::full("registry-publish"),
        FeatureRow::full("mcp-tools"),
        FeatureRow::full("structured-output"),
        FeatureRow::full("streaming"),
        FeatureRow::enterprise_only("enterprise-sso", "Requires enterprise license"),
        FeatureRow::enterprise_only("audit-log-export", "Requires enterprise license"),
        FeatureRow::enterprise_only("advanced-rbac", "Requires enterprise license"),
    ]
}
