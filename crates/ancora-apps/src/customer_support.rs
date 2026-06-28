/// Customer support application.
///
/// Routes support tickets to canned responses from a local knowledge base
/// and tracks ticket lifecycle without any external calls.

#[derive(Debug, Clone, PartialEq)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: u64,
    pub subject: String,
    pub body: String,
    pub status: TicketStatus,
}

impl Ticket {
    pub fn new(id: u64, subject: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id,
            subject: subject.into(),
            body: body.into(),
            status: TicketStatus::Open,
        }
    }

    pub fn resolve(&mut self) {
        self.status = TicketStatus::Resolved;
    }

    pub fn close(&mut self) {
        self.status = TicketStatus::Closed;
    }
}

#[derive(Debug, Clone)]
pub struct ResponseTemplate {
    pub keyword: String,
    pub response: String,
}

impl ResponseTemplate {
    pub fn new(keyword: impl Into<String>, response: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            response: response.into(),
        }
    }
}

pub struct SupportEngine {
    templates: Vec<ResponseTemplate>,
    tickets: Vec<Ticket>,
    next_id: u64,
}

impl SupportEngine {
    pub fn new(templates: Vec<ResponseTemplate>) -> Self {
        Self {
            templates,
            tickets: Vec::new(),
            next_id: 1,
        }
    }

    pub fn submit(&mut self, subject: impl Into<String>, body: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tickets.push(Ticket::new(id, subject, body));
        id
    }

    pub fn auto_respond(&self, ticket_id: u64) -> Option<String> {
        let ticket = self.tickets.iter().find(|t| t.id == ticket_id)?;
        let combined = format!("{} {}", ticket.subject, ticket.body).to_lowercase();
        for tmpl in &self.templates {
            if combined.contains(&tmpl.keyword.to_lowercase()) {
                return Some(tmpl.response.clone());
            }
        }
        Some("Thank you for contacting support. An agent will be in touch shortly.".to_string())
    }

    pub fn resolve_ticket(&mut self, ticket_id: u64) -> bool {
        if let Some(t) = self.tickets.iter_mut().find(|t| t.id == ticket_id) {
            t.resolve();
            true
        } else {
            false
        }
    }

    pub fn open_count(&self) -> usize {
        self.tickets
            .iter()
            .filter(|t| t.status == TicketStatus::Open)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_respond_matches_keyword() {
        let templates = vec![ResponseTemplate::new(
            "password",
            "Please use the reset link on the login page.",
        )];
        let mut engine = SupportEngine::new(templates);
        let id = engine.submit("Forgot password", "I cannot log in");
        let resp = engine.auto_respond(id).unwrap();
        assert!(resp.contains("reset link"));
    }
}
