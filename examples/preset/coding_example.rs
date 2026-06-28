use ancora_preset::{assemble, coding_agent};

fn main() {
    let preset = coding_agent();
    println!("Preset: {}", preset.name);
    println!("Capabilities: {:?}", preset.capabilities);

    let spec = assemble(&preset).expect("coding-agent preset should always assemble");
    println!("Agent ID: {}", spec.agent_id);
    println!("Tools: {:?}", spec.tools);
}
