import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from "zod";
import {
  ListDTO,
  CreateListRequestDTO,
  CreateListResponseDTO,
  UpdateListRequestDTO,
  UpdateListResponseDTO,
} from "@/lib/type";
import environments from "@/config/environments";
import { contactApi } from "@/app/services/ContactApi";
import { Contact } from "@/lib/type/contact";

type List = z.infer<typeof ListDTO>;
type CreateListRequest = z.infer<typeof CreateListRequestDTO>;
type CreateListResponse = z.infer<typeof CreateListResponseDTO>;
type UpdateListRequest = z.infer<typeof UpdateListRequestDTO>;
type UpdateListResponse = z.infer<typeof UpdateListResponseDTO>;

// List API slice...
export const listApi = createApi({
  reducerPath: "listApi",
  baseQuery: fetchBaseQuery({ baseUrl: environments.LIST_API_BASE_URL }),
  tagTypes: ["List"],
  endpoints: (builder) => ({
    // Query to fetch all lists...
    getLists: builder.query<List[], string>({
      query: (namespaceId) => `/namespaces/${namespaceId}/list`,
      providesTags: (result) =>
        result
          ? result.map((list) => ({ type: "List", id: list.id }))
          : [{ type: "List" }],
    }),
    getContactsFromLists: builder.query<Contact[], string[]>({
      query: (listIds) => ({
        url: `/contacts`,
        method: "POST",
        body: listIds,
      }),
    }),
    createList: builder.mutation<CreateListResponse, CreateListRequest>({
      query: (newList) => ({
        url: "",
        method: "POST",
        body: newList,
      }),
      invalidatesTags: ["List"],
    }),
    updateList: builder.mutation<
      UpdateListResponse,
      {
        listId: string;
        namespaceId: string;
        updatedList: UpdateListRequest;
      }
    >({
      query: ({ listId, namespaceId, updatedList }) => ({
        url: `/namespaces/${namespaceId}/list/${listId}`,
        method: "PATCH",
        body: updatedList,
      }),
      invalidatesTags: ["List"],
    }),
    deleteList: builder.mutation<
      void,
      {
        listId: string;
        namespaceId: string;
      }
    >({
      query: ({ listId, namespaceId }) => ({
        url: `/namespaces/${namespaceId}/list/${listId}`,
        method: "DELETE",
      }),
      invalidatesTags: ["List"],
    }),
    addContactsToList: builder.mutation<
      void,
      {
        listId: string;
        contacts: { id: string }[]; // Array of objects, each containing the contact's id
      }
    >({
      query: ({ listId, contacts }) => ({
        url: `/addContacts/${listId}`,
        method: "POST",
        body: { contact_ids: contacts.map((contact) => contact.id) },
      }),
      async onQueryStarted(_, { dispatch, queryFulfilled }) {
        await queryFulfilled;
        dispatch(contactApi.util.invalidateTags(["Contact"]));
      },
    }),
    removeContactFromList: builder.mutation<
      void,
      {
        listId: string;
        contacts: { id: string }[];
      }
    >({
      query: ({ listId, contacts }) => ({
        url: `/removeContacts/${listId}`,
        method: "DELETE",
        body: { contact_ids: contacts.map((contact) => contact.id) },
      }),
      async onQueryStarted(_, { dispatch, queryFulfilled }) {
        await queryFulfilled;
        dispatch(contactApi.util.invalidateTags(["Contact"]));
      },
    }),
  }),
});

export const {
  useGetListsQuery,
  useCreateListMutation,
  useUpdateListMutation,
  useDeleteListMutation,
  useAddContactsToListMutation,
  useRemoveContactFromListMutation,
  useGetContactsFromListsQuery,
} = listApi;
