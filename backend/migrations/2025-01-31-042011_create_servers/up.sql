-- Your SQL goes here
CREATE TABLE "servers" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "host" VARCHAR NOT NULL,
    "smtp_username" VARCHAR NOT NULL,
    "smtp_password" VARCHAR NOT NULL,
    "namespace_id" UUID REFERENCES namespaces(id) ON DELETE CASCADE,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


/*id 
host 
smtp_username
smtp_password
namespace_id
created_at
updated_at
*/