#[cfg(test)]
mod tests {
    use crate::component::{Component, ComponentKind, License};
    use crate::policy::SupplyChainPolicy;
    use crate::provenance::ProvenanceStore;
    use crate::report::SupplyChainReport;
    use crate::sbom::{Sbom, SbomFormat};
    use crate::signature::{ComponentSignature, SignatureAlgorithm, SignatureStore};

    fn make_component(id: &str) -> Component {
        Component::new(
            id,
            "lib",
            "1.0.0",
            ComponentKind::Library,
            License::Mit,
            "vendor",
            "sha256:00",
        )
    }

    fn make_sig(component_id: &str) -> ComponentSignature {
        ComponentSignature::new(
            component_id,
            SignatureAlgorithm::Ed25519,
            "signer",
            "sig",
            1000,
        )
    }

    #[test]
    fn test_sign_rate_all_signed() {
        let mut sbom = Sbom::new("sbom-rate", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1"));
        sbom.add_component(make_component("c2"));

        let mut sigs = SignatureStore::new();
        sigs.register(make_sig("c1"));
        sigs.register(make_sig("c2"));

        let prov = ProvenanceStore::new();
        let policy = SupplyChainPolicy::new("t1");

        let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1001);
        assert!((report.sign_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sign_rate_none_signed() {
        let mut sbom = Sbom::new("sbom-rate2", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1"));
        sbom.add_component(make_component("c2"));

        let sigs = SignatureStore::new();
        let prov = ProvenanceStore::new();
        let policy = SupplyChainPolicy::new("t1");

        let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1001);
        assert!((report.sign_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sign_rate_half_signed() {
        let mut sbom = Sbom::new("sbom-rate3", "t1", SbomFormat::CycloneDx, 1000);
        sbom.add_component(make_component("c1"));
        sbom.add_component(make_component("c2"));

        let mut sigs = SignatureStore::new();
        sigs.register(make_sig("c1"));

        let prov = ProvenanceStore::new();
        let policy = SupplyChainPolicy::new("t1");

        let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1001);
        assert!((report.sign_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_sign_rate_empty_sbom() {
        let sbom = Sbom::new("sbom-empty", "t1", SbomFormat::CycloneDx, 1000);
        let sigs = SignatureStore::new();
        let prov = ProvenanceStore::new();
        let policy = SupplyChainPolicy::new("t1");

        let report = SupplyChainReport::generate(&sbom, &sigs, &prov, &policy, 1001);
        assert!((report.sign_rate() - 0.0).abs() < f64::EPSILON);
    }
}
