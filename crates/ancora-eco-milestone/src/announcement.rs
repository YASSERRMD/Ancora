/// Announcement draft data for the ecosystem milestone.
#[derive(Debug, Clone)]
pub struct AnnouncementSection {
    pub heading: &'static str,
    pub body: &'static str,
}

impl AnnouncementSection {
    pub const fn new(heading: &'static str, body: &'static str) -> Self {
        Self { heading, body }
    }
}

#[derive(Debug, Clone)]
pub struct AnnouncementDraft {
    pub title: &'static str,
    pub tagline: &'static str,
    pub sections: Vec<AnnouncementSection>,
}

impl AnnouncementDraft {
    pub fn word_count(&self) -> usize {
        let mut count = self.title.split_whitespace().count();
        count += self.tagline.split_whitespace().count();
        for s in &self.sections {
            count += s.heading.split_whitespace().count();
            count += s.body.split_whitespace().count();
        }
        count
    }
}

pub fn announcement_draft() -> AnnouncementDraft {
    AnnouncementDraft {
        title: "Ancora v0.6.0: Ecosystem Milestone",
        tagline: "The Ancora ecosystem is now fully consolidated and production-ready.",
        sections: vec![
            AnnouncementSection::new(
                "What is new",
                "Plugin catalog, registry, sample applications, interop toolkit, and full test coverage are all green.",
            ),
            AnnouncementSection::new(
                "Breaking changes",
                "PluginCtx::invoke has been renamed to PluginCtx::call. Plugin manifests must use schema v3.",
            ),
            AnnouncementSection::new(
                "Getting started",
                "Run `cargo add ancora` and follow the extension author quickstart guide at ancora.dev/docs/quickstart.",
            ),
        ],
    }
}
