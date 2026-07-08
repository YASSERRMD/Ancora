/// Determinism: map ordering is stable in serialization.
/// JSON object keys must be sorted deterministically to guarantee journal equality.
use serde_json::Value;
use std::collections::BTreeMap;

fn stable_serialize(map: &BTreeMap<&str, &str>) -> String {
    serde_json::to_string(map).unwrap()
}

#[test]
fn btreemap_keys_are_always_sorted() {
    let mut m = BTreeMap::new();
    m.insert("z", "last");
    m.insert("a", "first");
    m.insert("m", "middle");
    let s = stable_serialize(&m);
    let v: Value = serde_json::from_str(&s).unwrap();
    let keys: Vec<&str> = v.as_object().unwrap().keys().map(|k| k.as_str()).collect();
    assert_eq!(keys, vec!["a", "m", "z"]);
}

#[test]
fn same_btreemap_serialises_identically_on_two_calls() {
    let mut m = BTreeMap::new();
    m.insert("foo", "1");
    m.insert("bar", "2");
    m.insert("baz", "3");
    assert_eq!(stable_serialize(&m), stable_serialize(&m));
}

#[test]
fn json_round_trip_preserves_all_entries() {
    let mut m = BTreeMap::new();
    m.insert("x", "10");
    m.insert("y", "20");
    let s = stable_serialize(&m);
    let v: Value = serde_json::from_str(&s).unwrap();
    assert_eq!(v["x"], "10");
    assert_eq!(v["y"], "20");
}

#[test]
fn two_maps_with_same_entries_in_different_insertion_order_produce_same_json() {
    let mut m1 = BTreeMap::new();
    m1.insert("b", "2");
    m1.insert("a", "1");
    let mut m2 = BTreeMap::new();
    m2.insert("a", "1");
    m2.insert("b", "2");
    assert_eq!(stable_serialize(&m1), stable_serialize(&m2));
}

#[test]
fn empty_map_serialises_to_empty_object() {
    let m: BTreeMap<&str, &str> = BTreeMap::new();
    assert_eq!(stable_serialize(&m), "{}");
}
