//! Encryption at rest for credentials stored in the database.
//!
//! Before this, `servers.smtp_password` and the secrets inside `servers.aws_credentials`
//! were stored exactly as typed, so any database read, dump or backup exposed every
//! tenant's SMTP password and AWS keys. Masking them in API responses did nothing about
//! that: masking changes what is returned, not what is stored.
//!
//! Design, and the reasoning behind each choice:
//!
//! - **AES-256-GCM**, not a plain cipher. GCM is authenticated, so a ciphertext someone has
//!   edited in the database fails to decrypt instead of silently yielding a different
//!   password and a confusing SMTP auth failure.
//! - **A fresh random 96-bit nonce per value**, stored alongside the ciphertext. Reusing a
//!   nonce with the same key is the one mistake that breaks GCM badly, so it is never
//!   derived from the plaintext or a counter.
//! - **HKDF-SHA256 to derive the key** from `DATA_ENCRYPTION_KEY`. That lets the operator
//!   supply any random string as a seed rather than exactly 32 bytes of base64, without
//!   the low-entropy input becoming the key directly.
//! - **A version tag on every stored value** (`v1:`). Anything without a recognised tag is
//!   treated as pre-encryption plaintext and passed through, so deploying this does not
//!   break the rows that already exist. Re-saving a server encrypts it.
//!
//! The key is only as protected as the deployment environment holding it, and losing it
//! makes every stored credential unrecoverable. Both are documented in DEVELOPER.md.

use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use hkdf::Hkdf;
use serde_json::Value;
use sha2::Sha256;
use std::env;
use zeroize::Zeroize;

use crate::error::AppError;

/// Environment variable holding the seed the data key is derived from.
pub const KEY_ENV_VAR: &str = "DATA_ENCRYPTION_KEY";

/// Prefix marking a value this module produced. Values without it are legacy plaintext.
const VERSION_PREFIX: &str = "v1:";

/// Domain separation for the HKDF expansion, so the same seed used elsewhere would not
/// produce the same key.
const HKDF_INFO: &[u8] = b"mail-service:v1:column-encryption";

/// Shortest seed accepted. Not a substitute for the seed being random, but it stops
/// `DATA_ENCRYPTION_KEY=test` from reaching production.
const MIN_SEED_LEN: usize = 32;

/// Keys inside `aws_credentials` that are secret and therefore encrypted.
///
/// `region` and anything else stay in cleartext: encrypting them would make the JSON
/// unqueryable for no benefit.
pub const ENCRYPTED_AWS_KEYS: [&str; 3] = ["access_key_id", "secret_access_key", "session_token"];

fn derive_key() -> Result<[u8; 32], AppError> {
    let seed = env::var(KEY_ENV_VAR).map_err(|_| {
        AppError::InternalServerError(Some(format!(
            "{KEY_ENV_VAR} is not set. Generate one with: openssl rand -base64 48"
        )))
    })?;

    let trimmed = seed.trim();
    if trimmed.len() < MIN_SEED_LEN {
        return Err(AppError::InternalServerError(Some(format!(
            "{KEY_ENV_VAR} must be at least {MIN_SEED_LEN} characters"
        ))));
    }

    let hkdf = Hkdf::<Sha256>::new(None, trimmed.as_bytes());
    let mut key = [0u8; 32];
    hkdf.expand(HKDF_INFO, &mut key)
        .map_err(|_| AppError::InternalServerError(Some("Failed to derive the data encryption key".to_string())))?;

    Ok(key)
}

fn cipher() -> Result<Aes256Gcm, AppError> {
    let mut key_bytes = derive_key()?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    // The derived copy is no longer needed; the cipher holds its own.
    key_bytes.zeroize();
    Ok(cipher)
}

/// True when `stored` was produced by this module rather than being legacy plaintext.
pub fn is_encrypted(stored: &str) -> bool {
    stored.starts_with(VERSION_PREFIX)
}

/// Verify at startup that the key is usable, so a misconfiguration fails immediately
/// rather than at the first send.
pub fn verify_key_available() -> Result<(), AppError> {
    let probe = encrypt("startup-probe")?;
    let round_tripped = decrypt(&probe)?;

    if round_tripped != "startup-probe" {
        return Err(AppError::InternalServerError(Some(
            "Data encryption self-check failed".to_string(),
        )));
    }

    Ok(())
}

