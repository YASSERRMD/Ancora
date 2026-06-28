#[cfg(test)]
mod tests {
    use crate::backup_ops::BackupOps;

    #[test]
    fn backup_command_produces_backup() {
        let mut ops = BackupOps::default();
        let bkp = ops.create_backup("tenant-a", 1000);
        assert_eq!(bkp.tenant_id, "tenant-a");
        assert_eq!(bkp.kind, "snapshot");
        assert!(!bkp.checksum.is_empty());
    }

    #[test]
    fn backup_list_grows() {
        let mut ops = BackupOps::default();
        ops.create_backup("t1", 1000);
        ops.create_backup("t1", 2000);
        assert_eq!(ops.list().len(), 2);
    }
}
