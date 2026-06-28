use ancora_preset::{assemble, customer_support};

fn main() {
    let preset = customer_support();
    println!("Preset: {}", preset.name);
    println!("Capabilities: {:?}", preset.capabilities);

    let spec = assemble(&preset).expect("customer-support preset should always assemble");
    println!("Agent ID: {}", spec.agent_id);
    println!("Tools: {:?}", spec.tools);
}
