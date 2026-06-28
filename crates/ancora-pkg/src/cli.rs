//! Packaging CLI for scaffolding Ancora products.
//!
//! Provides a programmatic interface for the `ancora-pkg` command-line tool
//! that scaffolds new product deployments from templates.

use crate::{
    airgap_template::{AirgapConfig, AirgapTemplate, ArtifactSource},
    compose_template::{ComposeConfig, ComposeService, ComposeTemplate},
    edge_template::{EdgeArch, EdgeConfig, EdgeTemplate},
    k8s_template::{K8sConfig, K8sTemplate},
    onprem_template::{OnPremConfig, OnPremTemplate},
    saas_template::{SaasConfig, SaasTemplate, SaasTier},
    tenant_onboard::{TenantOnboardConfig, TenantOnboardTemplate, TenantTier},
    whitelabel::{BrandIdentity, WhitelabelConfig, WhitelabelTemplate},
};

/// The kind of product template to scaffold.
#[derive(Debug, Clone, PartialEq)]
pub enum ScaffoldKind {
    Saas,
    OnPrem,
    Airgap,
    Compose,
    Kubernetes,
    Edge,
    Whitelabel,
    TenantOnboard,
}

impl ScaffoldKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "saas" => Some(ScaffoldKind::Saas),
            "onprem" => Some(ScaffoldKind::OnPrem),
            "airgap" => Some(ScaffoldKind::Airgap),
            "compose" => Some(ScaffoldKind::Compose),
            "k8s" | "kubernetes" => Some(ScaffoldKind::Kubernetes),
            "edge" => Some(ScaffoldKind::Edge),
            "whitelabel" => Some(ScaffoldKind::Whitelabel),
            "tenant" => Some(ScaffoldKind::TenantOnboard),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ScaffoldKind::Saas => "saas",
            ScaffoldKind::OnPrem => "onprem",
            ScaffoldKind::Airgap => "airgap",
            ScaffoldKind::Compose => "compose",
            ScaffoldKind::Kubernetes => "kubernetes",
            ScaffoldKind::Edge => "edge",
            ScaffoldKind::Whitelabel => "whitelabel",
            ScaffoldKind::TenantOnboard => "tenant",
        }
    }
}

/// Arguments for the scaffold command.
#[derive(Debug, Clone)]
pub struct ScaffoldArgs {
    pub kind: ScaffoldKind,
    pub product_name: String,
    pub output_dir: String,
    pub extra: std::collections::HashMap<String, String>,
}

impl ScaffoldArgs {
    pub fn new(kind: ScaffoldKind, product_name: impl Into<String>) -> Self {
        Self {
            kind,
            product_name: product_name.into(),
            output_dir: "./scaffold-output".to_string(),
            extra: std::collections::HashMap::new(),
        }
    }

    pub fn with_output(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    pub fn with_extra(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.extra.insert(key.into(), val.into());
        self
    }
}

/// Output produced by a scaffold operation.
#[derive(Debug, Clone)]
pub struct ScaffoldOutput {
    pub kind: ScaffoldKind,
    pub files: Vec<ScaffoldFile>,
    pub summary: String,
}

/// A single file produced by scaffold.
#[derive(Debug, Clone)]
pub struct ScaffoldFile {
    pub path: String,
    pub content: String,
}

impl ScaffoldOutput {
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn find_file(&self, name: &str) -> Option<&ScaffoldFile> {
        self.files.iter().find(|f| f.path.contains(name))
    }
}

/// CLI scaffolding engine.
pub struct PackagingCli;

impl PackagingCli {
    /// Runs the scaffold command and returns the produced output.
    pub fn scaffold(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        if args.product_name.is_empty() {
            return Err(CliError::MissingArg("product_name".to_string()));
        }

        match args.kind.clone() {
            ScaffoldKind::Saas => Self::scaffold_saas(args),
            ScaffoldKind::OnPrem => Self::scaffold_onprem(args),
            ScaffoldKind::Airgap => Self::scaffold_airgap(args),
            ScaffoldKind::Compose => Self::scaffold_compose(args),
            ScaffoldKind::Kubernetes => Self::scaffold_k8s(args),
            ScaffoldKind::Edge => Self::scaffold_edge(args),
            ScaffoldKind::Whitelabel => Self::scaffold_whitelabel(args),
            ScaffoldKind::TenantOnboard => Self::scaffold_tenant(args),
        }
    }

