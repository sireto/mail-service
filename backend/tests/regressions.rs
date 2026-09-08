//! Regression tests for defects that were found and fixed in this codebase.
//!
//! Each test pins down one specific defect and fails if it returns.
//! These are deliberately behavioural rather than "mock returns X, assert X" — the
//! pre-existing suite was entirely the latter, which is why none of these bugs were caught.

use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use backend::models::contact::Contact;
use backend::models::pagination::PageQuery;
use backend::repositories::contact::MockContactRepository;
use backend::servers::servers_model::{Server, ServerRequest, ServerTypeEnum, TlsTypeEnum};
use backend::servers::servers_repo::MockServerRepo;
use backend::servers::servers_services::{ServerService, ServerServiceTrait};
use backend::services::contact_service::ContactService;
use backend::services::mail_service::{has_reached_send_attempt_limit, sanitize_rate_limit, MAX_SEND_ATTEMPTS};
use backend::utils::bounce_logs::is_trusted_sns_url;
use backend::utils::contact_lists_functions::parse_csv_data;
use backend::utils::server_utils::{is_masked_secret, resolve_aws_credentials, resolve_secret, MASKED_SECRET};

fn smtp_server(password: &str, aws: Option<serde_json::Value>) -> Server {
    Server {
        id: Uuid::new_v4(),
        active: true,
        host: "smtp.example.com".to_string(),
        smtp_username: "user@example.com".to_string(),
        smtp_password: password.to_string(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::STARTTLS,
        port: 587,
        server_type: ServerTypeEnum::SMTP,
        aws_credentials: aws,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        default_from_email: "from@example.com".to_string(),
        rate_limit: 30,
    }
}

fn server_request(password: &str, aws: Option<serde_json::Value>) -> ServerRequest {
    ServerRequest {
        active: true,
        host: "smtp.example.com".to_string(),
        smtp_username: "user@example.com".to_string(),
        smtp_password: password.to_string(),
        namespace_id: Uuid::new_v4(),
        tls_type: TlsTypeEnum::STARTTLS,
        port: 587,
        server_type: ServerTypeEnum::SMTP,
        aws_credentials: aws,
        default_from_email: "from@example.com".to_string(),
        rate_limit: 30,
    }
}

/// C6: the read path masks smtp_password, the client round-trips what it was shown, and
/// PATCH replaces every column — so saving a server used to persist the mask itself and
/// destroy the credential. update_server must now treat a mask as "unchanged".
#[tokio::test]
async fn c6_updating_a_server_with_a_masked_password_keeps_the_stored_one() {
    let stored = smtp_server("the-real-password", None);
    let server_id = stored.id;

    let mut repo = MockServerRepo::new();
    let stored_for_get = stored.clone();
    repo.expect_get_server_by_id()
        .returning(move |_| Ok(stored_for_get.clone()));

    // The assertion lives here: whatever reaches the repository is what gets written.
    repo.expect_update_server().returning(|_, payload| {
        assert_eq!(
            payload.smtp_password, "the-real-password",
            "the mask was written to the database instead of the real password"
        );
        Ok(smtp_server(&payload.smtp_password, payload.aws_credentials.clone()))
    });

    let service = ServerService::new(Arc::new(repo));
    let updated = service
        .update_server(&server_id.to_string(), server_request(MASKED_SECRET, None))
        .await
        .expect("update should succeed");

    assert_eq!(updated.smtp_password, "the-real-password");
}

/// C6, rotation half: a genuinely new password must still be applied, or the fix above
/// would make passwords unchangeable.
#[tokio::test]
async fn c6_a_real_new_password_is_still_written() {
    let stored = smtp_server("old-password", None);
    let server_id = stored.id;

    let mut repo = MockServerRepo::new();
    let stored_for_get = stored.clone();
    repo.expect_get_server_by_id()
        .returning(move |_| Ok(stored_for_get.clone()));
    repo.expect_update_server().returning(|_, payload| {
        assert_eq!(payload.smtp_password, "rotated-password");
        Ok(smtp_server(&payload.smtp_password, None))
    });

    let service = ServerService::new(Arc::new(repo));
    service
        .update_server(&server_id.to_string(), server_request("rotated-password", None))
        .await
        .expect("update should succeed");
}

/// C6, AWS half: the same round-trip destroyed access_key_id and secret_access_key.
#[tokio::test]
async fn c6_masked_aws_credentials_are_merged_over_the_stored_blob() {
    let stored_creds = serde_json::json!({
        "access_key_id": "AKIAREAL",
        "secret_access_key": "realsecret",
        "region": "ap-southeast-1"
    });
    let stored = smtp_server("pw", Some(stored_creds));
    let server_id = stored.id;

    let mut repo = MockServerRepo::new();
    let stored_for_get = stored.clone();
    repo.expect_get_server_by_id()
        .returning(move |_| Ok(stored_for_get.clone()));
    repo.expect_update_server().returning(|_, payload| {
        let creds = payload.aws_credentials.clone().expect("credentials present");
        assert_eq!(creds["access_key_id"], serde_json::json!("AKIAREAL"));
        assert_eq!(creds["secret_access_key"], serde_json::json!("realsecret"));
        // A non-secret edit in the same request still lands.
        assert_eq!(creds["region"], serde_json::json!("eu-west-1"));
        Ok(smtp_server(&payload.smtp_password, payload.aws_credentials.clone()))
    });

    let incoming = serde_json::json!({
        "access_key_id": "*************",
        "secret_access_key": "*************",
        "region": "eu-west-1"
    });

    let service = ServerService::new(Arc::new(repo));
    service
        .update_server(&server_id.to_string(), server_request(MASKED_SECRET, Some(incoming)))
        .await
        .expect("update should succeed");
}

#[test]
fn c6_mask_detection_does_not_swallow_real_passwords() {
    assert!(is_masked_secret("***********"));
    assert!(is_masked_secret("*************"));
    assert!(!is_masked_secret(""));
    // A password that merely contains asterisks is a real password.
    assert!(!is_masked_secret("p*ssw*rd"));
    assert_eq!(resolve_secret("p*ssw*rd", "stored"), "p*ssw*rd");
    assert_eq!(resolve_aws_credentials(None, None), None);
}

/// C3: the SNS subscription-confirmation handler performs an outbound GET to a URL taken
/// from an unauthenticated request body. Only real Amazon SNS endpoints may be fetched.
#[test]
fn c3_only_amazon_sns_urls_are_fetchable() {
    assert!(is_trusted_sns_url("https://sns.eu-west-1.amazonaws.com/?Token=x"));

    // The canonical SSRF target.
    assert!(!is_trusted_sns_url("http://169.254.169.254/latest/meta-data/"));
    // Internal service on the compose network.
    assert!(!is_trusted_sns_url("http://postgres:5432/"));
    // Domain-suffix smuggling.
    assert!(!is_trusted_sns_url("https://sns.amazonaws.com.evil.tld/"));
    // Userinfo trick: the real host is evil.tld.
    assert!(!is_trusted_sns_url("https://sns.us-east-1.amazonaws.com@evil.tld/"));
}

/// M11: rate_limit is an unconstrained i32 read into a u32 quota. A negative value wrapped
/// to an enormous number, which silently removed the rate limit entirely.
#[test]
fn m11_a_negative_rate_limit_becomes_the_slowest_setting_not_the_fastest() {
    assert_eq!(sanitize_rate_limit(-1).get(), 1);
    assert_eq!(sanitize_rate_limit(i32::MIN).get(), 1);
    assert_eq!(sanitize_rate_limit(0).get(), 1);

    // Sane values pass through untouched.
    assert_eq!(sanitize_rate_limit(30).get(), 30);
    // And absurd ones are capped rather than trusted.
    assert_eq!(sanitize_rate_limit(i32::MAX).get(), 10_000);
}

/// L7: an empty "delimiter" form field indexed out of bounds and panicked the request.
#[test]
fn l7_csv_import_handles_odd_delimiters_without_panicking() {
    let csv = b"email,name\na@example.com,Ada Lovelace\n";

    let namespace = Uuid::new_v4();

    let parsed = parse_csv_data(csv, namespace, ",", "subscribe", "unconfirmed", false).expect("comma parses");
    assert_eq!(
        parsed[0].namespace_id, namespace,
        "imported rows must carry the namespace"
    );
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].email, "a@example.com");
    assert_eq!(parsed[0].first_name, "Ada");
    assert_eq!(parsed[0].last_name, "Lovelace");

    // Empty delimiter used to panic; it now falls back to a comma.
    assert!(parse_csv_data(csv, namespace, "", "subscribe", "unconfirmed", false).is_ok());

    // A multi-byte delimiter used to be silently truncated to its first byte.
    assert!(parse_csv_data(csv, namespace, "||", "subscribe", "unconfirmed", false).is_err());

    // A file with no email column is still rejected.
    assert!(parse_csv_data(b"name\nAda\n", namespace, ",", "subscribe", "unconfirmed", false).is_err());
}

