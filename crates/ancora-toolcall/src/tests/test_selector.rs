use crate::schema::ToolDef;
use crate::selector::ToolSelector;

fn tools() -> Vec<ToolDef> {
    vec![
        ToolDef::new("web_search", "search the web"),
        ToolDef::new("code_exec", "execute code"),
        ToolDef::new("file_read", "read a file"),
    ]
}

#[test]
fn select_by_name_substring() {
    let ts = tools();
    let refs: Vec<&ToolDef> = ts.iter().collect();
    let sel = ToolSelector::new(5);
    let results = sel.select(&refs, "search");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "web_search");
}

#[test]
fn select_by_description_keyword() {
    let ts = tools();
    let refs: Vec<&ToolDef> = ts.iter().collect();
    let sel = ToolSelector::new(5);
    let results = sel.select(&refs, "execute");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "code_exec");
}

#[test]
fn max_results_limits_output() {
    let ts = tools();
    let refs: Vec<&ToolDef> = ts.iter().collect();
    let sel = ToolSelector::new(1);
    let results = sel.select(&refs, "e"); // matches web_search + code_exec + file_read
    assert_eq!(results.len(), 1);
}

#[test]
fn select_exact_returns_matching_tool() {
    let ts = tools();
    let refs: Vec<&ToolDef> = ts.iter().collect();
    let sel = ToolSelector::new(5);
    let result = sel.select_exact(&refs, "code_exec");
    assert!(result.is_some());
}