/// Encrypt a value for storage. Empty input is left empty: an absent credential should
/// stay visibly absent rather than becoming an opaque blob.
pub fn encrypt(plaintext: &str) -> Result<String, AppError> {
    if plaintext.is_empty() {
        return Ok(String::new());
    }

    let cipher = cipher()?;

    // A fresh random nonce per value, from the OS CSPRNG.
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| AppError::InternalServerError(Some("Failed to encrypt value".to_string())))?;

    // nonce || ciphertext||tag, so decryption needs nothing but the stored string.
    let mut payload = Vec::with_capacity(nonce.len() + ciphertext.len());
    payload.extend_from_slice(&nonce);
    payload.extend_from_slice(&ciphertext);

    Ok(format!("{VERSION_PREFIX}{}", BASE64.encode(payload)))
}

/// Decrypt a stored value.
///
/// A value without the version prefix is returned unchanged. That is deliberate: rows
/// written before this feature existed are plaintext, and refusing to read them would take
/// the service down on deploy rather than migrating it forward.
pub fn decrypt(stored: &str) -> Result<String, AppError> {
    let Some(encoded) = stored.strip_prefix(VERSION_PREFIX) else {
        return Ok(stored.to_string());
    };

    let payload = BASE64
        .decode(encoded)
        .map_err(|_| AppError::InternalServerError(Some("Stored value is not valid base64".to_string())))?;

    if payload.len() <= 12 {
        return Err(AppError::InternalServerError(Some(
            "Stored value is too short to be a ciphertext".to_string(),
        )));
    }

    let (nonce_bytes, ciphertext) = payload.split_at(12);
    let cipher = cipher()?;

    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| {
            // Wrong key, or the row was edited. Either way the detail belongs in the log,
            // not in a response.
            AppError::InternalServerError(Some(
                "Failed to decrypt a stored credential: wrong DATA_ENCRYPTION_KEY, or the value was modified"
                    .to_string(),
            ))
        })?;

    String::from_utf8(plaintext)
        .map_err(|_| AppError::InternalServerError(Some("Decrypted value is not valid UTF-8".to_string())))
}

/// Encrypt the secret members of an `aws_credentials` blob, leaving the rest alone.
pub fn encrypt_aws_credentials(credentials: Option<Value>) -> Result<Option<Value>, AppError> {
    let Some(mut credentials) = credentials else {
        return Ok(None);
    };

    if let Value::Object(map) = &mut credentials {
        for key in ENCRYPTED_AWS_KEYS {
            if let Some(Value::String(value)) = map.get(key) {
                if value.is_empty() || is_encrypted(value) {
                    continue;
                }
                let encrypted = encrypt(value)?;
                map.insert(key.into(), Value::String(encrypted));
            }
        }
    }

    Ok(Some(credentials))
}

