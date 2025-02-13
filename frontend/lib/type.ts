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

// list DTOs...
export const ListDTO = z.object({
    id: z.string(),
    name: z.string(),
    description: z.string(),
    namespace_id: z.string(),
    created_at: z.string(), // ISO string...
    updated_at: z.string(),
});


export const AddListFormSchemaDTO = z.object({
    name: z.string().nonempty().min(2, "Template name must be atleast 2 characters long.").max(50),
    description: z.string().nonempty("Description is required.").min(4, "Description must be atleas 4 characters long.").max(100, "Description must be less than 100 characters long.")
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