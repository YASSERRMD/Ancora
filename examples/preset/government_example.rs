use ancora_preset::{assemble, government_compliant, AirGapPolicy, ResidencyConstraint};

fn main() {
    let zone = "us-gov-east-1";
    let preset = government_compliant(zone);

    println!("Preset: {}", preset.name);
    println!("Air-gap: {:?}", preset.air_gap);
    println!("Residency: {:?}", preset.residency);
    println!("Locked: {}", preset.locked);

    assert_eq!(preset.air_gap, AirGapPolicy::Required);
    assert_eq!(preset.residency, ResidencyConstraint::Zone(zone.to_string()));

    let spec = assemble(&preset).expect("government preset should assemble");
    println!("Agent ID: {}", spec.agent_id);
    println!("Tools: {:?}", spec.tools);

    assert!(spec.system_prompt.contains("air_gap:required"));
    assert!(spec.system_prompt.contains("locked:true"));
    println!("All government compliance checks passed.");
}
