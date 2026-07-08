use crate::k8s_template::{K8sConfig, K8sError, K8sTemplate};

#[test]
fn test_k8s_basic_render() {
    let config = K8sConfig::new("ancora-agent", "ancora", "ancora/agent:latest");
    let tmpl = K8sTemplate::render(config).expect("should render");
    assert!(tmpl.deployment_yaml.contains("ancora-agent"));
    assert!(tmpl.deployment_yaml.contains("ancora/agent:latest"));
}

#[test]
fn test_k8s_security_context() {
    let config = K8sConfig::new("sec-agent", "default", "ancora/agent:latest");
    let tmpl = K8sTemplate::render(config).expect("should render");
    assert!(tmpl.deployment_yaml.contains("runAsNonRoot: true"));
    assert!(tmpl
        .deployment_yaml
        .contains("allowPrivilegeEscalation: false"));
    assert!(tmpl
        .deployment_yaml
        .contains("readOnlyRootFilesystem: true"));
}

#[test]
fn test_k8s_run_as_non_root_uid() {
    let config = K8sConfig::new("uid-agent", "ns", "img:v1");
    let tmpl = K8sTemplate::render(config).expect("should render");
    assert!(
        tmpl.deployment_yaml.contains("runAsUser: 65534"),
        "must run as nobody (65534)"
    );
}

#[test]
fn test_k8s_network_policy_generated() {
    let config = K8sConfig::new("netpol-agent", "ns", "img:v1");
    let tmpl = K8sTemplate::render(config).expect("should render");
    let np = tmpl
        .network_policy_yaml
        .expect("network policy should be present");
    assert!(np.contains("NetworkPolicy"));
    assert!(np.contains("netpol-agent-netpol"));
}

#[test]
fn test_k8s_operator_cr_generated() {
    let config = K8sConfig::new("op-agent", "ns", "img:v1");
    let tmpl = K8sTemplate::render(config).expect("should render");
    let cr = tmpl
        .operator_cr_yaml
        .expect("operator CR should be present");
    assert!(cr.contains("AncoraAgent"));
    assert!(cr.contains("secureDefaults: true"));
}

#[test]
fn test_k8s_all_manifests_combined() {
    let config = K8sConfig::new("combined", "ns", "img:v1");
    let tmpl = K8sTemplate::render(config).expect("should render");
    let all = tmpl.all_manifests();
    assert!(all.contains("Deployment"));
    assert!(all.contains("Service"));
    assert!(all.contains("NetworkPolicy"));
    assert!(all.contains("AncoraAgent"));
}

#[test]
fn test_k8s_custom_replicas() {
    let config = K8sConfig::new("ha-agent", "ns", "img:v1").with_replicas(5);
    let tmpl = K8sTemplate::render(config).expect("should render");
    assert!(tmpl.deployment_yaml.contains("replicas: 5"));
}

#[test]
fn test_k8s_resource_limits_present() {
    let config = K8sConfig::new("res-agent", "ns", "img:v1");
    let tmpl = K8sTemplate::render(config).expect("should render");
    assert!(tmpl.deployment_yaml.contains("limits:"));
    assert!(tmpl.deployment_yaml.contains("requests:"));
}

#[test]
fn test_k8s_empty_product_fails() {
    let config = K8sConfig::new("", "ns", "img:v1");
    let err = K8sTemplate::render(config).unwrap_err();
    assert!(matches!(err, K8sError::InvalidConfig(_)));
}

#[test]
fn test_k8s_empty_image_fails() {
    let config = K8sConfig::new("agent", "ns", "");
    let err = K8sTemplate::render(config).unwrap_err();
    assert!(matches!(err, K8sError::InvalidConfig(_)));
}
