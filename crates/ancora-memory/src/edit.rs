/// Fixed-size short-term core memory block for an agent.
pub struct CoreMemory {
    content: String,
}

impl CoreMemory {
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            content: initial.into(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    /// Append `text` to the end of core memory.
    pub fn append(&mut self, text: &str) {
        self.content.push('\n');
        self.content.push_str(text);
    }

    /// Replace the entire core memory with `text`.
    pub fn replace(&mut self, text: impl Into<String>) {
        self.content = text.into();
    }
}

/// Long-term archival store supporting insert and keyword search.
pub struct ArchivalMemory {
    entries: Vec<String>,
}

impl ArchivalMemory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, text: impl Into<String>) {
        self.entries.push(text.into());
    }

    pub fn search(&self, query: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.contains(query))
            .map(|e| e.as_str())
            .collect()
    }
}

impl Default for ArchivalMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Tool calls an agent may issue to edit its own memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryEditTool {
    CoreAppend { text: String },
    CoreReplace { text: String },
    ArchivalInsert { text: String },
    ArchivalSearch { query: String },
}

/// Combines core and archival memory, driven by tool calls from an agent.
pub struct SelfEditingMemory {
    pub core: CoreMemory,
    pub archival: ArchivalMemory,
}

impl SelfEditingMemory {
    pub fn new(initial_core: impl Into<String>) -> Self {
        Self {
            core: CoreMemory::new(initial_core),
            archival: ArchivalMemory::new(),
        }
    }

    /// Apply a memory edit tool and return any output (e.g. search results).
    pub fn apply_tool(&mut self, tool: MemoryEditTool) -> Option<Vec<String>> {
        match tool {
            MemoryEditTool::CoreAppend { text } => {
                self.core.append(&text);
                None
            }
            MemoryEditTool::CoreReplace { text } => {
                self.core.replace(text);
                None
            }
            MemoryEditTool::ArchivalInsert { text } => {
                self.archival.insert(text);
                None
            }
            MemoryEditTool::ArchivalSearch { query } => Some(
                self.archival
                    .search(&query)
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_appends_to_core_memory() {
        let mut mem = SelfEditingMemory::new("initial");
        mem.apply_tool(MemoryEditTool::CoreAppend {
            text: "new fact".to_string(),
        });
        assert!(mem.core.content().contains("initial"));
        assert!(mem.core.content().contains("new fact"));
    }

    #[test]
    fn agent_replaces_core_memory() {
        let mut mem = SelfEditingMemory::new("old");
        mem.apply_tool(MemoryEditTool::CoreReplace {
            text: "fresh start".to_string(),
        });
        assert_eq!(mem.core.content(), "fresh start");
    }

    #[test]
    fn agent_inserts_and_searches_archival_memory() {
        let mut mem = SelfEditingMemory::new("");
        mem.apply_tool(MemoryEditTool::ArchivalInsert {
            text: "Rust is fast".to_string(),
        });
        mem.apply_tool(MemoryEditTool::ArchivalInsert {
            text: "Python is dynamic".to_string(),
        });
        let results = mem
            .apply_tool(MemoryEditTool::ArchivalSearch {
                query: "Rust".to_string(),
            })
            .unwrap();
        assert_eq!(results, vec!["Rust is fast"]);
    }
}
