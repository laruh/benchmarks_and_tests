use crate::swap_versioning::{legacy_swap_version, SwapVersion, TakerRequest};
use serde_json::Value;

#[test]
fn test_deserialization_with_missing_swap_version() {
    let json_data = r#"
        {
            "base": "BTC",
            "rel": "ETH"
        }"#;

    let request: TakerRequest = serde_json::from_str(json_data).unwrap();
    assert_eq!(request.base, "BTC");
    assert_eq!(request.rel, "ETH");
    assert_eq!(request.swap_version.version, legacy_swap_version());
}

#[test]
fn test_deserialization_with_non_legacy_swap_version() {
    let json_data = r#"
        {
            "base": "BTC",
            "rel": "ETH",
            "swap_version": { "version": 2 }
        }"#;

    let request: TakerRequest = serde_json::from_str(json_data).unwrap();
    assert_eq!(request.base, "BTC");
    assert_eq!(request.rel, "ETH");
    assert_eq!(request.swap_version.version, 2);
}

#[test]
fn test_serialization_skips_legacy_swap_version() {
    let request = TakerRequest {
        base: "BTC".to_string(),
        rel: "ETH".to_string(),
        swap_version: SwapVersion {
            version: legacy_swap_version(),
        },
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let json_value: Value = serde_json::from_str(&serialized).unwrap();

    assert_eq!(json_value["base"], "BTC");
    assert_eq!(json_value["rel"], "ETH");
    assert!(json_value.get("swap_version").is_none());
}

#[test]
fn test_serialization_includes_non_legacy_swap_version() {
    let request = TakerRequest {
        base: "BTC".to_string(),
        rel: "ETH".to_string(),
        swap_version: SwapVersion { version: 2 },
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let json_value: Value = serde_json::from_str(&serialized).unwrap();

    assert_eq!(json_value["base"], "BTC");
    assert_eq!(json_value["rel"], "ETH");
    assert_eq!(json_value["swap_version"]["version"], 2);
}
