import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { Server } from "@/lib/type";

export const ServerApi = createApi({
  reducerPath: "serverApi",
  baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/" }),
  tagTypes: ["Server"],
  endpoints: (builder) => ({
    getServers: builder.query<Server[], void>({
      query: () => "servers",
      providesTags: ["Server"],
    }),
    createServer: builder.mutation<Server, Omit<Server, "id">>({
      query: (newServer) => ({
        url: "servers",
        method: "POST",
        body: newServer,
      }),
      invalidatesTags: ["Server"],
    }),
    updateServer: builder.mutation<Server, Partial<Server>>({
      query: ({ id, ...patch }) => ({
        url: `servers/${id}`,
        method: "PATCH",
        body: patch,
      }),
      invalidatesTags: ["Server"],
    }),
    deleteServer: builder.mutation<void, string>({
      query: (id) => ({
        url: `servers/${id}`,
        method: "DELETE",
      }),
      invalidatesTags: ["Server"],
    }),
  }),
});

export const {
  useGetServersQuery,
  useCreateServerMutation,
  useUpdateServerMutation,
  useDeleteServerMutation,
} = ServerApi;
