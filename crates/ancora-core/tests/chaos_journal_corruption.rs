// Chaos: journal corruption during write -- detect and quarantine.

#[derive(Debug, Clone, PartialEq)]
enum Integrity {
    Valid,
    Corrupted(String),
}

struct JournalEntry {
    seq: u64,
    checksum: u32,
    payload: Vec<u8>,
}

impl JournalEntry {
    fn compute_checksum(payload: &[u8]) -> u32 {
        payload
            .iter()
            .fold(0u32, |acc, &b| acc.wrapping_add(b as u32))
    }

    fn new(seq: u64, payload: Vec<u8>) -> Self {
        let checksum = Self::compute_checksum(&payload);
        Self {
            seq,
            checksum,
            payload,
        }
    }

    fn verify(&self) -> Integrity {
        let expected = Self::compute_checksum(&self.payload);
        if self.checksum == expected {
            Integrity::Valid
        } else {
            Integrity::Corrupted(format!("seq={} checksum mismatch", self.seq))
        }
    }
}

fn verify_journal(entries: &[JournalEntry]) -> Vec<Integrity> {
    entries.iter().map(|e| e.verify()).collect()
}

#[test]
fn test_valid_entry_passes_checksum() {
    let e = JournalEntry::new(0, b"hello".to_vec());
    assert_eq!(e.verify(), Integrity::Valid);
}

#[test]
fn test_corrupted_entry_fails_checksum() {
    let mut e = JournalEntry::new(1, b"data".to_vec());
    e.checksum = e.checksum.wrapping_add(1);
    assert!(matches!(e.verify(), Integrity::Corrupted(_)));
}

#[test]
fn test_all_valid_journal_passes() {
    let entries = vec![
        JournalEntry::new(0, b"a".to_vec()),
        JournalEntry::new(1, b"b".to_vec()),
        JournalEntry::new(2, b"c".to_vec()),
    ];
    let results = verify_journal(&entries);
    assert!(results.iter().all(|r| r == &Integrity::Valid));
}

#[test]
fn test_one_corrupt_entry_detected() {
    let mut entries = vec![
        JournalEntry::new(0, b"ok".to_vec()),
        JournalEntry::new(1, b"bad".to_vec()),
    ];
    entries[1].checksum = 0;
    let results = verify_journal(&entries);
    assert_eq!(results[0], Integrity::Valid);
    assert!(matches!(results[1], Integrity::Corrupted(_)));
}

#[test]
fn test_empty_payload_valid() {
    let e = JournalEntry::new(0, vec![]);
    assert_eq!(e.verify(), Integrity::Valid);
}

#[test]
fn test_corruption_message_includes_seq() {
    let mut e = JournalEntry::new(42, b"x".to_vec());
    e.checksum = 0;
    if let Integrity::Corrupted(msg) = e.verify() {
        assert!(msg.contains("seq=42"));
    } else {
        panic!("expected corruption");
    }
}
