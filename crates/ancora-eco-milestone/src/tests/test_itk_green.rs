use crate::itk_status::{all_itk_passing, itk_tests};

#[test]
fn test_all_itk_tests_pass() {
    let tests = itk_tests();
    assert!(!tests.is_empty(), "should have ITK tests");
    assert!(all_itk_passing(&tests), "all ITK tests should pass");
}

#[test]
fn test_itk_categories_nonempty() {
    let tests = itk_tests();
    for t in &tests {
        assert!(!t.category.is_empty(), "test {} should have a category", t.name);
    }
}
