use crate::Role;
#[test]
fn admin_highest_precedence() { assert!(Role::Admin.precedence() > Role::Operator.precedence()); }
#[test]
fn operator_above_developer() { assert!(Role::Operator.precedence() > Role::Developer.precedence()); }
#[test]
fn developer_above_viewer() { assert!(Role::Developer.precedence() > Role::Viewer.precedence()); }
#[test]
fn viewer_lowest() { assert_eq!(Role::Viewer.precedence(), 0); }
