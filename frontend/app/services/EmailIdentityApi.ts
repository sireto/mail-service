import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import environments from "@/config/environments";

export const emailIdentityApi = createApi({
  reducerPath: "emailIdentityApi",
  baseQuery: fetchBaseQuery({
    baseUrl: environments.API_BASE_URL, // Adjust to your API base URL
  }),
  endpoints: (builder) => ({
    validateEmailIdentity: builder.mutation<
      { isValid: boolean; message?: string },
      { email: string; serverId: string }
    >({
      query: (data) => ({
        url: "/campaign-senders/email-identities/validate", // This endpoint will need to be created on your backend
        method: "POST",
        body: data,
      }),
      transformResponse: (response: {
        is_valid: boolean;
        message?: string;
      }) => ({
        isValid: response.is_valid, // Convert snake_case to camelCase
        message: response.message,
      }),
    }),

    getVerifiedIdentities: builder.query<string[], void>({
      query: () => "/campaign-senders/email-identities",
    }),
  }),
});

export const {
  useValidateEmailIdentityMutation,
  useGetVerifiedIdentitiesQuery,
} = emailIdentityApi;
