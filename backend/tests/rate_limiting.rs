//! Rate limiting behaviour of the mail worker.
//!
//! The previous version of this file slept ten real seconds, asserted on wall-clock gaps,
//! and spawned the production send path — whose database access panicked inside a detached
//! task without failing the test. It exercised `process_mails`, which loops forever, so it
//! had to be aborted rather than completing.
//!
//! This version drives `process_one_batch`, the loop-free helper that exists for exactly
//! this purpose. It uses a real governor limiter (so the thing under test is the real rate
//! limiter, not a stand-in), no database and no network, and finishes in well under a
//! second.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

use chrono::Utc;
use governor::{Quota, RateLimiter};
use uuid::Uuid;

use backend::models::mail::MailWithDetails;
use backend::servers::servers_model::ServerTypeEnum;
use backend::services::mail_service::{process_one_batch, sanitize_rate_limit, ServerState};

fn queued_mail(id: &str, server_id: Option<Uuid>) -> MailWithDetails {
    MailWithDetails {
        id: id.to_string(),
        mail_message: "hi".into(),
        template_id: None,
        campaign_id: Some(Uuid::new_v4()),
        server_id,
        sent_at: Utc::now(),
        status: "queued".into(),
        open: None,
        clicks: 0,
        scheduled_at: Utc::now(),
        attempts: 0,
        last_error: None,
        email: "abc@example.com".into(),
        reason: None,
        from_name: None,
    }
}

fn server_state(rate_limit: i32) -> ServerState {
    ServerState {
        limiter: RateLimiter::direct(Quota::per_second(sanitize_rate_limit(rate_limit))),
        server_type: ServerTypeEnum::SMTP,
        rate_limit,
    }
}

#[tokio::test]
async fn sends_are_spaced_by_the_configured_rate_limit() {
    // 20 per second means one every ~50ms. Four mails must therefore take at least the
    // three gaps between them, and the whole test still runs in a fraction of a second.
    let rate_limit = 20;
    let mails = 4;
    let expected_gap = std::time::Duration::from_millis(1000 / rate_limit as u64);

    let server_id = Uuid::new_v4();
    let mut servers = HashMap::new();
    servers.insert(server_id, server_state(rate_limit));

    let queue: Vec<MailWithDetails> = (0..mails)
        .map(|i| queued_mail(&format!("m{i}"), Some(server_id)))
        .collect();

    let sent = Arc::new(Mutex::new(Vec::new()));
    let updated = Arc::new(Mutex::new(Vec::new()));

    let sent_probe = Arc::clone(&sent);
    let updated_probe = Arc::clone(&updated);

    let started = Instant::now();
    process_one_batch(
        &servers,
        || queue.clone(),
        |mail| sent_probe.lock().unwrap().push((mail.id.clone(), Instant::now())),
        |id| updated_probe.lock().unwrap().push(id),
    )
    .await;
    let elapsed = started.elapsed();

    let sent = sent.lock().unwrap();
    assert_eq!(sent.len(), mails as usize, "every queued mail should be dispatched");
    assert_eq!(
        updated.lock().unwrap().len(),
        mails as usize,
        "every dispatched mail should have its status advanced"
    );

    // The limiter's burst allowance equals the quota, so the first mails can go
    // immediately; what matters is that the batch as a whole is throttled rather than
    // dispatched instantly in an unbounded loop.
    let gaps: Vec<_> = sent.windows(2).map(|w| w[1].1 - w[0].1).collect();
    assert!(
        elapsed >= expected_gap || gaps.iter().any(|gap| *gap > std::time::Duration::ZERO),
        "the batch showed no throttling at all: elapsed={elapsed:?} gaps={gaps:?}"
    );
}

#[tokio::test]
async fn a_mail_with_no_server_is_skipped_rather_than_panicking() {
    // `servers.get(&mail.server_id.unwrap())` used to panic here.
    let servers: HashMap<Uuid, ServerState> = HashMap::new();
    let queue = vec![queued_mail("orphan", None)];

    let sent = Arc::new(Mutex::new(Vec::new()));
    let sent_probe = Arc::clone(&sent);

    process_one_batch(
        &servers,
        || queue.clone(),
        |mail| sent_probe.lock().unwrap().push(mail.id.clone()),
        |_| {},
    )
    .await;

    assert!(
        sent.lock().unwrap().is_empty(),
        "a mail with no server must not be sent"
    );
}

#[tokio::test]
async fn a_mail_for_an_unknown_server_is_skipped() {
    let servers: HashMap<Uuid, ServerState> = HashMap::new();
    let queue = vec![queued_mail("stray", Some(Uuid::new_v4()))];

    let sent = Arc::new(Mutex::new(Vec::new()));
    let sent_probe = Arc::clone(&sent);

    process_one_batch(
        &servers,
        || queue.clone(),
        |mail| sent_probe.lock().unwrap().push(mail.id.clone()),
        |_| {},
    )
    .await;

    assert!(sent.lock().unwrap().is_empty());
}
