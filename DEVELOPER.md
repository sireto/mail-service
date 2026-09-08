# Developers Note

## Prerequisites
- **Node.js v22+** — for the frontend. `next` requires >= 20.9.0 and Storybook's Vite 7 requires ^20.19.0 || >= 22.12.0.
- **Yarn v4+** — package manager for the frontend, enabled with `corepack enable`
- **Rust 1.95.0** — pinned in `backend/rust-toolchain.toml`. Cargo.lock's aws-* crates set an MSRV of 1.94.1; on an older toolchain `cargo check` fails with an MSRV error that reads like a dependency problem.
- **Diesel CLI** — for database migrations (`cargo install diesel_cli --no-default-features --features postgres`)
- **Docker & Docker Compose** — optional, used to run Postgres locally
- **PostgreSQL** — required if not using Docker

## Versions this is built and verified against
- yarn 4.6.0
- rustc 1.95.0
- node v22

## Getting Started
### Clone the repository
> Using HTTPS: (recommended)
```bash
https://github.com/sireto/mail-service.git
```

> Using SSH:
```bash
git@github.com:sireto/mail-service.git
```

### Setup .env file on the root of the project mail-service/
```
DATABASE_URL="postgresql://<YOUR_POSTGRES_USERNAME>:<YOUR_POSTGRES_PASSWORD>@localhost:5432/mail_service"
DATABASE_URL_TEST="postgresql://<YOUR_POSTGRES_USERNAME>:<YOUR_POSTGRES_PASSWORD>@localhost:5432/__test_mail_service"
SERVER_ADDRESS="0.0.0.0:8000"

POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=5432
POSTGRES_USER=<YOUR_POSTGRES_USERNAME>
POSTGRES_PASSWORD=<YOUR_POSTGRES_PASSWORD>
POSTGRES_DB=mail_service

ORIGINS=http://localhost:3000

AWS_ACCESS_KEY_ID=<YOUR_AWS_ACCESS_KEY>
AWS_SECRET_ACCESS_KEY=<YOUR_AWS_SECRET_KEY>
AWS_REGION=ap-southeast-1
AWS_SES_CONFIGURATION_SET_NAME="mail-service"

# Optional. When set, the SNS webhook rejects notifications from any other topic. Leave
# unset to accept any topic. Note this is not a substitute for verifying the SNS message
# signature, which is not yet implemented.
AWS_SNS_TOPIC_ARN=

# Required. Encrypts SMTP passwords and AWS credentials at rest. Generate once with
# `openssl rand -base64 48`. Keep it stable and backed up: stored credentials cannot be
# decrypted with a different value, and the service refuses to start without it.
DATA_ENCRYPTION_KEY=

NEXT_PUBLIC_BASE_URL=http://localhost:8000/api
NEXT_PUBLIC_NAMESPACE_ID=e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82
```

### Backend
1. Navigate to the backend directory:
    ```
    cd backend
    ```

2. Install dependencies:
    ```bash
    cargo build
    ```

3. Start the Postgres Database
    ```bash
    docker compose -f docker-compose.yml up
    ```
    >**NOTE**: You need to update the environment variables placeholder in docker-compose.yml file _(if there are any)_

4. Install Diesel CLI
    You’ll need the Diesel CLI to run database migrations locally. Install it with:
    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```

5. Run the Rust server: 
    ```bash
    cargo run
    ```

6. Swagger UI:
    ```bash
    http://localhost:8000/swagger-ui/
    ```

Test to see the server is working with Swagger UI.

### Frontend
1. Navigate to the frontend directory:
    ```
    cd frontend
    ```

2. Install dependencies:
    ```bash
    yarn install
    ```

4. Run the development server:
    ```bash
    yarn dev
    ```

Check out the frontend server on `http://localhost:3000/`

### Database Migrations
Mail-service uses the diesel-ORM for migrations and all the pending migrations starts when the server starts so you won't need to run the migration on your own. However, if you intend to tweak your local db you might want to run some db migrations cmd.

Some useful commands:
1. Generate a new migration file
    ```bash
    diesel migration generate <migration_name>
    ```

2. Run a migration file
    ```bash
    diesel migration run
    ```

3. Rollback
    ```bash
    diesel migration revert
    ```

4. Redo
    ```bash
    diesel migration redo
    ```

