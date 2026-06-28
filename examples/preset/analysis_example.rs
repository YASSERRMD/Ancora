use ancora_preset::{assemble, data_analysis};

fn main() {
    let preset = data_analysis();
    println!("Preset: {}", preset.name);
    println!("Capabilities: {:?}", preset.capabilities);

    let spec = assemble(&preset).expect("data-analysis preset should always assemble");
    println!("Agent ID: {}", spec.agent_id);
    println!("Tools: {:?}", spec.tools);
}
