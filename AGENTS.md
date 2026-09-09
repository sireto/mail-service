# AGENTS.md

Bulk email platform. Rust/axum + Diesel/Postgres backend in backend/, Next.js 16 App
Router frontend in frontend/. Sends through AWS SES v2 or a user-configured SMTP relay.
Human-facing setup lives in DEVELOPER.md and README.md; this file is the agent-facing
companion and does not repeat them.

DEVELOPER.md's "Known gaps" section lists what is still outstanding. The short version:
there is no authentication anywhere, namespaces are not a security boundary, and the SNS
webhook does not verify signatures.

## Environment limits here

The Rust toolchain is pinned by backend/rust-toolchain.toml to 1.95.0, because Cargo.lock's
aws-* crates set an MSRV of 1.94.1. On a machine whose default is older, cargo reports that
as a dependency error rather than a toolchain one. If rustup has not fetched the pinned
version, prefix commands with the toolchain explicitly.

frontend/node_modules is not committed. Run corepack enable and yarn install --immutable
before claiming any frontend result. Standalone tsc --noEmit reports a spurious error on
components/common/Loading.tsx's .gif import, because next-env.d.ts is gitignored and
generated during next build; use yarn build to typecheck, not tsc alone.

## Commands CI runs

.github/workflows/backend-test.yaml, two jobs. Backend, in backend/ on Rust 1.95.0:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --locked
cargo build --locked --no-default-features
cargo test --locked
```

Frontend, in frontend/ on Node 22:

```bash
yarn install --immutable
yarn lint
yarn build
```

Clippy is -D warnings, so any new warning fails the build. yarn format:check is
deliberately not gated: prettier has never run over this repository, so it would fail on
unrelated files. yarn lint currently emits 11 warnings and exits 0.

publish-ghcr.yaml publishes the backend image only for a push to develop or main, checked
out at the exact head_sha the tests passed on. latest is minted from main only; develop
gets nightly.

install-hooks.sh symlinks hooks/pre-commit.sh, which runs backend/pre-commit.sh and
frontend/pre-commit.sh for whichever directory has staged changes. Those mirror the CI
gates above.

## Traps

The mocks cargo feature gates every #[automock]. It is in default features so plain
cargo test works, and the Dockerfile builds --no-default-features so mockall stays out of
the release binary. A change that uses a Mock* type outside a test, or that adds an
#[automock] without the cfg_attr wrapper, breaks the --no-default-features build only:
CI checks that shape, a local cargo build does not.

Tests need no database. Every test in backend/tests/ is mock-backed and the whole suite
runs in about 25 seconds. Repositories fetch connections from the GLOBAL_APP_STATE Lazy in
backend/src/lib.rs, so a test that reaches a real repository panics on that static rather
than failing cleanly.

Wiring a service into a handler is not checked at compile time. Handlers take
Extension<Arc<SomeService>>, and each route module constructs and layers its own service
instances (see backend/src/routes/mail.rs). A handler that extracts an Extension its route
module does not layer compiles, then returns 500 at runtime.

An email address is only unique within a namespace. contacts_namespace_id_email_key replaced
the global unique on email, so every contact lookup takes the namespace as part of its key:
ContactRepository::get_contact_by_email and get_all_contacts both require it, and
CreateMailRequest carries a namespace_id for exactly this reason. Each send path already knows
the right one (campaign sends the campaign's, server sends the server's, template sends the
template's). The SNS webhook has none, so it recovers the contact from the mail row instead of
from the bounced address.

Secrets are masked on read and merged on write. ServerResponse replaces smtp_password and
the aws_credentials secrets with runs of asterisks, and ServerService::update_server treats
a masked or blank incoming secret as "leave unchanged" (backend/src/utils/server_utils.rs).
Both halves are load-bearing: PATCH is a full-column replace, so removing the merge makes
every server edit destroy its credentials. tests/regressions.rs pins this.

Mail status is a free-text column, but the vocabulary is now constants in
backend/src/services/mail_service.rs: queued, submitted, sent, delivered, failed, bounced.
Only enqueue_email may write "queued", the value the worker acts on; a path that has
already delivered must write "sent". Writing "queued" after a synchronous send is
what previously caused every SMTP campaign to be delivered twice. POST and PATCH
/api/mails reject "queued" for the same reason.

The worker claims a mail before dispatching it. process_mails sets "submitted" and only
then spawns the send; the spawned task records the outcome. Removing the claim reintroduces
duplicate sends on the next tick.

Queued mail requires a campaign. get_mails_by_status inner-joins campaigns and
campaign_senders, so a mail row with campaign_id NULL is invisible to the worker whatever
its status.

NEXT_PUBLIC_* variables must be build arguments. Next inlines them at build time. Both
NEXT_PUBLIC_BASE_URL and NEXT_PUBLIC_NAMESPACE_ID are wired as ARGs in frontend/Dockerfile
and as build args in both compose files. Adding a new one as a runtime environment entry
leaves it undefined in the browser with no error.

Migrations run automatically at startup (embed_migrations! in backend/src/main.rs) and the
process exits if Postgres is unreachable. Both compose files now gate on a postgres
healthcheck and set a restart policy; a new compose file needs the same.

DATA_ENCRYPTION_KEY is required to start. main.rs runs an encrypt/decrypt self-check before
anything else and panics if it fails, so a container without the key never comes up looking
healthy. Any environment that runs the backend or its tests needs the variable set; CI sets
a throwaway value. Changing it makes every stored credential unreadable, since the AES key
is derived from it by HKDF.

Stored credentials are encrypted, and the repository is the only place that knows. Encrypt
and decrypt happen inside backend/src/servers/servers_repo.rs, so every layer above works
with plaintext. A new query that reads or writes servers.smtp_password or
servers.aws_credentials without going through encrypt_payload/decrypt_server will store
plaintext or hand back ciphertext, and nothing will complain. Values carry a "v1:" prefix;
anything without it is treated as pre-encryption plaintext and read through unchanged, which
is what lets existing rows keep working.

List endpoints are paginated and the cap is not negotiable. limit defaults to 50 and is
clamped to 200 server-side (backend/src/models/pagination.rs), so a caller cannot ask for a
whole table. Responses are a Page envelope, not a bare array: items, total, limit, offset,
has_more. Anything that genuinely needs every row has to walk the pages, as
frontend/app/dashboard/contacts/export/page.tsx does.

## Couplings

Adding or changing a servers column touches, in this order: a new
backend/migrations/<timestamp>_name/{up,down}.sql, the servers table! block in
backend/src/schema.rs, three structs plus the From impl in
backend/src/servers/servers_model.rs (Server, ServerRequest, ServerResponse), the explicit
column list in get_all_servers and the explicit set() in update_server in
backend/src/servers/servers_repo.rs, then ServerSchema in frontend/lib/type.ts and the two
defaultValues/reset blocks in frontend/hooks/useServerForm.ts. Miss the repo's set() and
the field silently never persists on update; miss either form block and the field resets
whenever the dialog opens. If the column holds a secret, it also needs adding to
AWS_SECRET_KEYS or the resolve_secret path in backend/src/utils/server_utils.rs.

Adding a mails column additionally requires editing the explicit select tuple in every
query in backend/src/repositories/mail_repository.rs and
backend/src/servers/servers_repo.rs (get_mails_by_server_id), plus MailWithDetails and
GetMailResponse in backend/src/models/mail.rs. These selects are positional tuples: a
field added in the wrong position is a type error at best and wrong data at worst.

Adding a list endpoint means taking PageQuery in the handler, returning Page<T>, and
returning (rows, total) from the repository with a count query that repeats the same
filters. Diesel's boxed queries cannot be cloned, so the filters really do have to be
written twice; a count that drifts from the query is how has_more starts lying. Order by
something unique as a tiebreaker or offset paging will skip and repeat rows.

Adding an endpoint touches four files: the handler in backend/src/handlers/ (or
backend/src/servers/servers_handler.rs), its route module in backend/src/routes/, the
paths(...) list in backend/src/route.rs so it appears in Swagger, and (for a new file)
the manual module declaration in backend/src/lib.rs. lib.rs lists every module by hand; a
new file not listed there is simply not compiled.

The frontend namespace is one constant: NAMESPACE_ID in frontend/config/namespace.ts. It was
pasted literally into eleven files; do not reintroduce the literal. The API slices inject it,
so a new contact call site does not pass it explicitly. The remaining copies of the UUID are
the seed INSERT in backend/migrations/2025-01-19-115659_create_namespaces/up.sql, both compose
files, and one story fixture.

Contacts are namespace-scoped, but namespaces are still not a security boundary: the caller
asserts the namespace and nothing authenticates it (C1). Scope queries by it for correctness,
never as access control.

## Architecture notes worth knowing before editing

There is no authentication or authorization anywhere. No middleware, no token, no session.
Every route under /api is open. Until that changes, assume any caller is hostile and do
not add an endpoint that trusts its input about who it is or what tenant it belongs to.

The SNS webhook at POST /api/bounce-logs/sns/bounce is unauthenticated and its message
signature is still not verified. It refuses to fetch a SubscribeURL that is not an Amazon
SNS host (is_trusted_sns_url in backend/src/utils/bounce_logs.rs) and honours an optional
AWS_SNS_TOPIC_ARN allowlist, but a forged notification with the right topic is still
accepted. Treat everything it writes as attacker-influenced.

The repository layer takes no connection parameter. Each repo module defines its own
get_connection_pool() that pulls from the GLOBAL_APP_STATE Lazy static in
backend/src/lib.rs. That is why services are constructible from anywhere and why nothing is
injectable except through the mockall traits.

Services exist twice: as structs with injected repository traits (used by tests) and as
free functions that construct their own Arc<...RepositoryImpl> inline (used by most
handlers). backend/src/services/campaign_service.rs contains both. Free functions cannot be
tested with mocks; prefer extending the struct methods.

List endpoints have no pagination. GET /api/mails returns every mail row including the full
rendered HTML of each one, and only contacts is namespace-filtered so far. Do not build
anything that assumes those responses are bounded.

Templates are MJML rendered by mrml, with Tera doing variable substitution first
(populate_contact_template in backend/src/utils/contact_lists_functions.rs). Both the
template body and the substituted values come from user input.

## Docs already wrong

None known. DEVELOPER.md, README.md and frontend/README.md were all brought in step with
the code; if you change a documented command or version, update them in the same commit.