If you want to know more about diesel, you can see its documentation [here](https://diesel.rs/guides/getting-started)

### Tests and checks

CI runs exactly these. Run them locally before pushing, or install the pre-commit hook
with `./install-hooks.sh`, which runs the backend and frontend gates for whichever
directory has staged changes.

Backend, from `backend/`:
```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --no-default-features   # the shape the release image builds
```

`--no-default-features` drops the `mocks` feature. That feature compiles the mockall
`Mock*` types the integration tests need; it is on by default so plain `cargo test` works,
and off in the Dockerfile so a test framework does not ship in the production binary.

`cargo test` needs no database: every test uses mocks, and `DATA_ENCRYPTION_KEY` can be any
long string.

The one exception is `tests/encryption_db.rs`, which checks that credentials are ciphertext
in the column and plaintext to the caller. That property cannot be checked with a mock, so
it is `#[ignore]`d and run explicitly:

```bash
createdb ms_crypto_check
for d in migrations/*/; do psql -q -f "$d/up.sql" ms_crypto_check; done
DATABASE_URL=postgres://localhost/ms_crypto_check \
  DATA_ENCRYPTION_KEY=any-long-random-string-at-least-32-chars \
  cargo test --test encryption_db -- --ignored --test-threads=1
```

### Credential encryption

SMTP passwords and AWS credentials are encrypted at rest with AES-256-GCM. The key is
derived (HKDF-SHA256) from `DATA_ENCRYPTION_KEY`, which is a seed rather than raw key
material, so any sufficiently long random string works:

```bash
openssl rand -base64 48
```

Three things to know before deploying:

1. The service refuses to start without it. `main.rs` runs an encrypt/decrypt self-check
   first, so a container missing the key fails immediately instead of coming up and then
   failing on the first send.
2. **Keep it stable and back it up.** The key is not stored anywhere but your environment,
   and every stored credential is unrecoverable without it. Rotating it requires
   re-encrypting existing rows; there is no tooling for that yet.
3. Existing plaintext rows keep working. Encrypted values carry a `v1:` prefix; anything
   without it is read through unchanged and is encrypted the next time that server is
   saved.

Frontend, from `frontend/`:
```bash
yarn lint
yarn build
```

There are no automated frontend tests. `yarn storybook` opens the component explorer,
which holds stories, not assertions. `yarn format:check` is not yet a CI gate because
prettier has never been run across the repository — run `yarn format` and commit that on
its own first.

### Project Structure
```bash
sireto-mail-service/
├── README.md
├── docker-compose-qa.yml
├── docker-compose.yml
├── LICENSE
├── AGENTS.md        # agent-facing notes: couplings, traps, the commands CI runs
├── backend/
│   ├── Cargo.toml
│   ├── rust-toolchain.toml  # pins the Rust version; see Prerequisites
│   ├── Dockerfile
│   ├── migrations/          # Diesel migrations, applied automatically at startup
│   ├── src/
│   │   ├── handlers/        # axum handlers, one module per resource
│   │   ├── routes/          # Router construction; also wires services into Extension
│   │   ├── services/        # business logic
│   │   ├── repositories/    # Diesel queries behind mockable traits
│   │   ├── models/          # Diesel models and request/response DTOs
│   │   ├── servers/         # the servers resource, self-contained across all layers
│   │   └── utils/
│   └── tests/               # integration tests, all mock-backed
├── frontend/
│   ├── README.md
│   ├── components.json
│   ├── Dockerfile
│   ├── Dockerfile.storybook
│   ├── eslint.config.mjs
│   ├── next.config.ts
│   ├── package.json
│   ├── postcss.config.cjs
│   ├── tailwind.config.ts
│   ├── tsconfig.json
│   ├── yarn.lock
│   ├── .dockerignore
│   ├── .gitignore
│   ├── .yarnrc.yml
│   └── app/
```

## Known gaps

**The API has no authentication of any kind.** Every endpoint under `/api` is reachable by
anyone who can open a connection to the service, including the ones that read contacts and
manage sending credentials. Do not deploy this to a public address without an
authenticating proxy in front of it.

Also outstanding:
- Namespaces are not a security boundary. The caller asserts which namespace it is acting
  in and nothing verifies that claim, so scoping is for correctness, not access control.
- The SNS webhook does not verify message signatures, so delivery and open events can be
  forged. `AWS_SNS_TOPIC_ARN` narrows this but does not close it.
- Only `contacts` is namespace-filtered. The other list endpoints return rows across all
  namespaces.

## Contribution
Want to contribute? Please check out our [CONTRIBUTING.md](https://github.com/sireto/mail-service/blob/develop/CONTRIBUTING.md) guide for instructions on how to get started.