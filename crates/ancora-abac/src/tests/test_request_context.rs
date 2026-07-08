use crate::RequestContext;
#[test]
fn context_builder_sets_all_attrs() {
    let ctx = RequestContext::new()
        .with_subject_attr("role", "admin")
        .with_resource_attr("classification", "secret")
        .with_env_attr("time_of_day", "business_hours");
    assert_eq!(ctx.subject.get_str("role"), Some("admin"));
    assert_eq!(ctx.resource.get_str("classification"), Some("secret"));
    assert_eq!(
        ctx.environment.get_str("time_of_day"),
        Some("business_hours")
    );
}
