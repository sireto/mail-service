import { z } from "zod";

export const ContactFormSchema = z.object({
  email: z.string().email("Invalid email address"),
  name: z.string().optional(),
  status: z.enum(["Enabled", "Disabled"]).default("Enabled"),
  listId: z.string().optional(),
  attributes: z.string().optional(),
  preconfirm: z.boolean().default(false),
  created_at: z.string().optional(),
  updated_at: z.string().optional(),
});

export interface Contact {
  id: string;
  email: string;
  first_name?: string;
  last_name?: string;
  listId?: string;
  attributes?: string;
  preconfirm?: boolean;
  created_at: string;
  updated_at: string;
}
