use ancora_auth::{ServiceAccount, ServiceAccountRegistry};

fn main() {
    let mut reg = ServiceAccountRegistry::new();
    reg.register(
        ServiceAccount::new("ci-pipeline", "acme-corp", "sha256:abc123", "CI/CD pipeline service account")
            .with_scope("read:agents")
            .with_scope("write:tasks")
            .with_scope("read:logs"),
    );

    match reg.authenticate("ci-pipeline", "sha256:abc123", 3600, 1000) {
        Ok(token) => {
            println!("Service account auth ok");
            println!("  subject: {}", token.subject);
            println!("  tenant: {}", token.tenant_id);
            println!("  scopes: {:?}", token.scopes);
            println!("  expires at tick: {}", token.expires_at_tick);
        }
        Err(e) => eprintln!("Auth error: {:?}", e),
    }
}
