use ancora_preset::{assemble, research_assistant};

fn main() {
    let preset = research_assistant();
    println!("Preset: {}", preset.name);
    println!("Description: {}", preset.description);
    println!("Capabilities: {:?}", preset.capabilities);

    let spec = assemble(&preset).expect("research-assistant preset should always assemble");
    println!("Agent ID: {}", spec.agent_id);
    println!("Tools: {:?}", spec.tools);
    println!("System prompt (first 80 chars): {}", &spec.system_prompt[..80.min(spec.system_prompt.len())]);
}
