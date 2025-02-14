import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    TemplateDTO, 
    CreateTemplateResponseDTO, 
    CreateTemplateRequestDTO, 
    UpdateTemplateRequestDTO, 
    UpdateTemplateResponseDTO 
} from '@/lib/type';

type Template = z.infer<typeof TemplateDTO>;
type CreateTemplateRequest = z.infer<typeof CreateTemplateRequestDTO>;
type CreateTemplateResponse = z.infer<typeof CreateTemplateResponseDTO>;
type UpdateTemplateRequest = z.infer<typeof UpdateTemplateRequestDTO>;
type UpdateTemplateResponse = z.infer<typeof UpdateTemplateResponseDTO>;

// Template API slice...
export const templateApi = createApi({
    reducerPath: "templateApi",
    baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/templates"}),
    endpoints: (builder) => ({
        // Query to fetch all templates...
        getTemplates: builder.query<Template[], void>({query: () => ""}),
        createTemplate: builder.mutation<CreateTemplateResponse, CreateTemplateRequest>({
            query: (newTemplate) => ({
                url: "",
                method: "POST",
                body: newTemplate
            })
        }),
        updateTemplate: builder.mutation<UpdateTemplateResponse, {templateId: string , updatedTemplate: UpdateTemplateRequest}>({
            query: ({templateId, updatedTemplate}) => ({
                url: `/${templateId}`,
                method: "PATCH",
                body: updatedTemplate
            })
        }),
        deleteTemplate: builder.mutation<void, string>({
            query: (templateId) => ({
                url: `/${templateId}`,
                method: "DELETE",
            })
        })
    })
});

export const { 
    useGetTemplatesQuery, 
    useCreateTemplateMutation, 
    useUpdateTemplateMutation, 
    useDeleteTemplateMutation 
} = templateApi;