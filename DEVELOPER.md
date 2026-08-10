# Developers Note

## Prerequisites
- **Node.js v20+** — for the frontend
- **Yarn v4+** — package manager for the frontend
- **Rust & Cargo** — for building and running the backend
- **Diesel CLI** — for database migrations (`cargo install diesel_cli --no-default-features --features postgres`)
- **Docker & Docker Compose** — optional, used to run Postgres locally
- **PostgreSQL** — required if not using Docker

## Version used during the development
- yarn 4.6.0
- rustc 1.83.0 (90b35a623 2024-11-26)
- node v20.14.0
- npm 10.7.0  

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
AWS_SES_CONFIGURATION_SET_NAME="mail-service"

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

### Tests
Backend tests:
1. Navigate to the backend directory:
    ```
    cd backend
    ``` 
2. Run the following cmd:
    ```
    cargo test
    ```

Frontend tests:
1. Navigate to the frontend directory:
    ```
    cd frontend
    ``` 
2. Run the following cmd:
    ```
    yarn storybook
    ```

### Project Structure
```bash
sireto-mail-service/
├── README.md
├── docker-compose-qa.yml
├── docker-compose.yml
├── LICENSE
├── backend/
│   ├── README.md
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── docker-compose.yml
│   ├── Dockerfile
│   ├── migrations/  # contains Diesel database migration folders
│   ├── src/
│   ├── tests/
│   └── .cargo/
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

## Contribution
Want to contribute? Please check out our [CONTRIBUTING.md](https://github.com/sireto/mail-service/blob/develop/CONTRIBUTING.md) guide for instructions on how to get started.