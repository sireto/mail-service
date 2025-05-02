# Mail Service Documentation

## Overview

The Mail Service is a comprehensive email management platform designed to simplify and enhance bulk mailing operations for organizations. This service streamlines the process of creating, managing, and sending email campaigns at scale. It comes equipped with features to design reusable email templates, manage contact lists, and gain valuable insights into email performance through detailed analytics. By leveraging reliable email delivery services like Amazon SES and SMTP server, this platform ensures seamless, scalable, and efficient communication with subscribers.

![mail_service_img](https://github.com/user-attachments/assets/7e959e14-f2c9-4733-ad53-344a4b288b97)

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

**Efficient Template Utilization**
- Save time and maintain consistency by creating, editing, and reusing pre-designed email templates tailored for bulk email campaigns.

## Limitations

- **SMTP Infrastructure:** This service includes the an SMTP server implementation. However, emails sent via the SMTP server are not tracked for opens, clicks, bounces, or delivery status.

## Running with Docker
The latest images are available at: [here](https://github.com/sireto/mail-service).

Or, you can also run the docker-compose.yml file of the root by setting up your own environment variables by simply running:
```bash
docker compose up -d
```

### Frontend image
The latest frontend image is available at: [here](ghcr.io/sireto/mail-service-frontend:3cb2544983c38e85581d8c0b330d66a4751e1683)

You can visit the above link or download pull the image using the following cmd:
```bash
docker pull ghcr.io/sireto/mail-service-frontend:3cb2544983c38e85581d8c0b330d66a4751e1683
```

### Backend image
The latest backend image is available at: [here](ghcr.io/sireto/mail-service-backend:nightly)

You can visit the above link or download pull the image using the following cmd:
```bash
docker pull ghcr.io/sireto/mail-service-backend:nightly
```

## Developers
Mail-service is free and open-source software licensed under Apache 2.0 License. If you're interested in contributing, please refer to the [Developer README](https://github.com/sireto/mail-service/blob/develop/DEVELOPER.md) for setup instructions and [Contribution README](https://github.com/sireto/mail-service/blob/develop/CONTRIBUTING.md) contribution guidelines.

## License

- [Apache 2.0 License](LICENSE)

By using the Mail Service, organizations can efficiently manage email communication, optimize campaign performance, and achieve their outreach goals without the complexities of managing email infrastructure.
