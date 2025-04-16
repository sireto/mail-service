export interface AwsCredentials {
  access_key_id: string;
  secret_access_key: string;
  region: string;
}

export type ServerType = "AWS" | "SMTP";

export interface Server {
  id: string;
  server_type: ServerType;
  host?: string;
  port?: number;
  smtp_username?: string;
  smtp_password?: string;
  aws_credentials?: AwsCredentials;
}
