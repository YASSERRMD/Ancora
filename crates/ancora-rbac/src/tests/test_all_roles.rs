use crate::Role;
#[test]
fn all_returns_four_roles() { assert_eq!(Role::all().len(), 4); }
#[test]
fn all_roles_have_unique_precedences() {
    let mut precs: Vec<u8> = Role::all().iter().map(|r| r.precedence()).collect();
    precs.sort();
    precs.dedup();
    assert_eq!(precs.len(), 4);
}
