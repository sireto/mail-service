# Mail Service Documentation

## Overview

The Mail Service is a comprehensive email management platform designed to simplify and enhance bulk mailing operations for organizations. This service streamlines the process of creating, managing, and sending email campaigns at scale. It comes equipped with features to design reusable email templates, manage contact lists, and gain valuable insights into email performance through detailed analytics. By leveraging reliable email delivery services like Amazon SES and SMTP server, this platform ensures seamless, scalable, and efficient communication with subscribers.

## Why This Service Is Essential

### Analytics and Optimization
- Gain access to critical metrics such as open rates, click rates, and bounce rates. These insights enable organizations to evaluate the effectiveness of their email campaigns and make data-driven decisions to optimize performance.

### Efficient Template Utilization
- Save time and maintain consistency by creating, editing, and reusing pre-designed email templates tailored for bulk email campaigns.

## Features and Scope

**Email Campaign Management:**

- **Draft, Send, and Schedule:** Easily create campaigns and schedule them for later delivery or send immediately to a targeted audience.
- **Bulk and Individual Emails:** Whether sending to a large audience or just a single contact, the platform accommodates both.

**Contact Management:**

- **Contact Handling:** Add, edit, and organize subscriber lists to ensure accurate targeting for email campaigns.

**Template Management:**

- **Dynamic Template Creation:** Design and customize reusable email templates to match your campaign's tone.
- **HTML and Plain Text Support:** Flexibility to craft visually appealing HTML emails or simple plain text messages.

**Integration with Email Delivery Services:**

- **Seamless Integration:** Reliably send emails through third-party services like Amazon SES, eliminating the need for building or maintaining an SMTP server.

**Analytics and Reporting:**

- **Performance Tracking:** Monitor email campaign performance with metrics like open rates, click-through rates, and delivery statuses.
- **Visual Reports:** Generate detailed, easy-to-interpret visual reports to track campaign trends and outcomes over time.

**Dashboard:**

- **Unified Interface:** Manage all aspects of your campaigns, from templates to analytics, in one user-friendly dashboard.

## Out of Scope

- **SMTP Infrastructure:** This service does have the implementation of SMTP server for email delivery. However, mails sent through the SMTP servers are not tracked such as open, clicks, bounce, and delivery.

## Developers Note

### Version used during the development
- yarn 4.6.0
- rustc 1.83.0 (90b35a623 2024-11-26)
- node v20.14.0
- npm 10.7.0  

### Getting Started
#### Clone the repository
```bash
git@github.com:sireto/mail-service.git

// or

https://github.com/sireto/mail-service.git
```

#### Setup .env file on the root of the project mail-service/
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
NEXT_PUBLIC_NAMESPACE_ID=e3bda5cf-760e-43ea-8e9a-
```

#### Backend
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

4. Run the Rust server: 
    ```bash
    cargo run
    ```

5. Swagger UI:
    ```bash
    http://localhost:8000/swagger-ui/
    ```

Test to see the server is working with Swagger UI.

#### Frontend
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

### Docker
The latest images are available on ghcr at https://github.com/sireto/mail-service.

Or, you can also run the docker-compose.yml file of the root by setting up your own environment variables by simply running:
```bash
docker compose up -d
```

#### Frontend image
The latest frontend image is available on ghcr at ghcr.io/sireto/mail-service-frontend:3cb2544983c38e85581d8c0b330d66a4751e1683

You can visit the above link or download pull the image using the following cmd:
```bash
docker pull ghcr.io/sireto/mail-service-frontend:3cb2544983c38e85581d8c0b330d66a4751e1683
```

#### Backend image
The latest backend image is available on ghcr at ghcr.io/sireto/mail-service-backend:nightly

You can visit the above link or download pull the image using the following cmd:
```bash
docker pull ghcr.io/sireto/mail-service-backend:nightly
```

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
We welcome contributions from developers of all skill levels! To contribute:
1. Fork the Repository
Click the "Fork" button at the top right of this page to create your own copy.

2. Clone Your Fork
    ```bash
    git clone https://github.com/<your-username>/mail-service.git
    cd mail-service
    ```

3. Create a New Branch
    ```bash
    git checkout -b feature/<your-feature-name>
    ```

4. Make Your Changes
    Implement your feature or bug fix. Be sure to follow the existing code style and naming conventions.

5. Test Your Changes
    Run the relevant tests (cargo test, yarn storybook) and ensure everything passes.

6. Commit and Push
    ```bash
    git add .
    git commit -m "Add <your feature/fix>"
    git push origin feature/<your-feature-name>
    ```

7. Open a Pull Request
    Go to the original repository on GitHub and open a pull request from your fork.

## Guidelines
- Keep pull requests small and focused.
- Write clear, descriptive commit messages.
- If your change introduces a new feature or configuration, update the documentation accordingly.
- Discuss major changes with maintainers before starting work.

## License

- [Apache 2.0 License](LICENSE)

By using the Mail Service, organizations can efficiently manage email communication, optimize campaign performance, and achieve their outreach goals without the complexities of managing email infrastructure.