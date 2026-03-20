use serde_json::{Value, json};

pub fn oauth_config_schema_json() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "component-oauth-card configuration",
        "type": "object",
        "properties": {
            "provider_id": {
                "type": "string",
                "description": "OAuth provider identifier used by the upstream Greentic OAuth operations. This must match a provider exposed by the installed Greentic OAuth provider extension."
            },
            "default_subject": {
                "type": ["string", "null"],
                "description": "Default subject when the invoke payload omits `subject`."
            },
            "scopes": {
                "type": "array",
                "items": { "type": "string" },
                "default": []
            },
            "tenant": {
                "type": ["string", "null"]
            },
            "team": {
                "type": ["string", "null"]
            },
            "redirect_path": {
                "type": ["string", "null"]
            },
            "allow_auto_sign_in": {
                "type": "boolean",
                "default": false
            }
        },
        "required": [],
        "additionalProperties": false
    })
}