/// A failed final attempt must be recognized immediately, while a row already at or above
/// the cap must also be eligible for finalization after a worker restart.
#[test]
fn m4_retry_cap_is_a_single_shared_constant() {
    assert_eq!(MAX_SEND_ATTEMPTS, 3);
    assert!(!has_reached_send_attempt_limit(MAX_SEND_ATTEMPTS - 1));
    assert!(has_reached_send_attempt_limit(MAX_SEND_ATTEMPTS));
    assert!(has_reached_send_attempt_limit(MAX_SEND_ATTEMPTS + 1));
}

/// M7: contacts had no tenant column and email was globally UNIQUE, so every namespace saw
/// every other namespace's subscribers and two tenants could not hold the same address.
///
/// The repository is now the boundary: the namespace is part of the lookup key, not an
/// optional filter applied afterwards. These assert on the arguments the service passes
/// down, because that is what the SQL predicate is built from.
#[tokio::test]
async fn m7_contact_lookups_are_scoped_to_a_namespace() {
    let namespace = Uuid::new_v4();
    let other_namespace = Uuid::new_v4();

    let mut repo = MockContactRepository::new();

    // Listing must carry the namespace through untouched.
    repo.expect_get_all_contacts()
        .withf(move |ns, _list, _search, _limit, _offset| *ns == namespace)
        .returning(|_, _, _, _, _| Ok((vec![], 0)));

    // Resolving an address must carry it too, otherwise the same email in another namespace
    // would be returned instead.
    repo.expect_get_contact_by_email()
        .withf(move |ns, email| *ns == namespace && email == "shared@example.com")
        .returning(move |ns, email| {
            Ok(Contact {
                id: Uuid::new_v4(),
                namespace_id: ns,
                first_name: "Scoped".into(),
                last_name: "Contact".into(),
                email,
                attribute: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        });

    let service = ContactService::new(Arc::new(repo));

    service
        .get_all_contacts(namespace, None, None, &PageQuery::default())
        .await
        .expect("listing is scoped");

    let found = service
        .get_contact_by_email(namespace, "shared@example.com".to_string())
        .await
        .expect("lookup is scoped");

    assert_eq!(
        found.namespace_id, namespace,
        "a contact resolved for one namespace must not belong to another"
    );
    assert_ne!(found.namespace_id, other_namespace);
}

/// The same address in two namespaces is two different contacts, so a CSV import must stamp
/// every row with the namespace it was imported into.
#[test]
fn m7_imported_rows_carry_their_namespace() {
    let namespace = Uuid::new_v4();
    let csv = b"email,name\nshared@example.com,Ada Lovelace\nshared2@example.com,Alan Turing\n";

    let parsed = parse_csv_data(csv, namespace, ",", "subscribe", "unconfirmed", false).expect("parses");

    assert_eq!(parsed.len(), 2);
    assert!(
        parsed.iter().all(|c| c.namespace_id == namespace),
        "every imported row must belong to the namespace it was imported into"
    );
}