/// Inverse of `encrypt_aws_credentials`.
pub fn decrypt_aws_credentials(credentials: Option<Value>) -> Result<Option<Value>, AppError> {
    let Some(mut credentials) = credentials else {
        return Ok(None);
    };

    if let Value::Object(map) = &mut credentials {
        for key in ENCRYPTED_AWS_KEYS {
            if let Some(Value::String(value)) = map.get(key) {
                if !is_encrypted(value) {
                    continue;
                }
                let decrypted = decrypt(value)?;
                map.insert(key.into(), Value::String(decrypted));
            }
        }
    }

    Ok(Some(credentials))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Mutex;

    /// These tests mutate a process-global environment variable, so they must not interleave.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    const TEST_SEED: &str = "test-seed-that-is-long-enough-to-be-accepted-0123456789";

    fn with_key<T>(seed: &str, f: impl FnOnce() -> T) -> T {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let previous = env::var(KEY_ENV_VAR).ok();
        env::set_var(KEY_ENV_VAR, seed);

        let result = f();

        match previous {
            Some(value) => env::set_var(KEY_ENV_VAR, value),
            None => env::remove_var(KEY_ENV_VAR),
        }
        result
    }

    #[test]
    fn round_trips_a_secret() {
        with_key(TEST_SEED, || {
            let stored = encrypt("s3cr3t-smtp-password").expect("encrypts");

            // What lands in the database must not contain the plaintext.
            assert!(!stored.contains("s3cr3t-smtp-password"));
            assert!(is_encrypted(&stored));

            assert_eq!(decrypt(&stored).expect("decrypts"), "s3cr3t-smtp-password");
        });
    }

    #[test]
    fn encrypting_twice_gives_different_ciphertexts() {
        with_key(TEST_SEED, || {
            let first = encrypt("same-input").expect("encrypts");
            let second = encrypt("same-input").expect("encrypts");

            // A fresh nonce each time; equal ciphertexts would leak that two servers share
            // a password.
            assert_ne!(first, second);
            assert_eq!(decrypt(&first).unwrap(), decrypt(&second).unwrap());
        });
    }

    #[test]
    fn a_tampered_ciphertext_is_rejected() {
        with_key(TEST_SEED, || {
            let stored = encrypt("original").expect("encrypts");
            let encoded = stored.strip_prefix(VERSION_PREFIX).unwrap();
            let mut payload = BASE64.decode(encoded).unwrap();

            // Flip a bit in the ciphertext body, past the nonce.
            let last = payload.len() - 1;
            payload[last] ^= 0x01;
            let tampered = format!("{VERSION_PREFIX}{}", BASE64.encode(payload));

            assert!(
                decrypt(&tampered).is_err(),
                "GCM must reject a modified ciphertext rather than returning garbage"
            );
        });
    }

    #[test]
    fn the_wrong_key_cannot_decrypt() {
        let stored = with_key(TEST_SEED, || encrypt("secret").expect("encrypts"));

        with_key("a-completely-different-seed-also-long-enough-0123456789", || {
            assert!(decrypt(&stored).is_err());
        });
    }

    #[test]
    fn legacy_plaintext_is_read_through_unchanged() {
        with_key(TEST_SEED, || {
            // Rows written before this feature existed have no version prefix.
            assert_eq!(decrypt("plain-old-password").unwrap(), "plain-old-password");
            assert!(!is_encrypted("plain-old-password"));
        });
    }

    #[test]
    fn empty_values_stay_empty() {
        with_key(TEST_SEED, || {
            assert_eq!(encrypt("").unwrap(), "");
            assert_eq!(decrypt("").unwrap(), "");
        });
    }

    #[test]
    fn a_missing_or_weak_key_is_refused() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let previous = env::var(KEY_ENV_VAR).ok();

        env::remove_var(KEY_ENV_VAR);
        assert!(
            encrypt("x").is_err(),
            "no key must be an error, not a silent passthrough"
        );

        env::set_var(KEY_ENV_VAR, "too-short");
        assert!(encrypt("x").is_err(), "a trivially guessable seed must be refused");

        match previous {
            Some(value) => env::set_var(KEY_ENV_VAR, value),
            None => env::remove_var(KEY_ENV_VAR),
        }
    }

    #[test]
    fn aws_credentials_encrypt_only_their_secrets() {
        with_key(TEST_SEED, || {
            let plain = json!({
                "access_key_id": "AKIAREAL",
                "secret_access_key": "realsecret",
                "region": "ap-southeast-1"
            });

            let encrypted = encrypt_aws_credentials(Some(plain)).unwrap().unwrap();

            assert!(is_encrypted(encrypted["access_key_id"].as_str().unwrap()));
            assert!(is_encrypted(encrypted["secret_access_key"].as_str().unwrap()));
            // region is not a secret and stays queryable.
            assert_eq!(encrypted["region"], json!("ap-southeast-1"));

            let decrypted = decrypt_aws_credentials(Some(encrypted)).unwrap().unwrap();
            assert_eq!(decrypted["access_key_id"], json!("AKIAREAL"));
            assert_eq!(decrypted["secret_access_key"], json!("realsecret"));
            assert_eq!(decrypted["region"], json!("ap-southeast-1"));
        });
    }

    #[test]
    fn encrypting_credentials_is_idempotent() {
        with_key(TEST_SEED, || {
            let once = encrypt_aws_credentials(Some(json!({ "access_key_id": "AKIA" })))
                .unwrap()
                .unwrap();
            let twice = encrypt_aws_credentials(Some(once.clone())).unwrap().unwrap();

            // Double-encrypting on a re-save would make the value unrecoverable.
            assert_eq!(once, twice);
            assert_eq!(
                decrypt_aws_credentials(Some(twice)).unwrap().unwrap()["access_key_id"],
                json!("AKIA")
            );
        });
    }

    #[test]
    fn startup_self_check_passes_with_a_valid_key() {
        with_key(TEST_SEED, || assert!(verify_key_available().is_ok()));
    }
}
