import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from "zod";
import { MailDTO } from "@/lib/type";
import environments from "@/config/environments";
import { contactApi } from "./ContactApi";

type Mail = z.infer<typeof MailDTO>;

interface MailDeleteResponse {
  id: string;
  contact_id: string;
  status: string;
}

// Mail API slice...
export const mailApi = createApi({
  reducerPath: "mailApi",
  baseQuery: fetchBaseQuery({ baseUrl: environments.MAIL_API_BASE_URL }),
  tagTypes: ["Mail"],
  endpoints: (builder) => ({
    // Query to fetch all lists...
    getMails: builder.query<
      Mail[],
      {
        campaign_ids?: string[];
        from?: string;
        to?: string;
      }
    >({
      query: ({ campaign_ids, from, to }) => {
        const params = new URLSearchParams();

        if (campaign_ids) {
          params.append("campaign_ids", campaign_ids.join(","));
        }
        if (from) {
          params.append("from", from);
        }
        if (to) {
          params.append("to", to);
        }

        console.warn("THE latest query is ==> ", params.toString());

        // Return the query URL with the query parameters
        return {
          url: `?${params.toString()}`, // Concatenate query parameters to the base URL
        };
      },
      providesTags: (result) =>
        result
          ? result.map((mail) => ({ type: "Mail", id: mail.id }))
          : [{ type: "Mail" }],
    }),
    deleteMail: builder.mutation<MailDeleteResponse, string>({
      query: (mailId) => ({
        url: `/${mailId}`,
        method: "DELETE",
      }),
      invalidatesTags: [{ type: "Mail" }],
      async onQueryStarted(contactId, { dispatch, queryFulfilled }) {
        try {
          const { data } = await queryFulfilled;
          dispatch(
            contactApi.util.invalidateTags([
              { type: "Mail", id: data.contact_id },
            ]),
          );
        } catch (error) {
          console.error("Error deleting mail: ", error);
        }
      },
    }),
    getBouncedMail: builder.query<Mail[], void>({
      query: () => "/bounce",
      providesTags: ["Mail"],
    }),
  }),
});

export const {
  useGetMailsQuery,
  useDeleteMailMutation,
  useGetBouncedMailQuery,
} = mailApi;
