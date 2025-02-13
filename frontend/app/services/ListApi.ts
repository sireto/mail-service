import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    ListDTO, 
    CreateListRequestDTO,
    CreateListResponseDTO,
    UpdateListRequestDTO,
    UpdateListResponseDTO
} from '@/lib/type';

type List = z.infer<typeof ListDTO>;
type CreateListRequest = z.infer<typeof CreateListRequestDTO>;
type CreateListResponse = z.infer<typeof CreateListResponseDTO>;
type UpdateListRequest = z.infer<typeof UpdateListRequestDTO>;
type UpdateListResponse = z.infer<typeof UpdateListResponseDTO>;

// List API slice...
export const listApi = createApi({
    reducerPath: "listApi",
    baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/list"}),
    endpoints: (builder) => ({
        // Query to fetch all lists...
        getLists: builder.query<List[], string>({query: (namespaceId) => `/namespaces/${namespaceId}/list`}),
        createList: builder.mutation<CreateListResponse, CreateListRequest>({
            query: (newList) => ({
                url: "",
                method: "POST",
                body: newList,
            })
        }),
        updateList: builder.mutation<UpdateListResponse, {
            listId: string, 
            namespaceId: string, 
            updatedList: UpdateListRequest
        }>({
            query: ({listId, namespaceId, updatedList}) => ({
                url: `/namespaces/${namespaceId}/list/${listId}`,
                method: "PATCH",
                body: updatedList
            })
        }),
        deleteList: builder.mutation<void, {
            listId: string, 
            namespaceId: string,
        }>({
            query: ({listId, namespaceId}) => ({
                url: `/namespaces/${namespaceId}/list/${listId}`,
                method: "DELETE",
            })
        })
    })
});

export const { 
    useGetListsQuery, 
    useCreateListMutation,
    useUpdateListMutation,
    useDeleteListMutation
} = listApi;