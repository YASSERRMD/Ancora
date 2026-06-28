use crate::{KeyAlgorithm, KeyBuilder, KeyPurpose};
#[test]
fn builder_sets_all_fields() {
    let k = KeyBuilder::new("k1", "t1")
        .algorithm(KeyAlgorithm::EcdsaP256)
        .purpose(KeyPurpose::Signing)
        .tick(100)
        .expires_at(1000)
        .material("secret-material")
        .build();
    assert_eq!(k.algorithm, KeyAlgorithm::EcdsaP256);
    assert_eq!(k.purpose, KeyPurpose::Signing);
    assert_eq!(k.created_tick, 100);
    assert_eq!(k.expires_tick, Some(1000));
    assert_eq!(k.key_material, "secret-material");
}
