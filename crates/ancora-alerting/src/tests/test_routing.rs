#[cfg(test)]
mod tests {
    use crate::{routing::WebhookRouter, rules::check_high_error_rate};

    #[test]
    fn webhook_router_receives_alert() {
        let mut router = WebhookRouter::new("http://alerts.example.com/webhook");
        let alert = check_high_error_rate(0.10, 0.05, 1000).unwrap();
        router.route(alert);
        assert_eq!(router.sent_count(), 1);
    }

    #[test]
    fn webhook_router_accumulates_multiple() {
        let mut router = WebhookRouter::new("http://alerts.example.com/webhook");
        router.route(check_high_error_rate(0.10, 0.05, 1000).unwrap());
        router.route(check_high_error_rate(0.20, 0.05, 2000).unwrap());
        assert_eq!(router.sent_count(), 2);
    }
}
