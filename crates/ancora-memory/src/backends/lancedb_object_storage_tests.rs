//! Object-storage path tests for the LanceDB backend.
//! All offline -- verifies path classification and config wiring.

#![cfg(test)]

use crate::backends::lancedb::*;

#[test]
fn s3_path_uri_returns_full_path() {
    let p = LanceDbPath::s3("s3://my-bucket/data/lancedb");
    assert_eq!(p.uri(), "s3://my-bucket/data/lancedb");
}

#[test]
fn gcs_path_uri_returns_full_path() {
    let p = LanceDbPath::gcs("gs://my-bucket/embeddings");
    assert_eq!(p.uri(), "gs://my-bucket/embeddings");
}

#[test]
fn azure_path_uri_returns_full_path() {
    let p = LanceDbPath::azure("az://mycontainer/vectors");
    assert_eq!(p.uri(), "az://mycontainer/vectors");
}

#[test]
fn s3_config_stores_region() {
    let cfg = LanceDbConfig::s3("s3://b/p", "eu-central-1");
    assert_eq!(cfg.aws_region.as_deref(), Some("eu-central-1"));
}

#[test]
fn s3_config_path_is_remote() {
    let cfg = LanceDbConfig::s3("s3://b/p", "us-east-1");
    assert!(cfg.path.is_remote());
}

#[test]
fn local_config_has_no_region() {
    let cfg = LanceDbConfig::local("/data");
    assert!(cfg.aws_region.is_none());
}

#[test]
fn detect_storage_type_handles_all_schemes() {
    assert_eq!(detect_storage_type("s3://b/k"), "s3");
    assert_eq!(detect_storage_type("gs://b/k"), "gcs");
    assert_eq!(detect_storage_type("az://c/b"), "azure");
    assert_eq!(detect_storage_type("/mnt/data"), "local");
}

#[test]
fn detect_storage_type_relative_path_is_local() {
    assert_eq!(detect_storage_type("./data"), "local");
}
