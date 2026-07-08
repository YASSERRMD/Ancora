//! Docker Compose deployment template.
//!
//! Generates a docker-compose.yml scaffold for local development and
//! single-host deployments, with secure networking defaults.

use std::collections::HashMap;

/// A service definition within a compose file.
#[derive(Debug, Clone)]
pub struct ComposeService {
    pub name: String,
    pub image: String,
    pub ports: Vec<(u16, u16)>,
    pub environment: HashMap<String, String>,
    pub depends_on: Vec<String>,
    pub read_only: bool,
    pub no_new_privileges: bool,
}

impl ComposeService {
    pub fn new(name: impl Into<String>, image: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            image: image.into(),
            ports: vec![],
            environment: HashMap::new(),
            depends_on: vec![],
            read_only: true,
            no_new_privileges: true,
        }
    }

    pub fn with_port(mut self, host: u16, container: u16) -> Self {
        self.ports.push((host, container));
        self
    }

    pub fn with_env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.environment.insert(key.into(), val.into());
        self
    }

    pub fn depends_on(mut self, service: impl Into<String>) -> Self {
        self.depends_on.push(service.into());
        self
    }
}

/// Configuration for a Compose-based deployment.
#[derive(Debug, Clone)]
pub struct ComposeConfig {
    pub project_name: String,
    pub services: Vec<ComposeService>,
    pub networks: Vec<String>,
    pub volumes: Vec<String>,
}

impl ComposeConfig {
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: project_name.into(),
            services: vec![],
            networks: vec!["internal".to_string()],
            volumes: vec![],
        }
    }

    pub fn add_service(mut self, service: ComposeService) -> Self {
        self.services.push(service);
        self
    }

    pub fn add_volume(mut self, volume: impl Into<String>) -> Self {
        self.volumes.push(volume.into());
        self
    }
}

/// Rendered docker-compose template.
#[derive(Debug, Clone)]
pub struct ComposeTemplate {
    pub config: ComposeConfig,
    pub rendered: String,
}

impl ComposeTemplate {
    pub fn render(config: ComposeConfig) -> Result<Self, ComposeError> {
        if config.project_name.is_empty() {
            return Err(ComposeError::InvalidConfig(
                "project_name is required".to_string(),
            ));
        }
        if config.services.is_empty() {
            return Err(ComposeError::InvalidConfig(
                "at least one service is required".to_string(),
            ));
        }

        let mut out = format!(
            "# ancora-pkg docker compose template\n\
             version: '3.9'\n\
             name: {}\n\
             services:\n",
            config.project_name
        );

        for svc in &config.services {
            out.push_str(&format!("  {}:\n", svc.name));
            out.push_str(&format!("    image: {}\n", svc.image));
            out.push_str(&format!("    read_only: {}\n", svc.read_only));
            out.push_str(&format!(
                "    security_opt:\n      - no-new-privileges:{}\n",
                svc.no_new_privileges
            ));
            if !svc.ports.is_empty() {
                out.push_str("    ports:\n");
                for (h, c) in &svc.ports {
                    out.push_str(&format!("      - \"{}:{}\"\n", h, c));
                }
            }
            if !svc.environment.is_empty() {
                out.push_str("    environment:\n");
                for (k, v) in &svc.environment {
                    out.push_str(&format!("      {}: {}\n", k, v));
                }
            }
            if !svc.depends_on.is_empty() {
                out.push_str("    depends_on:\n");
                for dep in &svc.depends_on {
                    out.push_str(&format!("      - {}\n", dep));
                }
            }
            out.push_str("    networks:\n      - internal\n");
        }

        out.push_str("networks:\n");
        for net in &config.networks {
            out.push_str(&format!(
                "  {}:\n    driver: bridge\n    internal: true\n",
                net
            ));
        }

        if !config.volumes.is_empty() {
            out.push_str("volumes:\n");
            for vol in &config.volumes {
                out.push_str(&format!("  {}:\n", vol));
            }
        }

        Ok(Self {
            config,
            rendered: out,
        })
    }

    pub fn contains(&self, field: &str) -> bool {
        self.rendered.contains(field)
    }

    pub fn service_count(&self) -> usize {
        self.config.services.len()
    }
}

/// Errors for compose template rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum ComposeError {
    InvalidConfig(String),
}

impl std::fmt::Display for ComposeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComposeError::InvalidConfig(msg) => write!(f, "ComposeError: {}", msg),
        }
    }
}

impl std::error::Error for ComposeError {}
