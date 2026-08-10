import { z } from "zod";
import { MailDTO } from "../type";

export type Mail = z.infer<typeof MailDTO>;
