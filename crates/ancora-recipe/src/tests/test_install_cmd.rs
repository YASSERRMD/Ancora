use crate::install::{InstallCommand, InstallRegistry, InstallStatus, InstallTarget};
use crate::params::ParamSet;
use crate::rag_citations;

fn make_rag_recipe() -> crate::format::Recipe {
    let ps = ParamSet::default();
    rag_citations::build(&ps)
}

#[test]
fn install_command_parse_valid() {
    let cmd = InstallCommand::parse("rag-citations:/workspace/myproject").unwrap();
    assert_eq!(cmd.recipe_id, "rag-citations");
    assert_eq!(cmd.target_dir, "/workspace/myproject");
}

#[test]
fn install_command_parse_invalid_no_colon() {
    assert!(InstallCommand::parse("no-colon-here").is_err());
}

#[test]
fn install_command_parse_empty_recipe_id() {
    assert!(InstallCommand::parse(":/some/dir").is_err());
}

#[test]
fn registry_install_and_query() {
    let mut reg = InstallRegistry::new();
    let recipe = make_rag_recipe();
    let entry = reg.install(&recipe, InstallTarget::Directory("/proj/a".into()));
    assert_eq!(entry.status, InstallStatus::Installed);
    assert!(reg.is_installed("rag-citations", "/proj/a"));
    assert!(!reg.is_installed("rag-citations", "/proj/b"));
}

#[test]
fn registry_double_install_is_already_present() {
    let mut reg = InstallRegistry::new();
    let recipe = make_rag_recipe();
    reg.install(&recipe, InstallTarget::Directory("/proj".into()));
    let second = reg.install(&recipe, InstallTarget::Directory("/proj".into()));
    assert_eq!(second.status, InstallStatus::AlreadyPresent);
}

#[test]
fn registry_installed_ids_nonempty() {
    let mut reg = InstallRegistry::new();
    let recipe = make_rag_recipe();
    reg.install(&recipe, InstallTarget::Directory("/p".into()));
    assert!(!reg.installed_ids().is_empty());
}
