use crate::crd::cluster::{AncoraCluster, AncoraClusterStatus, ConditionStatus};
use crate::crd::tenant::{AncoraTenant, AncoraTenantStatus};
use crate::fake_k8s::FakeK8s;
use crate::status::{degraded_condition, ready_condition};
use serde_json::json;
use thiserror::Error;
use tracing::info;

const FINALIZER: &str = "ancora.io/cleanup";

#[derive(Debug, Error)]
pub enum ReconcileError {
    #[error("invalid spec: {0}")]
    InvalidSpec(String),
    #[error("admission rejected: {0}")]
    AdmissionRejected(String),
}

pub struct Reconciler {
    k8s: FakeK8s,
}

impl Reconciler {
    pub fn new() -> Self {
        Reconciler {
            k8s: FakeK8s::new(),
        }
    }

    // --- AncoraCluster ---

    pub fn reconcile_cluster(&mut self, cluster: &mut AncoraCluster) -> Result<(), ReconcileError> {
        let name = cluster.metadata.name.clone();
        info!(name = %name, "reconciling AncoraCluster");

        // Add finalizer if not present
        if !cluster.metadata.finalizers.contains(&FINALIZER.to_string()) {
            cluster.metadata.finalizers.push(FINALIZER.to_string());
        }

        // Reconcile control plane deployment
        self.k8s.apply(
            "Deployment",
            &format!("{}-control-plane", name),
            json!({
                "kind": "Deployment",
                "replicas": cluster.spec.control_plane_replicas,
                "image": cluster.spec.image,
            }),
        );

        // Reconcile worker deployment
        self.k8s.apply(
            "Deployment",
            &format!("{}-worker", name),
            json!({
                "kind": "Deployment",
                "replicas": cluster.spec.worker_replicas,
                "image": cluster.spec.image,
                "concurrency": cluster.spec.worker_concurrency,
            }),
        );

        // Reconcile HPA if autoscaler enabled
        if cluster.spec.autoscaler_enabled {
            self.k8s.apply(
                "HorizontalPodAutoscaler",
                &format!("{}-worker-hpa", name),
                json!({
                    "min": cluster.spec.min_workers,
                    "max": cluster.spec.max_workers,
                }),
            );
        }

        // Reconcile journal store config
        self.k8s.apply(
            "ConfigMap",
            &format!("{}-journal", name),
            json!({
                "backend": cluster.spec.journal_store.backend,
            }),
        );

        // Update status
        cluster.status.control_plane_ready = true;
        cluster.status.worker_ready = true;
        cluster.status.observed_generation = cluster.metadata.generation;
        cluster.status.conditions = vec![ready_condition(true, "cluster reconciled")];

        Ok(())
    }

    pub fn delete_cluster(&mut self, cluster: &mut AncoraCluster) -> Result<(), ReconcileError> {
        let name = &cluster.metadata.name;
        self.k8s
            .delete("Deployment", &format!("{}-control-plane", name));
        self.k8s.delete("Deployment", &format!("{}-worker", name));
        self.k8s
            .delete("HorizontalPodAutoscaler", &format!("{}-worker-hpa", name));
        self.k8s.delete("ConfigMap", &format!("{}-journal", name));
        cluster.metadata.finalizers.retain(|f| f != FINALIZER);
        info!(name = %name, "cluster deleted via finalizer");
        Ok(())
    }

    // --- AncoraTenant ---

    pub fn reconcile_tenant(&mut self, tenant: &mut AncoraTenant) -> Result<(), ReconcileError> {
        let name = tenant.metadata.name.clone();
        info!(name = %name, "reconciling AncoraTenant");

        if !tenant.metadata.finalizers.contains(&FINALIZER.to_string()) {
            tenant.metadata.finalizers.push(FINALIZER.to_string());
        }

        let ns = format!("ancora-{}", name);
        self.k8s
            .apply("Namespace", &ns, json!({ "kind": "Namespace" }));

        if let Some(ref role) = tenant.spec.admin_role_binding {
            self.k8s.apply(
                "RoleBinding",
                &format!("{}-admin", name),
                json!({ "role": role, "namespace": ns }),
            );
        }

        tenant.status.provisioned = true;
        tenant.status.namespace = Some(ns);
        tenant.status.conditions = vec![ready_condition(true, "tenant provisioned")];

        Ok(())
    }

    pub fn delete_tenant(&mut self, tenant: &mut AncoraTenant) -> Result<(), ReconcileError> {
        let name = &tenant.metadata.name;
        let ns = format!("ancora-{}", name);
        self.k8s.delete("Namespace", &ns);
        self.k8s.delete("RoleBinding", &format!("{}-admin", name));
        tenant.metadata.finalizers.retain(|f| f != FINALIZER);
        tenant.status.provisioned = false;
        Ok(())
    }

    pub fn k8s(&self) -> &FakeK8s {
        &self.k8s
    }

    pub fn rolling_update_cluster(&mut self, cluster: &mut AncoraCluster, new_image: String) {
        cluster.spec.image = new_image;
        let _ = self.reconcile_cluster(cluster);
    }
}

impl Default for Reconciler {
    fn default() -> Self {
        Self::new()
    }
}
