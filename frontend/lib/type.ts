import { z } from "zod";

export const ServerSchema = z.object({
  id: z.string().optional(),
  host: z.string().min(1, "Host is required"),
  namespace_id: z.string().uuid("Invalid namespace ID"),
  port: z.number().min(1).max(65535),
  smtp_password: z.string().min(1, "Password is required"),
  smtp_username: z.string().email("Invalid email"),
  tls_type: z.enum(["NONE", "TLS", "STARTTLS"]),
});

export type Server = z.infer<typeof ServerSchema>;
