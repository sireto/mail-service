//! End-to-end check that stored credentials are ciphertext in the column and plaintext to
//! the caller.
//!
//! Every other test in this suite is mock-backed and needs no database. This one is the
//! exception, because the property it checks is precisely what the database holds, and a
//! mock cannot tell you that. It is therefore ignored by default and run explicitly:
//!
//! ```bash
//! createdb ms_crypto_check
//! for d in migrations/*/; do psql -f "$d/up.sql" ms_crypto_check; done
//! DATABASE_URL=postgres://localhost/ms_crypto_check \
//!   DATA_ENCRYPTION_KEY=any-long-random-string-at-least-32-chars \
//!   cargo test --test encryption_db -- --ignored --test-threads=1
//! ```

use diesel::prelude::*;
use uuid::{uuid, Uuid};

use backend::servers::servers_model::{ServerRequest, ServerTypeEnum, TlsTypeEnum};
use backend::servers::servers_repo::{ServerRepo, ServerRepoImpl};
use backend::utils::crypto;

const NAMESPACE: Uuid = uuid!("e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82");
const PLAINTEXT_PASSWORD: &str = "s3cr3t-smtp-password";

fn request() -> ServerRequest {
    ServerRequest {
        active: true,
        host: "smtp.example.com".to_string(),
        smtp_username: "user@example.com".to_string(),
        smtp_password: PLAINTEXT_PASSWORD.to_string(),
        namespace_id: NAMESPACE,
        tls_type: TlsTypeEnum::STARTTLS,
        port: 587,
        server_type: ServerTypeEnum::SMTP,
        aws_credentials: Some(serde_json::json!({
            "access_key_id": "AKIAREALKEY",
            "secret_access_key": "realsecretvalue",
            "region": "ap-southeast-1"
        })),
        default_from_email: "from@example.com".to_string(),
        rate_limit: 30,
    }
}

/// Read the column straight out of Postgres, bypassing the repository, so we see exactly
/// what was persisted rather than what the repository chooses to return.
fn raw_column_values(server_id: Uuid) -> (String, serde_json::Value) {
    use backend::schema::servers::dsl::*;
    let mut conn = backend::establish_connection().get().expect("a pooled connection");

    servers
        .filter(id.eq(server_id))
        .select((smtp_password, aws_credentials))
        .first::<(String, Option<serde_json::Value>)>(&mut conn)
        .map(|(pw, creds)| (pw, creds.unwrap_or(serde_json::Value::Null)))
        .expect("the row exists")
}

#[tokio::test]
#[ignore = "needs a migrated Postgres; see the module docs"]
async fn credentials_are_ciphertext_in_the_column_and_plaintext_to_the_caller() {
    let repo = ServerRepoImpl;

    let created = repo.create_server(request()).await.expect("server is created");

    // The repository hands back plaintext, so nothing above it needs to know.
    assert_eq!(created.smtp_password, PLAINTEXT_PASSWORD);
    let creds = created.aws_credentials.clone().expect("credentials");
    assert_eq!(creds["access_key_id"], serde_json::json!("AKIAREALKEY"));
    assert_eq!(creds["secret_access_key"], serde_json::json!("realsecretvalue"));

    // The column does not.
    let (raw_password, raw_creds) = raw_column_values(created.id);
    assert!(
        crypto::is_encrypted(&raw_password),
        "smtp_password was stored without encryption: {raw_password}"
    );
    assert!(
        !raw_password.contains(PLAINTEXT_PASSWORD),
        "the plaintext password is recoverable from the column"
    );
    assert!(crypto::is_encrypted(raw_creds["access_key_id"].as_str().unwrap()));
    assert!(crypto::is_encrypted(raw_creds["secret_access_key"].as_str().unwrap()));
    // Non-secret fields stay queryable.
    assert_eq!(raw_creds["region"], serde_json::json!("ap-southeast-1"));

    // Reading it back by id decrypts too, not just the create path.
    let fetched = repo.get_server_by_id(created.id).await.expect("server is found");
    assert_eq!(fetched.smtp_password, PLAINTEXT_PASSWORD);

    // And so does the list path.
    let listed = repo.get_all_servers().await.expect("servers are listed");
    let found = listed
        .iter()
        .find(|s| s.id == created.id)
        .expect("the created server is listed");
    assert_eq!(found.smtp_password, PLAINTEXT_PASSWORD);

    // An update re-encrypts rather than double-encrypting or storing plaintext.
    let mut rotated = request();
    rotated.smtp_password = "rotated-password".to_string();
    let updated = repo
        .update_server(created.id, rotated)
        .await
        .expect("server is updated");
    assert_eq!(updated.smtp_password, "rotated-password");

    let (raw_after_update, _) = raw_column_values(created.id);
    assert!(crypto::is_encrypted(&raw_after_update));
    assert!(!raw_after_update.contains("rotated-password"));

    repo.delete_server(created.id).await.expect("cleanup");
}

/// Rows written before encryption existed have no version prefix. Reading them must keep
/// working, or deploying this feature takes the service down instead of migrating it.
#[tokio::test]
#[ignore = "needs a migrated Postgres with a legacy plaintext row; see the module docs"]
async fn legacy_plaintext_rows_still_read() {
    use backend::schema::servers::dsl::*;
    let mut conn = backend::establish_connection().get().expect("a pooled connection");

    let legacy_id: Uuid = diesel::insert_into(servers)
        .values((
            host.eq("smtp.legacy"),
            smtp_username.eq("u"),
            // Written directly, with no version prefix: exactly what an existing row holds.
            smtp_password.eq("legacy-plaintext-password"),
            namespace_id.eq(NAMESPACE),
            default_from_email.eq("a@b.c"),
        ))
        .returning(id)
        .get_result(&mut conn)
        .expect("legacy row is inserted");

    let repo = ServerRepoImpl;
    let fetched = repo.get_server_by_id(legacy_id).await.expect("legacy row is read");

    assert_eq!(
        fetched.smtp_password, "legacy-plaintext-password",
        "an unencrypted legacy value must be passed through, not rejected"
    );

    repo.delete_server(legacy_id).await.expect("cleanup");
}
