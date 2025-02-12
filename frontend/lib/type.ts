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
    name: z.string().nonempty().min(2, "Template name must be atleast 2 characters long.").max(50),
    raw_mjml_content: z.string().nonempty().refine((content) => {
        return content.includes("<mjml>") && content.includes("<mj-body");
    }, {
        message: "MJML content must be wrapped in the <mjml> tags."
    }),
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
    template_data: z.string()
});

export const UpdateTemplateResponseDTO = z.object({
    id: z.string(),
    name: z.string(),
    updated_at: z.string(),
});