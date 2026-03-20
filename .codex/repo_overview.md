# Repository Overview

## 1. High-Level Purpose
Greentic OAuth card component implemented for wasm32-wasip2 on the self-describing split `greentic:component/component-v0-v6-v0@0.6.0` ABI. The component exposes an OAuth card runtime operation plus QA/i18n helper operations, imports the Greentic OAuth broker through the same local WIT world, and embeds locale bundles into the wasm at build time.

## 2. Main Components and Functionality
- **Path:** `src/lib.rs`  
  **Role:** Component entrypoint, operation dispatcher, and schema helpers.  
  **Key functionality:** Routes `oauth_card.handle_message`, `component-info`, `qa-spec`, `apply-answers`, and `i18n-keys`; merges runtime config defaults into invoke payloads; emits JSON schema documents for the self-describing manifest/describe surface. The described component version is sourced from `CARGO_PKG_VERSION`, and the self-description explicitly documents the runtime dependency on the Greentic OAuth provider extension/broker.
- **Path:** `src/logic.rs`  
  **Role:** OAuth runtime behavior.  
  **Key functionality:** Handles status-card, start-sign-in, complete-sign-in, ensure-token, and disconnect flows; builds reconnect/sign-in/connected cards; emits auth context and authorization headers from broker tokens.
- **Path:** `src/broker.rs`  
  **Role:** OAuth broker abstraction.  
  **Key functionality:** Defines the `OAuthBackend` trait, native `NoopBroker`, wasm `HostBroker`, and test `MockBroker`; includes JSON input parsing for OAuth card requests.
- **Path:** `src/model.rs`  
  **Role:** JSON data contract model.  
  **Key functionality:** Defines OAuth input/output structs, token/auth structures, card/action types, and OAuth provider/prompt/status enums.
- **Path:** `src/qa.rs`  
  **Role:** Setup/update/remove QA contract.  
  **Key functionality:** Normalizes lifecycle aliases through `NormalizedMode`; emits QA specs with i18n-backed labels; applies setup answers into runtime config and validates remove/setup edge cases.
- **Path:** `src/i18n.rs` and `src/i18n_bundle.rs`  
  **Role:** Translation bundle loading and lookup.  
  **Key functionality:** Packs `assets/i18n/*.json` into canonical CBOR during build, exposes runtime lookup with locale fallback, and returns the full English key set for `i18n-keys`.
- **Path:** `build.rs`  
  **Role:** Build-time asset generation.  
  **Key functionality:** Embeds the i18n CBOR bundle and renders `component.manifest.json` from `component.manifest.template.json`, keeping the manifest version and package metadata aligned with `Cargo.toml`.
- **Path:** `component.manifest.template.json` and generated `component.manifest.json`  
  **Role:** Manifest source and concrete artifact.  
  **Key functionality:** Define the 0.6.0 component manifest, runtime capabilities, config schema, operation schemas, and artifact path. The generated manifest tracks `CARGO_PKG_NAME` and `CARGO_PKG_VERSION`.
- **Path:** `schemas/component.schema.json`  
  **Role:** Standalone config schema file.  
  **Key functionality:** Mirrors the runtime config schema used by `apply-answers` and the manifest (`provider_id`, default subject, scopes, tenant/team, redirect path, auto sign-in).
- **Path:** `assets/i18n/en.json`, `assets/i18n/locales.json`, and `tools/i18n.sh`  
  **Role:** Translation source and translation workflow.  
  **Key functionality:** English source strings for QA flows and component self-description, locale registry, and helper script that enforces a minimum translator batch size of `500`.
- **Path:** `tests/conformance.rs` and module tests in `src/lib.rs`, `src/logic.rs`, `src/broker.rs`, `src/qa.rs`, `src/i18n_bundle.rs`  
  **Role:** Automated verification.  
  **Key functionality:** Covers manifest/version invariants, schema file presence, README requirement documentation, dispatch/config merge behavior, QA normalization and config application, broker parsing, i18n bundle round-trip, and most OAuth runtime branches.
- **Path:** `Makefile` and `ci/local_check.sh`  
  **Role:** Local verification entrypoints.  
  **Key functionality:** Provide `fmt`, `clippy`, `test`, `wasm`, and `doctor` targets plus the CI wrapper script used for final local validation.

## 3. Work In Progress, TODOs, and Stubs
- Wasm broker integration still depends on the guest oauth broker bindings linking correctly in this repo’s environment; the native test backend remains a no-op placeholder.
- `component.manifest.json` is generated from the template during build, so edits should be applied to `component.manifest.template.json` rather than directly to the generated file.
- The manifest hash remains the placeholder value until a wasm artifact is built and hashed through `make wasm`.

## 4. Broken, Failing, or Conflicting Areas
- `make wasm` / actual wasm linking may still fail if the `greentic-interfaces-guest` oauth broker feature expects host exports incompatible with the current runner/tooling setup. Native tests and clippy currently pass.

## 5. Notes for Future Work
- Add direct wasm-level doctor/invocation tests once the broker guest bindings are stable in this repository.
- Consider generating `schemas/component.schema.json` from the same Rust/schema source used by `src/lib.rs` to remove one remaining schema duplication point.
