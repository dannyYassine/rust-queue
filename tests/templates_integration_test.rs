mod common;

#[cfg(feature = "tera_templates")]
#[tokio::test]
async fn test_feature_enabled() {
    // Use the feature flag in your tests
    // Perform your assertions or other test logic
    assert_eq!(true, true);
}
