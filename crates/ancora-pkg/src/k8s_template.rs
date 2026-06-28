//! Kubernetes deployment template with operator support.
//!
//! Generates Kubernetes manifests including Deployment, Service, NetworkPolicy,
//! and a custom resource for the Ancora operator.

use std::collections::HashMap;

/// Resource requests and limits for a container.
#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub cpu_request: String,
    pub cpu_limit: String,
    pub memory_request: String,
    pub memory_limit: String,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu_request: "100m".to_string(),
            cpu_limit: "500m".to_string(),
            memory_request: "128Mi".to_string(),
            memory_limit: "512Mi".to_string(),
        }
    }
}

/// Pod security policy settings.
#[derive(Debug, Clone)]
pub struct PodSecurity {
    pub run_as_non_root: bool,
    pub run_as_user: u64,
    pub allow_privilege_escalation: bool,
    pub read_only_root_filesystem: bool,
}

impl Default for PodSecurity {
    fn default() -> Self {
        Self {
            run_as_non_root: true,
            run_as_user: 65534,
            allow_privilege_escalation: false,
            read_only_root_filesystem: true,
        }
    }
}

/// Configuration for a Kubernetes deployment template.
#[derive(Debug, Clone)]
pub struct K8sConfig {
    pub product_name: String,
    pub namespace: String,
    pub image: String,
    pub replicas: u32,
    pub service_port: u16,
    pub labels: HashMap<String, String>,
    pub resources: ResourceRequirements,
    pub pod_security: PodSecurity,
    pub network_policy_enabled: bool,
    pub operator_enabled: bool,
}

impl K8sConfig {
    pub fn new(
        product_name: impl Into<String>,
        namespace: impl Into<String>,
        image: impl Into<String>,
    ) -> Self {
        let product_name = product_name.into();
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), product_name.clone());
        labels.insert("managed-by".to_string(), "ancora-operator".to_string());
        Self {
            product_name,
            namespace: namespace.into(),
            image: image.into(),
            replicas: 2,
            service_port: 8080,
            labels,
            resources: ResourceRequirements::default(),
            pod_security: PodSecurity::default(),
            network_policy_enabled: true,
            operator_enabled: true,
        }
    }

    pub fn with_replicas(mut self, n: u32) -> Self {
        self.replicas = n;
        self
    }

    pub fn with_service_port(mut self, port: u16) -> Self {
        self.service_port = port;
        self
    }
}

/// Rendered Kubernetes manifest bundle.
#[derive(Debug, Clone)]
pub struct K8sTemplate {
    pub config: K8sConfig,
    pub deployment_yaml: String,
    pub service_yaml: String,
    pub network_policy_yaml: Option<String>,
    pub operator_cr_yaml: Option<String>,
}

impl K8sTemplate {
    pub fn render(config: K8sConfig) -> Result<Self, K8sError> {
        if config.product_name.is_empty() {
            return Err(K8sError::InvalidConfig("product_name is required".to_string()));
        }
        if config.image.is_empty() {
            return Err(K8sError::InvalidConfig("image is required".to_string()));
        }

        let deployment_yaml = format!(
            "apiVersion: apps/v1\n\
             kind: Deployment\n\
             metadata:\n\
             \x20\x20name: {name}\n\
             \x20\x20namespace: {ns}\n\
             spec:\n\
             \x20\x20replicas: {replicas}\n\
             \x20\x20template:\n\
             \x20\x20\x20\x20spec:\n\
             \x20\x20\x20\x20\x20\x20securityContext:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20runAsNonRoot: {nonroot}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20runAsUser: {uid}\n\
             \x20\x20\x20\x20\x20\x20containers:\n\
             \x20\x20\x20\x20\x20\x20- name: {name}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20image: {image}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20securityContext:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20allowPrivilegeEscalation: {priv_esc}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20readOnlyRootFilesystem: {ro_root}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20resources:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20requests:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20cpu: {cpu_req}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20memory: {mem_req}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20limits:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20cpu: {cpu_lim}\n\
             \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20memory: {mem_lim}\n",
            name = config.product_name,
            ns = config.namespace,
            replicas = config.replicas,
            nonroot = config.pod_security.run_as_non_root,
            uid = config.pod_security.run_as_user,
            image = config.image,
            priv_esc = config.pod_security.allow_privilege_escalation,
            ro_root = config.pod_security.read_only_root_filesystem,
            cpu_req = config.resources.cpu_request,
            mem_req = config.resources.memory_request,
            cpu_lim = config.resources.cpu_limit,
            mem_lim = config.resources.memory_limit,
        );

        let service_yaml = format!(
            "apiVersion: v1\n\
             kind: Service\n\
             metadata:\n\
             \x20\x20name: {name}-svc\n\
             \x20\x20namespace: {ns}\n\
             spec:\n\
             \x20\x20selector:\n\
             \x20\x20\x20\x20app: {name}\n\
             \x20\x20ports:\n\
             \x20\x20- port: {port}\n\
             \x20\x20\x20\x20targetPort: {port}\n",
            name = config.product_name,
            ns = config.namespace,
            port = config.service_port,
        );

        let network_policy_yaml = if config.network_policy_enabled {
            Some(format!(
                "apiVersion: networking.k8s.io/v1\n\
                 kind: NetworkPolicy\n\
                 metadata:\n\
                 \x20\x20name: {name}-netpol\n\
                 \x20\x20namespace: {ns}\n\
                 spec:\n\
                 \x20\x20podSelector:\n\
                 \x20\x20\x20\x20matchLabels:\n\
                 \x20\x20\x20\x20\x20\x20app: {name}\n\
                 \x20\x20policyTypes:\n\
                 \x20\x20- Ingress\n\
                 \x20\x20- Egress\n",
                name = config.product_name,
                ns = config.namespace,
            ))
        } else {
            None
        };

        let operator_cr_yaml = if config.operator_enabled {
            Some(format!(
                "apiVersion: ancora.io/v1alpha1\n\
                 kind: AncoraAgent\n\
                 metadata:\n\
                 \x20\x20name: {name}\n\
                 \x20\x20namespace: {ns}\n\
                 spec:\n\
                 \x20\x20replicas: {replicas}\n\
                 \x20\x20image: {image}\n\
                 \x20\x20secureDefaults: true\n",
                name = config.product_name,
                ns = config.namespace,
                replicas = config.replicas,
                image = config.image,
            ))
        } else {
            None
        };

        Ok(Self {
            config,
            deployment_yaml,
            service_yaml,
            network_policy_yaml,
            operator_cr_yaml,
        })
    }

    pub fn all_manifests(&self) -> String {
        let mut out = self.deployment_yaml.clone();
        out.push_str("---\n");
        out.push_str(&self.service_yaml);
        if let Some(np) = &self.network_policy_yaml {
            out.push_str("---\n");
            out.push_str(np);
        }
        if let Some(cr) = &self.operator_cr_yaml {
            out.push_str("---\n");
            out.push_str(cr);
        }
        out
    }
}

/// Errors for Kubernetes template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum K8sError {
    InvalidConfig(String),
}

impl std::fmt::Display for K8sError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            K8sError::InvalidConfig(msg) => write!(f, "K8sError: {}", msg),
        }
    }
}

impl std::error::Error for K8sError {}
