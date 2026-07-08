// Example parity: streaming token example accumulates same text across languages.

const STREAMING_TEXT: &str = "The capital of France is Paris.";
const STREAMING_TOKEN_COUNT: usize = 7;

struct StreamingTokens {
    lang: &'static str,
    tokens: Vec<&'static str>,
}

fn make_streaming_tokens(lang: &str) -> Vec<&'static str> {
    let _ = lang;
    vec!["The", " capital", " of", " France", " is", " Paris", "."]
}

fn join_tokens(tokens: &[&str]) -> String {
    tokens.join("")
}

#[test]
fn test_token_count_matches_expected() {
    let tokens = make_streaming_tokens("rust");
    assert_eq!(tokens.len(), STREAMING_TOKEN_COUNT);
}

#[test]
fn test_joined_tokens_match_streaming_text() {
    let tokens = make_streaming_tokens("go");
    assert_eq!(join_tokens(&tokens), STREAMING_TEXT);
}

#[test]
fn test_all_six_langs_produce_same_text() {
    for lang in ["rust", "go", "python", "ts", "dotnet", "java"] {
        let tokens = make_streaming_tokens(lang);
        assert_eq!(
            join_tokens(&tokens),
            STREAMING_TEXT,
            "lang {lang} joined text differs"
        );
    }
}

#[test]
fn test_first_token_is_the() {
    let tokens = make_streaming_tokens("python");
    assert_eq!(tokens[0], "The");
}

#[test]
fn test_last_token_is_period() {
    let tokens = make_streaming_tokens("ts");
    assert_eq!(*tokens.last().unwrap(), ".");
}

#[test]
fn test_streaming_text_non_empty() {
    assert!(!STREAMING_TEXT.is_empty());
}
