use serde_json::Value;

/// The placeholder substituted for a secret on the way out.
///
/// It has to be a named constant because the write path needs to recognise it. Responses
/// mask secrets, clients round-trip whatever they were given, and PATCH replaces every
/// column, so without this check saving a server wrote the mask itself into the database
/// and destroyed the real credential.
pub const MASKED_SECRET: &str = "***********";
const MASKED_AWS_SECRET: &str = "*************";

/// Keys inside `aws_credentials` that must never be returned to a client.
const AWS_SECRET_KEYS: [&str; 3] = ["access_key_id", "secret_access_key", "session_token"];

pub fn secure_server_response(mut creds: Value) -> Value {
    if let Value::Object(map) = &mut creds {
        for key in AWS_SECRET_KEYS {
            // Only mask keys that are actually present, so a credential blob without a
            // session token does not gain a fake one.
            if map.contains_key(key) {
                map.insert(key.into(), Value::String(MASKED_AWS_SECRET.to_string()));
            }
        }
    }
    creds
}

/// True when `value` is a mask this service produced rather than a real secret.
pub fn is_masked_secret(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.chars().all(|c| c == '*')
}

/// Choose the secret to persist: the incoming one, unless it is a mask or empty, in which
/// case the stored value is kept.
pub fn resolve_secret(incoming: &str, stored: &str) -> String {
    if incoming.trim().is_empty() || is_masked_secret(incoming) {
        stored.to_string()
    } else {
        incoming.to_string()
    }
}

/// Merge incoming `aws_credentials` over the stored blob, keeping any stored secret whose
/// replacement is a mask or blank.
pub fn resolve_aws_credentials(incoming: Option<Value>, stored: Option<Value>) -> Option<Value> {
    let Some(mut incoming) = incoming else {
        return stored;
    };

    let stored_map = match &stored {
        Some(Value::Object(map)) => map.clone(),
        _ => Default::default(),
    };

    if let Value::Object(map) = &mut incoming {
        for key in AWS_SECRET_KEYS {
            let incoming_is_placeholder = match map.get(key) {
                Some(Value::String(value)) => value.trim().is_empty() || is_masked_secret(value),
                Some(Value::Null) | None => true,
                _ => false,
            };

            if incoming_is_placeholder {
                match stored_map.get(key) {
                    Some(stored_value) => map.insert(key.into(), stored_value.clone()),
                    None => map.remove(key),
                };
            }
        }
    }

    Some(incoming)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn masks_every_aws_secret_present() {
        let masked = secure_server_response(json!({
            "access_key_id": "AKIAREAL",
            "secret_access_key": "realsecret",
            "session_token": "realtoken",
            "region": "ap-southeast-1"
        }));

        assert_eq!(masked["access_key_id"], json!(MASKED_AWS_SECRET));
        assert_eq!(masked["secret_access_key"], json!(MASKED_AWS_SECRET));
        // session_token used to be returned in the clear.
        assert_eq!(masked["session_token"], json!(MASKED_AWS_SECRET));
        // Non-secret fields are untouched.
        assert_eq!(masked["region"], json!("ap-southeast-1"));
    }

    #[test]
    fn does_not_invent_absent_keys() {
        let masked = secure_server_response(json!({ "region": "eu-west-1" }));
        assert!(masked.get("session_token").is_none());
    }

    #[test]
    fn keeps_stored_password_when_client_returns_the_mask() {
        assert_eq!(resolve_secret(MASKED_SECRET, "realpassword"), "realpassword");
        assert_eq!(resolve_secret("*************", "realpassword"), "realpassword");
        assert_eq!(resolve_secret("", "realpassword"), "realpassword");
        assert_eq!(resolve_secret("   ", "realpassword"), "realpassword");
    }

    #[test]
    fn accepts_a_genuine_new_password() {
        assert_eq!(resolve_secret("brand-new-secret", "realpassword"), "brand-new-secret");
        // A password that merely contains an asterisk is real.
        assert_eq!(resolve_secret("p*ssword", "realpassword"), "p*ssword");
    }

    #[test]
    fn merges_masked_aws_credentials_over_stored_ones() {
        let stored = json!({
            "access_key_id": "AKIAREAL",
            "secret_access_key": "realsecret",
            "region": "ap-southeast-1"
        });
        let incoming = json!({
            "access_key_id": MASKED_AWS_SECRET,
            "secret_access_key": MASKED_AWS_SECRET,
            "region": "eu-west-1"
        });

        let merged = resolve_aws_credentials(Some(incoming), Some(stored)).expect("merged");

        assert_eq!(merged["access_key_id"], json!("AKIAREAL"));
        assert_eq!(merged["secret_access_key"], json!("realsecret"));
        // A non-secret change still applies.
        assert_eq!(merged["region"], json!("eu-west-1"));
    }

    #[test]
    fn applies_genuinely_rotated_aws_credentials() {
        let stored = json!({ "access_key_id": "AKIAOLD", "secret_access_key": "old" });
        let incoming = json!({ "access_key_id": "AKIANEW", "secret_access_key": "new" });

        let merged = resolve_aws_credentials(Some(incoming), Some(stored)).expect("merged");

        assert_eq!(merged["access_key_id"], json!("AKIANEW"));
        assert_eq!(merged["secret_access_key"], json!("new"));
    }
}
