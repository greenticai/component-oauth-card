use std::fs;
use std::path::PathBuf;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn cargo_package_version() -> String {
    let cargo_toml =
        fs::read_to_string(project_root().join("Cargo.toml")).expect("read Cargo.toml");
    let mut in_package = false;

    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package = true;
            continue;
        }
        if in_package && trimmed.starts_with('[') {
            break;
        }
        if in_package && trimmed.starts_with("version = ") {
            return trimmed
                .trim_start_matches("version = ")
                .trim_matches('"')
                .to_string();
        }
    }

    panic!("package.version not found in Cargo.toml");
}

#[test]
fn manifest_mentions_component_v060_world() {
    let manifest_path = project_root().join("component.manifest.json");
    let text = fs::read_to_string(&manifest_path).expect("read component.manifest.json");
    let json: serde_json::Value = serde_json::from_str(&text).expect("manifest should be valid");
    assert_eq!(json["world"], "greentic:component/component@0.6.0");
}

#[test]
fn manifest_default_operation_is_declared() {
    let manifest_path = project_root().join("component.manifest.json");
    let text = fs::read_to_string(&manifest_path).expect("read component.manifest.json");
    let json: serde_json::Value = serde_json::from_str(&text).expect("manifest should be valid");
    let default_op = json["default_operation"]
        .as_str()
        .expect("default_operation should be string");
    let operations = json["operations"].as_array().expect("operations array");
    assert!(
        operations
            .iter()
            .any(|entry| entry["name"].as_str() == Some(default_op))
    );
}

#[test]
fn schema_file_exists() {
    assert!(
        project_root()
            .join("schemas/component.schema.json")
            .exists()
    );
}

#[test]
fn manifest_version_matches_cargo_version() {
    let manifest_path = project_root().join("component.manifest.json");
    let text = fs::read_to_string(&manifest_path).expect("read component.manifest.json");
    let json: serde_json::Value = serde_json::from_str(&text).expect("manifest should be valid");
    let manifest_version = json["version"]
        .as_str()
        .expect("manifest version should be string");

    assert_eq!(manifest_version, cargo_package_version());
}

#[test]
fn readme_mentions_required_oauth_provider_extension() {
    let text = fs::read_to_string(project_root().join("README.md")).expect("read README.md");
    assert!(text.contains("This component cannot work on its own."));
    assert!(text.contains("Greentic OAuth provider extension"));
    assert!(text.contains("../greentic-oauth"));
    assert!(text.contains("does not import the OAuth broker directly"));
}

#[test]
fn standalone_schema_matches_runtime_schema() {
    let text = fs::read_to_string(project_root().join("schemas/component.schema.json"))
        .expect("read schemas/component.schema.json");
    let file_json: serde_json::Value =
        serde_json::from_str(&text).expect("schema file should be valid json");

    assert_eq!(file_json, component_oauth_card::oauth_config_schema_json());
}
