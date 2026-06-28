/// Announcement channel target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Channel {
    Blog,
    Discord,
    Twitter,
    Email,
    ReleaseNotes,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Channel::Blog => "blog",
            Channel::Discord => "discord",
            Channel::Twitter => "twitter",
            Channel::Email => "email",
            Channel::ReleaseNotes => "release-notes",
        };
        write!(f, "{}", s)
    }
}

/// Draft announcement for the obs and eval milestone.
#[derive(Debug, Clone)]
pub struct Announcement {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub channels: Vec<Channel>,
    pub published: bool,
}

impl Announcement {
    pub fn new(
        title: impl Into<String>,
        summary: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            summary: summary.into(),
            body: body.into(),
            channels: Vec::new(),
            published: false,
        }
    }

    pub fn add_channel(mut self, channel: Channel) -> Self {
        self.channels.push(channel);
        self
    }

    pub fn publish(&mut self) {
        self.published = true;
    }

    pub fn word_count(&self) -> usize {
        self.body.split_whitespace().count()
    }

    pub fn render(&self) -> String {
        format!(
            "# {}\n\n> {}\n\n{}\n\nChannels: {}\nPublished: {}",
            self.title,
            self.summary,
            self.body,
            self.channels
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.published
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn announcement_not_published_by_default() {
        let a = Announcement::new("Title", "Summary", "Body text here.");
        assert!(!a.published);
    }

    #[test]
    fn publish_flips_flag() {
        let mut a = Announcement::new("Title", "Summary", "Body.");
        a.publish();
        assert!(a.published);
    }

    #[test]
    fn word_count_correct() {
        let a = Announcement::new("T", "S", "one two three four five");
        assert_eq!(a.word_count(), 5);
    }

    #[test]
    fn channels_rendered() {
        let a = Announcement::new("T", "S", "B")
            .add_channel(Channel::Blog)
            .add_channel(Channel::Discord);
        let r = a.render();
        assert!(r.contains("blog"));
        assert!(r.contains("discord"));
    }
}
