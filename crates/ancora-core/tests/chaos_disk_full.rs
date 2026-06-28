// Chaos: disk-full simulation -- journal write fails gracefully.

struct DiskQuota {
    used_bytes: u64,
    quota_bytes: u64,
}

impl DiskQuota {
    fn new(quota_bytes: u64) -> Self { Self { used_bytes: 0, quota_bytes } }
    fn write(&mut self, bytes: u64) -> Result<(), String> {
        if self.used_bytes + bytes > self.quota_bytes {
            Err(format!("disk full: used={} quota={}", self.used_bytes, self.quota_bytes))
        } else {
            self.used_bytes += bytes;
            Ok(())
        }
    }
    fn free_space(&self) -> u64 { self.quota_bytes.saturating_sub(self.used_bytes) }
}

fn write_journal_entries(disk: &mut DiskQuota, entries: &[u64]) -> (usize, bool) {
    let mut written = 0;
    for &sz in entries {
        if disk.write(sz).is_err() {
            return (written, false);
        }
        written += 1;
    }
    (written, true)
}

#[test]
fn test_all_entries_written_within_quota() {
    let mut disk = DiskQuota::new(1000);
    let (n, ok) = write_journal_entries(&mut disk, &[100, 200, 300]);
    assert!(ok);
    assert_eq!(n, 3);
}

#[test]
fn test_write_fails_when_quota_exceeded() {
    let mut disk = DiskQuota::new(200);
    let (n, ok) = write_journal_entries(&mut disk, &[100, 100, 100]);
    assert!(!ok);
    assert_eq!(n, 2);
}

#[test]
fn test_free_space_decreases_on_write() {
    let mut disk = DiskQuota::new(500);
    disk.write(200).unwrap();
    assert_eq!(disk.free_space(), 300);
}

#[test]
fn test_write_exact_quota_succeeds() {
    let mut disk = DiskQuota::new(100);
    assert!(disk.write(100).is_ok());
}

#[test]
fn test_write_past_quota_returns_error() {
    let mut disk = DiskQuota::new(50);
    let r = disk.write(51);
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("disk full"));
}

#[test]
fn test_partial_write_does_not_increase_used() {
    let mut disk = DiskQuota::new(100);
    disk.write(90).unwrap();
    let _ = disk.write(20); // fails
    assert_eq!(disk.used_bytes, 90);
}
