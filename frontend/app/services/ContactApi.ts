import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { Contact } from "@/lib/type/contact";

export const contactApi = createApi({
  reducerPath: "contactApi",
  baseQuery: fetchBaseQuery({ baseUrl: process.env.NEXT_PUBLIC_BASE_URL }),
  tagTypes: ["Contact"],
  endpoints: (builder) => ({
    getContacts: builder.query<Contact[], void>({
      query: () => "contacts",
      providesTags: ["Contact"],
    }),
    addContact: builder.mutation<Contact, Omit<Contact, "id">>({
      query: (newContact) => ({
        url: "contacts",
        method: "POST",
        body: [newContact],
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
        params: { email },
      }),
    }),
    importContacts: builder.mutation<
      { success: boolean; imported: number; errors?: string[] },
      FormData
    >({
      query: (formData) => ({
        url: "contacts/import",
        method: "POST",
        body: formData,
      }),
      invalidatesTags: ["Contact"],
    }),
  }),
});

// Export hooks for usage in components
export const {
  useGetContactsQuery,
  useAddContactMutation,
  useUpdateContactMutation,
  useDeleteContactMutation,
  useCheckEmailQuery,
  useImportContactsMutation,
  useLazyCheckEmailQuery,
} = contactApi;