    fn scaffold_saas(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let region = args.extra.get("region").cloned().unwrap_or_else(|| "us-east-1".to_string());
        let config = SaasConfig::new(args.product_name.clone(), SaasTier::Production, region);
        let template = SaasTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::Saas,
            files: vec![ScaffoldFile {
                path: format!("{}/deployment.yaml", args.output_dir),
                content: template.rendered_yaml,
            }],
            summary: format!("Scaffolded SaaS deployment for {}", args.product_name),
        })
    }

    fn scaffold_onprem(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let hostname = args.extra.get("hostname").cloned().unwrap_or_else(|| "ancora.local".to_string());
        let config = OnPremConfig::new(args.product_name.clone(), hostname, 3);
        let template = OnPremTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::OnPrem,
            files: vec![ScaffoldFile {
                path: format!("{}/appliance.yaml", args.output_dir),
                content: template.rendered,
            }],
            summary: format!("Scaffolded on-prem appliance for {}", args.product_name),
        })
    }

    fn scaffold_airgap(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let registry = args.extra.get("registry").cloned().unwrap_or_else(|| "registry.local:5000".to_string());
        let config = AirgapConfig::new(
            args.product_name.clone(),
            "1.0.0",
            ArtifactSource::LocalRegistry(registry),
            "/etc/ancora/license.key",
        );
        let template = AirgapTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::Airgap,
            files: vec![ScaffoldFile {
                path: format!("{}/airgap.yaml", args.output_dir),
                content: template.rendered,
            }],
            summary: format!("Scaffolded air-gapped deployment for {}", args.product_name),
        })
    }

    fn scaffold_compose(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let svc = ComposeService::new(args.product_name.clone(), "ancora/agent:latest")
            .with_port(8080, 8080);
        let config = ComposeConfig::new(args.product_name.clone()).add_service(svc);
        let template = ComposeTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::Compose,
            files: vec![ScaffoldFile {
                path: format!("{}/docker-compose.yml", args.output_dir),
                content: template.rendered,
            }],
            summary: format!("Scaffolded Docker Compose for {}", args.product_name),
        })
    }

    fn scaffold_k8s(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let namespace = args.extra.get("namespace").cloned().unwrap_or_else(|| "ancora".to_string());
        let image = args.extra.get("image").cloned().unwrap_or_else(|| "ancora/agent:latest".to_string());
        let config = K8sConfig::new(args.product_name.clone(), namespace, image);
        let template = K8sTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::Kubernetes,
            files: vec![ScaffoldFile {
                path: format!("{}/manifests.yaml", args.output_dir),
                content: template.all_manifests(),
            }],
            summary: format!("Scaffolded Kubernetes manifests for {}", args.product_name),
        })
    }

    fn scaffold_edge(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let config = EdgeConfig::new(args.product_name.clone(), "1.0.0", EdgeArch::X86_64);
        let template = EdgeTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::Edge,
            files: vec![
                ScaffoldFile {
                    path: format!("{}/build.yaml", args.output_dir),
                    content: template.build_spec,
                },
                ScaffoldFile {
                    path: format!("{}/runtime.yaml", args.output_dir),
                    content: template.runtime_config,
                },
            ],
            summary: format!("Scaffolded edge binary spec for {}", args.product_name),
        })
    }

    fn scaffold_whitelabel(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let domain = args.extra.get("domain").cloned().unwrap_or_else(|| "partner.example.com".to_string());
        let brand = BrandIdentity::new(args.product_name.clone());
        let config = WhitelabelConfig::new(brand, domain);
        let template = WhitelabelTemplate::apply(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::Whitelabel,
            files: vec![ScaffoldFile {
                path: format!("{}/whitelabel.yaml", args.output_dir),
                content: template.rendered,
            }],
            summary: format!("Scaffolded white-label config for {}", args.product_name),
        })
    }

    fn scaffold_tenant(args: ScaffoldArgs) -> Result<ScaffoldOutput, CliError> {
        let email = args.extra.get("admin_email").cloned().unwrap_or_else(|| "admin@example.com".to_string());
        let tenant_id = args.product_name.to_lowercase().replace(' ', "-");
        let config = TenantOnboardConfig::new(tenant_id, args.product_name.clone(), TenantTier::Business, email);
        let template = TenantOnboardTemplate::render(config).map_err(|e| CliError::TemplateError(e.to_string()))?;
        Ok(ScaffoldOutput {
            kind: ScaffoldKind::TenantOnboard,
            files: vec![ScaffoldFile {
                path: format!("{}/tenant.yaml", args.output_dir),
                content: template.rendered,
            }],
            summary: format!("Scaffolded tenant onboarding for {}", args.product_name),
        })
    }
}

/// Errors produced by the packaging CLI.
#[derive(Debug, Clone, PartialEq)]
pub enum CliError {
    MissingArg(String),
    TemplateError(String),
    UnknownKind(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingArg(arg) => write!(f, "CliError: missing argument '{}'", arg),
            CliError::TemplateError(msg) => write!(f, "CliError: template error: {}", msg),
            CliError::UnknownKind(kind) => write!(f, "CliError: unknown scaffold kind '{}'", kind),
        }
    }
}

impl std::error::Error for CliError {}
