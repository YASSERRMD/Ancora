use crate::capability::{
    Capability, CapabilityBuilder, CapabilityFlags, QuantLimitations,
};

#[test]
fn capability_flags_honored() {
    let flags = CapabilityFlags::new()
        .with_capability(Capability::Chat)
        .with_capability(Capability::CodeGeneration);

    assert!(flags.supports(&Capability::Chat));
    assert!(flags.supports(&Capability::CodeGeneration));
    assert!(!flags.supports(&Capability::Embedding));
}

#[test]
fn capability_flags_remove_works() {
    let mut flags = CapabilityFlags::new()
        .with_capability(Capability::Chat)
        .with_capability(Capability::ToolCalling);

    flags.remove_capability(&Capability::ToolCalling);
    assert!(!flags.supports(&Capability::ToolCalling));
    assert!(flags.supports(&Capability::Chat));
}

#[test]
fn effective_context_length_respects_limits() {
    let flags = CapabilityFlags::new().with_limitations(QuantLimitations {
        max_context_length: Some(2048),
        ..Default::default()
    });

    // Model native is 4096, but limit is 2048.
    assert_eq!(flags.effective_context_length(4096), 2048);
    // Model native is 1024, below limit.
    assert_eq!(flags.effective_context_length(1024), 1024);
}

#[test]
fn builder_chat_model() {
    let flags = CapabilityBuilder::chat_model(true, 0);
    assert!(flags.supports(&Capability::Chat));
    assert!(flags.supports(&Capability::TextGeneration));
    assert!(flags.limitations.cpu_viable);
}

#[test]
fn builder_code_model() {
    let flags = CapabilityBuilder::code_model(false);
    assert!(flags.supports(&Capability::CodeGeneration));
    assert!(!flags.limitations.cpu_viable);
}

#[test]
fn capability_names_sorted() {
    let flags = CapabilityFlags::new()
        .with_capability(Capability::Vision)
        .with_capability(Capability::Chat)
        .with_capability(Capability::Embedding);

    let names = flags.capability_names();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);
}
