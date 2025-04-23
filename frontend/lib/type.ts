import { z } from "zod";

export const TemplateDTO = z.object({
  id: z.string(),
  name: z.string(),
  content_html: z.string(),
  namespace_id: z.string(),
  content_plaintext: z.string(),
  template_data: z.string(),
  created_at: z.string(), // ISO string...
  updated_at: z.string(),
});

export const AddTemplateFormSchemaDTO = z.object({
  name: z
    .string()
    .nonempty()
    .min(2, "Template name must be atleast 2 characters long.")
    .max(50),
  raw_mjml_content: z
    .string()
    .nonempty()
    .refine(
      (content) => {
        return content.includes("<mjml>") && content.includes("<mj-body");
      },
      {
        message: "MJML content must be wrapped in the <mjml> tags.",
      }
    ),
});

export const CreateTemplateRequestDTO = z.object({
  name: z.string(),
  content_html: z.string(),
  namespace_id: z.string(),
  content_plaintext: z.string(),
  template_data: z.string(),
});

export const CreateTemplateResponseDTO = z.object({
  id: z.string(),
  name: z.string(),
  created_at: z.string(),
});

export const UpdateTemplateRequestDTO = z.object({
  name: z.string(),
  content_html: z.string(),
  content_plaintext: z.string(),
  template_data: z.string(),
});

export const UpdateTemplateResponseDTO = z.object({
  id: z.string(),
  name: z.string(),
  updated_at: z.string(),
});

export const ServerSchema = z.object({
  id: z.string().uuid().optional(),
  active: z.boolean().default(true),
  host: z.string(),
  namespace_id: z.string().uuid(),
  port: z.number().int().min(1).max(65535),
  smtp_username: z.string(),
  smtp_password: z.string(),
  tls_type: z.enum(["STARTTLS", "SSL/TLS", "NONE"]),
  server_type: z.enum(["SMTP", "AWS"]),
  default_from_email: z.string().email("Invalid email format"),
  aws_credentials: z.object({
    access_key_id: z.string(),
    secret_access_key: z.string(),
    region: z.string(),
    session_token: z.string().nullable(),
  }),
  created_at: z.string().optional(),
  updated_at: z.string().optional(),
});

export type Server = z.infer<typeof ServerSchema>;

export const ListDTO = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string(),
  namespace_id: z.string(),
  created_at: z.string(), // ISO string...
  updated_at: z.string(),
});

export const AddListFormSchemaDTO = z.object({
  name: z
    .string()
    .nonempty()
    .min(2, "Template name must be atleast 2 characters long.")
    .max(50),
  description: z
    .string()
    .nonempty("Description is required.")
    .min(4, "Description must be atleas 4 characters long.")
    .max(100, "Description must be less than 100 characters long."),
});

export const CreateListRequestDTO = z.object({
  name: z.string(),
  description: z.string(),
  namespace_id: z.string(),
});

export const CreateListResponseDTO = z.object({
  id: z.string(),
  name: z.string(),
  created_at: z.string(),
});

export const UpdateListRequestDTO = z.object({
  name: z.string(),
  description: z.string(),
});

export const UpdateListResponseDTO = z.object({
  id: z.string(),
  name: z.string(),
  updated_at: z.string(),
});

export const Contact = z.object({
  id: z.string(),
  email: z.string(),
  first_name: z.string(),
  last_name: z.string(),
  listId: z.string(),
  attributes: z.string(),
  preconfirm: z.boolean(),
  created_at: z.string(),
  updated_at: z.string(),
});

export interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

// campaigns DTOs...
export const CampaignDTO = z.object({
  id: z.string(),
  campaign_name: z.string(),
  campaign_senders: z.string(),
  created_at: z.string(), // ISO string...
  updated_at: z.string(),
  namespace_id: z.string(),
  template_id: z.string(),
  status: z.string(),
  scheduled_at: z.string(),
  lists: z.array(ListDTO),
});

export const AddCampaignFormSchemaDTO = z.object({
  campaign_name: z.string(),
  campaign_senders: z.string(),
  namespace_id: z.string(),
  template_id: z.string(),
  list_ids: z.array(z.string()),
});

export const CreateCampaignRequestDTO = z.object({
  campaign_name: z.string(),
  campaign_senders: z.string(),
  namespace_id: z.string(),
  template_id: z.string(),
  status: z.string(),
  scheduled_at: z.string(),
  list_ids: z.array(z.string()),
});

export const CreateCampaignResponseDTO = z.object({
  id: z.string(),
  campaign_name: z.string(),
  campaign_senders: z.string(),
  created_at: z.string(), // ISO string...
  updated_at: z.string(),
  namespace_id: z.string(),
  template_id: z.string(),
  status: z.string(),
  scheduled_at: z.string(),
});

export const UpdateCampaignRequestDTO = z.object({
  campaign_name: z.string(),
  campaign_senders: z.string(),
  template_id: z.string(),
  status: z.string(),
  scheduled_at: z.string(),
  list_ids: z.array(z.string()),
});

export const UpdateCampaignResponseDTO = z.object({
  id: z.string(),
  campaign_name: z.string(),
  campaign_senders: z.string(),
  template_id: z.string(),
  status: z.string(),
  scheduled_at: z.string(),
  created_at: z.string(),
  updated_at: z.string(),
  lists: z.array(ListDTO),
});

// mails DTOs...
export const MailDTO = z.object({
  id: z.string(),
  campaign_id: z.string(),
  contact_id: z.string(),
  mail_message: z.string(), // ISO string...
  sent_at: z.string(),
  template_id: z.string(),
  open: z.boolean(),
  clicks: z.number(),
  status: z.string(),
  status_reason: z.string().nullable(),
});

// send transactional mail request DTO...
export const SendTransactionalMailRequestDTO = z.object({
  bcc: z.string().nullable(),
  cc: z.string().nullable(),
  from: z.string(),
  receiver: z.string(),
  subject: z.string(),
  template_data: z.string(),
});

export const CampaignSenderDTO = z.object({
  id: z.string(),
  server_id: z.string(),
  from_name: z.string(),
  from_email: z.string(),
  created_at: z.string(),
  updated_at: z.string(),
});

export const EditCampaignSenderFormSchemaDTO = z.object({
  from_name: z.string().min(1, "Name is required"),
  from_email: z.string().email("Invalid email format"),
});

export const AddCampaignSenderFormSchemaDTO = z.object({
  from_name: z.string().min(1, "Name is required"),
  from_email: z.string().email("Invalid email format"),
  server_id: z.string(),
});
