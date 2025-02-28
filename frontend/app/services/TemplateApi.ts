import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    TemplateDTO, 
    CreateTemplateResponseDTO, 
    CreateTemplateRequestDTO, 
    UpdateTemplateRequestDTO, 
    UpdateTemplateResponseDTO ,
    SendTransactionalMailRequestDTO
} from '@/lib/type';
import environments from "@/config/environments";

type Template = z.infer<typeof TemplateDTO>;
type CreateTemplateRequest = z.infer<typeof CreateTemplateRequestDTO>;
type CreateTemplateResponse = z.infer<typeof CreateTemplateResponseDTO>;
type UpdateTemplateRequest = z.infer<typeof UpdateTemplateRequestDTO>;
type UpdateTemplateResponse = z.infer<typeof UpdateTemplateResponseDTO>;
type SendTransactionalMailRequest = z.infer<typeof SendTransactionalMailRequestDTO>;

// Template API slice...
export const templateApi = createApi({
    reducerPath: "templateApi",
    baseQuery: fetchBaseQuery({ baseUrl: environments.TEMPLATE_API_BASE_URL}),
    tagTypes: ['Template'],
    endpoints: (builder) => ({
        // Query to fetch all templates...
        getTemplates: builder.query<Template[], void>({
            query: () => "",
            providesTags: (result) => 
                result
                ? result.map((template) => ({ type: 'Template', id: template.id }))
                : [{ type: 'Template' }]
        }),
        getTemplateById: builder.query<Template, string>({
            query: (templateId) => `/${templateId}`,
            providesTags: (result, error, templateId) => [{ type: 'Template', id: templateId }]
        }),
        createTemplate: builder.mutation<CreateTemplateResponse, CreateTemplateRequest>({
            query: (newTemplate) => ({
                url: "",
                method: "POST",
                body: newTemplate
            }),
            invalidatesTags: ['Template']
        }),
        updateTemplate: builder.mutation<UpdateTemplateResponse, {templateId: string , updatedTemplate: UpdateTemplateRequest}>({
            query: ({templateId, updatedTemplate}) => ({
                url: `/${templateId}`,
                method: "PATCH",
                body: updatedTemplate
            }),
            invalidatesTags: ['Template']
        }),
        deleteTemplate: builder.mutation<void, string>({
            query: (templateId) => ({
                url: `/${templateId}`,
                method: "DELETE",
            }),
            invalidatesTags: ['Template']
        }),
        sendTemplatedEmail: builder.mutation<void, { templateId: string, payload: SendTransactionalMailRequest}>({
            query: ({templateId, payload}) => ({
                url: `/${templateId}/send`,
                method: "POST",
                body: payload
            }),
        })
    })
});

export const { 
    useGetTemplatesQuery, 
    useGetTemplateByIdQuery,
    useCreateTemplateMutation, 
    useUpdateTemplateMutation, 
    useDeleteTemplateMutation,
    useSendTemplatedEmailMutation
} = templateApi;