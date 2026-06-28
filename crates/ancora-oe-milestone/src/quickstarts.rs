/// Supported SDK language for quickstart guides.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    Go,
    TypeScript,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::Go => "Go",
            Language::TypeScript => "TypeScript",
        };
        write!(f, "{}", s)
    }
}

/// A single step in a quickstart guide.
#[derive(Debug, Clone)]
pub struct QuickstartStep {
    pub title: String,
    pub command: Option<String>,
    pub description: String,
}

impl QuickstartStep {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            command: None,
            description: description.into(),
        }
    }

    pub fn with_command(mut self, cmd: impl Into<String>) -> Self {
        self.command = Some(cmd.into());
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!("### {}\n{}\n", self.title, self.description);
        if let Some(cmd) = &self.command {
            out.push_str(&format!("```\n{}\n```\n", cmd));
        }
        out
    }
}

/// A complete quickstart guide for a language.
#[derive(Debug, Clone)]
pub struct Quickstart {
    pub language: Language,
    pub title: String,
    pub steps: Vec<QuickstartStep>,
}

impl Quickstart {
    pub fn new(language: Language, title: impl Into<String>) -> Self {
        Self {
            language,
            title: title.into(),
            steps: Vec::new(),
        }
    }

    pub fn add_step(mut self, step: QuickstartStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!("# {} Quickstart: {}\n\n", self.language, self.title);
        for step in &self.steps {
            out.push_str(&step.render());
            out.push('\n');
        }
        out
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// Return only quickstarts for the requested language.
pub fn for_language<'a>(guides: &'a [Quickstart], lang: &Language) -> Vec<&'a Quickstart> {
    guides.iter().filter(|g| &g.language == lang).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quickstart_renders_steps() {
        let qs = Quickstart::new(Language::Rust, "Observability setup")
            .add_step(
                QuickstartStep::new("Install", "Add the crate to Cargo.toml.")
                    .with_command("cargo add ancora-observability"),
            )
            .add_step(QuickstartStep::new("Initialize", "Call init() at startup."));
        let r = qs.render();
        assert!(r.contains("Rust Quickstart"));
        assert!(r.contains("cargo add ancora-observability"));
        assert_eq!(qs.step_count(), 2);
    }

    #[test]
    fn filter_by_language() {
        let guides = vec![
            Quickstart::new(Language::Go, "Go OBS"),
            Quickstart::new(Language::Rust, "Rust OBS"),
        ];
        let go = for_language(&guides, &Language::Go);
        assert_eq!(go.len(), 1);
        assert_eq!(go[0].language, Language::Go);
    }
}
