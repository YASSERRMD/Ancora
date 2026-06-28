use crate::descriptor::PresetDescriptor;

/// Apply caller-supplied overrides to a preset, returning a modified copy.
///
/// Existing override pairs with the same key are replaced; new pairs are
/// appended.  The preset's `name`, `capabilities`, `air_gap`, and `residency`
/// fields are not affected -- only the `overrides` list is modified.
pub fn apply_overrides(
    mut preset: PresetDescriptor,
    extra: Vec<(String, String)>,
) -> PresetDescriptor {
    for (new_key, new_val) in extra {
        let existing = preset
            .overrides
            .iter_mut()
            .find(|(k, _)| k == &new_key);
        match existing {
            Some(pair) => pair.1 = new_val,
            None => preset.overrides.push((new_key, new_val)),
        }
    }
    preset
}

/// Read a single override value from the preset by key.
pub fn get_override<'a>(preset: &'a PresetDescriptor, key: &str) -> Option<&'a str> {
    preset
        .overrides
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
}
