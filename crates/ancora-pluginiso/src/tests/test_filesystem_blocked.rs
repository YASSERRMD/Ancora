use crate::filesystem_policy::{AccessMode, FilesystemPolicy, PathRule};

#[test]
fn plugin_filesystem_blocked_by_default_deny() {
    let policy = FilesystemPolicy::deny_all();
    assert!(!policy.permits("/etc/passwd", AccessMode::Read));
    assert!(!policy.permits("/home/user/.ssh/id_rsa", AccessMode::Read));
    assert!(!policy.permits("/tmp/anything", AccessMode::Write));
    assert!(!policy.permits("/var/log/syslog", AccessMode::Read));
}

#[test]
fn read_only_path_blocks_write() {
    let mut policy = FilesystemPolicy::deny_all();
    policy.add_rule(PathRule::read_only("/var/data/plugin-cache"));

    assert!(policy.permits("/var/data/plugin-cache/records.db", AccessMode::Read));
    assert!(!policy.permits("/var/data/plugin-cache/records.db", AccessMode::Write));
}

#[test]
fn scratch_dir_allows_read_write() {
    let mut policy = FilesystemPolicy::deny_all();
    policy.add_rule(PathRule::read_write("/tmp/plugin-scratch"));

    assert!(policy.permits("/tmp/plugin-scratch/output.json", AccessMode::Read));
    assert!(policy.permits("/tmp/plugin-scratch/output.json", AccessMode::Write));
    // Outside scratch dir still blocked.
    assert!(!policy.permits("/tmp/other/file.txt", AccessMode::Read));
}

#[test]
fn parent_dir_access_blocked_when_only_subdir_allowed() {
    let mut policy = FilesystemPolicy::deny_all();
    policy.add_rule(PathRule::read_only("/data/plugin/cache"));

    // Exact prefix match required.
    assert!(!policy.permits("/data/plugin/secrets", AccessMode::Read));
    assert!(!policy.permits("/data/plugin", AccessMode::Read));
}
