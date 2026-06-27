/// A boxed function that mutates a JSON request body before it is sent.
pub type RequestTransformFn = Box<dyn Fn(&mut serde_json::Value) + Send + Sync>;

/// Return a `RequestTransformFn` that sets a fixed top-level field in the request body.
///
/// Useful for provider-specific fields such as `"stream_options"` or `"safe_mode"`.
pub fn set_field(key: &'static str, value: serde_json::Value) -> impl Fn(&mut serde_json::Value) + Send + Sync + 'static {
    move |body| {
        body[key] = value.clone();
    }
}

/// A boxed function that mutates a JSON response body before it is parsed.
pub type ResponseTransformFn = Box<dyn Fn(&mut serde_json::Value) + Send + Sync>;

/// Ordered chain of request transforms applied left-to-right before sending.
#[derive(Default)]
pub struct RequestTransformChain {
    transforms: Vec<RequestTransformFn>,
}

impl RequestTransformChain {
    pub fn push(&mut self, f: impl Fn(&mut serde_json::Value) + Send + Sync + 'static) {
        self.transforms.push(Box::new(f));
    }

    /// Apply all transforms in registration order.
    pub fn apply(&self, body: &mut serde_json::Value) {
        for f in &self.transforms {
            f(body);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }
}

/// Ordered chain of response transforms applied left-to-right after receiving.
#[derive(Default)]
pub struct ResponseTransformChain {
    transforms: Vec<ResponseTransformFn>,
}

impl ResponseTransformChain {
    pub fn push(&mut self, f: impl Fn(&mut serde_json::Value) + Send + Sync + 'static) {
        self.transforms.push(Box::new(f));
    }

    /// Apply all transforms in registration order.
    pub fn apply(&self, body: &mut serde_json::Value) {
        for f in &self.transforms {
            f(body);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_transforms_run_in_order() {
        let mut chain = RequestTransformChain::default();
        chain.push(|v| v["step1"] = json!(true));
        chain.push(|v| v["step2"] = json!(true));
        let mut body = json!({});
        chain.apply(&mut body);
        assert_eq!(body["step1"], json!(true));
        assert_eq!(body["step2"], json!(true));
    }

    #[test]
    fn response_transforms_run_in_order() {
        let mut chain = ResponseTransformChain::default();
        chain.push(|v| v["a"] = json!(1));
        chain.push(|v| v["b"] = json!(v["a"].as_i64().unwrap_or(0) + 1));
        let mut body = json!({});
        chain.apply(&mut body);
        assert_eq!(body["a"], json!(1));
        assert_eq!(body["b"], json!(2));
    }

    #[test]
    fn empty_chains_are_empty() {
        let req = RequestTransformChain::default();
        let res = ResponseTransformChain::default();
        assert!(req.is_empty());
        assert!(res.is_empty());
    }
}
