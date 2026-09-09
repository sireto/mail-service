import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { Contact } from "@/lib/type/contact";
import { MailDTO } from "@/lib/type";
import { NAMESPACE_ID } from "@/config/namespace";
import {
  appendPageParams,
  type Page,
  type PageParams,
} from "@/lib/type/pagination";
import { z } from "zod";

type Mail = z.infer<typeof MailDTO>;

export const contactApi = createApi({
  reducerPath: "contactApi",
  baseQuery: fetchBaseQuery({ baseUrl: process.env.NEXT_PUBLIC_BASE_URL }),
  tagTypes: ["Contact", "Mail"],
  endpoints: (builder) => ({
    getContacts: builder.query<
      Page<Contact>,
      { list_id?: string; search?: string } & PageParams
    >({
      query: ({ list_id, search, limit, offset }) => {
        const params = new URLSearchParams();
        params.append("namespace_id", NAMESPACE_ID);
        if (list_id) {
          params.append("list_id", list_id);
        }
        if (search) {
          params.append("search", search);
        }
        appendPageParams(params, { limit, offset });
        return { url: `contacts?${params.toString()}` };
      },
      providesTags: ["Contact"],
    }),
    getContactById: builder.query<Contact, string>({
      query: (contactId) => `contacts/${contactId}`,
      providesTags: (result, error, contactId) => [
        { type: "Contact", id: contactId },
      ],
    }),
    addContact: builder.mutation<Contact, Omit<Contact, "id">>({
      query: (newContact) => ({
        url: "contacts",
        method: "POST",
        body: [{ namespace_id: NAMESPACE_ID, ...newContact }],
      }),
      invalidatesTags: ["Contact"],
    }),
    updateContact: builder.mutation<
      Contact,
      { id: string; data: Partial<Omit<Contact, "id">> }
    >({
      query: ({ id, data }) => ({
        url: `contacts/${id}`,
        method: "PATCH",
        body: data,
      }),
      invalidatesTags: ["Contact"],
    }),
    deleteContact: builder.mutation<{ success: boolean }, string>({
      query: (id) => ({
        url: `contacts/${id}`,
        method: "DELETE",
      }),
      invalidatesTags: ["Contact"],
    }),
    checkEmail: builder.query<{ exists: boolean }, string>({
      query: (email) => ({
        url: `contacts/check-email`,
        // An address is only unique within a namespace, so the check is scoped too.
        params: { email, namespace_id: NAMESPACE_ID },
      }),
    }),
    importContacts: builder.mutation<
      { success: boolean; imported: number; errors?: string[] },
      FormData
    >({
      query: (formData) => {
        // The import endpoint requires it; append here so no caller has to remember.
        if (!formData.has("namespace_id")) {
          formData.append("namespace_id", NAMESPACE_ID);
        }
        return { url: "contacts/import", method: "POST", body: formData };
      },
      invalidatesTags: ["Contact"],
    }),
    getMailsForContact: builder.query<Mail[], string>({
      query: (contactId) => `contacts/${contactId}/mails`,
      providesTags: (result, error, contactId) => {
        return [{ type: "Mail", id: contactId }];
      },
    }),
  }),
});

// Export hooks for usage in components
export const {
  useGetContactsQuery,
  useLazyGetContactsQuery,
  useAddContactMutation,
  useGetContactByIdQuery,
  useUpdateContactMutation,
  useDeleteContactMutation,
  useCheckEmailQuery,
  useImportContactsMutation,
  useLazyCheckEmailQuery,
  useGetMailsForContactQuery,
} = contactApi;
