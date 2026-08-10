# Mail Service Documentation

## Overview

The Mail Service is a comprehensive email management platform designed to simplify and enhance bulk mailing operations for organizations. This service streamlines the process of creating, managing, and sending email campaigns at scale. It comes equipped with features to design reusable email templates, manage contact lists, and gain valuable insights into email performance through detailed analytics. By leveraging reliable email delivery services like Amazon SES and SMTP server, this platform ensures seamless, scalable, and efficient communication with subscribers.

![mail_service_img](https://github.com/user-attachments/assets/2a71f828-a17e-4114-b1ee-350f8c409b93)

## Running with Docker
The latest images are available at: [here](https://github.com/sireto/mail-service).

Or, you can also run the docker-compose.yml file of the root by setting up your own environment variables by simply running:
```bash
docker compose up -d
```

Here is the sample docker-compose.yml file:
```bash
services:
  postgres:
    image: postgres:latest
    container_name: postgres_mailservice
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: mail_service
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  api:
    image: ghcr.io/sireto/mail-service-backend:nightly
    ports:
      - "8000:8000"
    depends_on:
      - postgres
    environment:
      DATABASE_URL: postgresql://postgres:postgres@postgres_mailservice:5432/mail_service
      AWS_ACCESS_KEY_ID: <AWS_ACCESS_KEY_ID>
      AWS_SECRET_ACCESS_KEY: <AWS_SECRET_ACCESS_KEY>
      AWS_SES_CONFIGURATION_SET_NAME: <AWS_SES_CONFIGURATION_SET_NAME>
      ORIGINS: http://localhost:3000,http://172.31.0.6:3600

  webapp:
    build:
      context: ./frontend
      dockerfile: Dockerfile
      args:
        NEXT_PUBLIC_BASE_URL: http://localhost:8000/api
    depends_on:
      - postgres
      - api
    environment:
      NEXT_PUBLIC_NAMESPACE_ID: e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82

    ports:
      - "3000:3000"

volumes:
  postgres_data:
```

## Features and Scope
- **Email Campaign Management**
- **Contact Management**
- **Template Management**
- **Integration with Email Delivery Services**
- **Analytics and Reporting**
- **Dashboard**
- **Efficient Template Utilization**

## Limitations

- **SMTP Infrastructure:** This service includes the SMTP server implementation. However, emails sent via the SMTP server are not tracked for opens, clicks, bounces, or delivery status.

## Developers
Mail-service is free and open-source software licensed under Apache 2.0 License. If you're interested in contributing, please refer to the [Developer README](https://github.com/sireto/mail-service/blob/develop/DEVELOPER.md) for setup instructions and [Contribution README](https://github.com/sireto/mail-service/blob/develop/CONTRIBUTING.md) contribution guidelines.

## License

- [Apache 2.0 License](LICENSE)

By using the Mail Service, organizations can efficiently manage email communication, optimize campaign performance, and achieve their outreach goals without the complexities of managing email infrastructure.
