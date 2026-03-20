#[path = "src/i18n_bundle.rs"]
mod i18n_bundle;
#[path = "src/schema.rs"]
mod schema;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let i18n_dir = Path::new("assets/i18n");
    let manifest_template = Path::new("component.manifest.template.json");
    let manifest_output = Path::new("component.manifest.json");
    let schema_output = Path::new("schemas/component.schema.json");

    println!("cargo:rerun-if-changed={}", i18n_dir.display());
    println!("cargo:rerun-if-changed={}", manifest_template.display());
    println!("cargo:rerun-if-changed={}", schema_output.display());

    let locales = i18n_bundle::load_locale_files(i18n_dir)
        .unwrap_or_else(|err| panic!("failed to load locale files: {err}"));
    let bundle = i18n_bundle::pack_locales_to_cbor(&locales)
        .unwrap_or_else(|err| panic!("failed to pack locale bundle: {err}"));

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by cargo");
    let bundle_path = Path::new(&out_dir).join("i18n.bundle.cbor");
    fs::write(&bundle_path, bundle).expect("write i18n.bundle.cbor");

    let rs_path = Path::new(&out_dir).join("i18n_bundle.rs");
    fs::write(
        &rs_path,
        "pub const I18N_BUNDLE_CBOR: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/i18n.bundle.cbor\"));\n",
    )
    .expect("write i18n_bundle.rs");

    let manifest = render_manifest_template(manifest_template, manifest_output);
    write_if_changed(manifest_output, &manifest);

    let config_schema = serde_json::to_string_pretty(&schema::oauth_config_schema_json())
        .expect("serialize config schema");
    write_if_changed(schema_output, &format!("{config_schema}\n"));
}

fn render_manifest_template(template_path: &Path, manifest_output: &Path) -> String {
    let template = fs::read_to_string(template_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", template_path.display()));

    let package_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME must be set");
    let package_version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION must be set");
    let package_name_underscore = package_name.replace('-', "_");
    let current_hash = current_component_hash(manifest_output);

    let rendered = template
        .replace("${CARGO_PKG_NAME}", &package_name)
        .replace("${CARGO_PKG_VERSION}", &package_version)
        .replace("${CARGO_PKG_NAME_UNDERSCORE}", &package_name_underscore);

    if let Some(hash) = current_hash {
        preserve_component_hash(&rendered, &hash)
    } else {
        rendered
    }
}

fn current_component_hash(manifest_output: &Path) -> Option<String> {
    let text = fs::read_to_string(manifest_output).ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;
    json.get("hashes")?
        .get("component_wasm")?
        .as_str()
        .map(ToOwned::to_owned)
}

fn preserve_component_hash(rendered: &str, hash: &str) -> String {
    let mut json: serde_json::Value =
        serde_json::from_str(rendered).expect("rendered manifest should be valid json");
    if let Some(hashes) = json
        .get_mut("hashes")
        .and_then(serde_json::Value::as_object_mut)
    {
        hashes.insert(
            "component_wasm".to_string(),
            serde_json::Value::String(hash.to_string()),
        );
    }
    let mut output = serde_json::to_string_pretty(&json).expect("serialize rendered manifest");
    output.push('\n');
    output
}

fn write_if_changed(path: &Path, contents: &str) {
    let current = fs::read_to_string(path).ok();
    if current.as_deref() == Some(contents) {
        return;
    }
    fs::write(path, contents)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", path.display()));
}
