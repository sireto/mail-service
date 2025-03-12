import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from "zod";
import { CampaignSenderDTO } from "@/lib/type";
import { AddCampaignSenderFormSchemaDTO } from "@/lib/type";
import environments from "@/config/environments";

type CampaignSender = z.infer<typeof CampaignSenderDTO>;
type AddCampaignSender = z.infer<typeof AddCampaignSenderFormSchemaDTO>;

export const campaignSenderApi = createApi({
  reducerPath: "campaignSenderApi",
  baseQuery: fetchBaseQuery({
    baseUrl: environments.CAMPAIGN_SENDER_API_BASE_URL,
  }),
  tagTypes: ["CampaignSender"],
  endpoints: (builder) => ({
    // Query to fetch all campaign senders
    getCampaignSenders: builder.query<CampaignSender[], void>({
      query: () => "",
      providesTags: (result) =>
        result
          ? [
              ...result.map(({ id }) => ({
                type: "CampaignSender" as const,
                id,
              })),
              { type: "CampaignSender" },
            ]
          : [{ type: "CampaignSender" }],
    }),
    createCampaignSender: builder.mutation<void, AddCampaignSender>({
      query: (payload) => ({
        url: "",
        method: "POST",
        body: payload,
      }),
      invalidatesTags: ["CampaignSender"],
    }),

    // Mutation to update an existing campaign sender
    updateCampaignSender: builder.mutation<
      void,
      { senderId: string; updatedSender: Partial<CampaignSender> }
    >({
      query: ({ senderId, updatedSender }) => ({
        url: `/${senderId}`,
        method: "PATCH",
        body: updatedSender,
      }),
      invalidatesTags: (result, error, { senderId }) => [
        { type: "CampaignSender", id: senderId },
      ],
    }),

    // Mutation to delete a campaign sender
    deleteCampaignSender: builder.mutation<void, string>({
      query: (senderId) => ({
        url: `/${senderId}`,
        method: "DELETE",
      }),
      invalidatesTags: (result, error, senderId) => [
        { type: "CampaignSender", id: senderId },
      ],
    }),
  }),
});

export const {
  useGetCampaignSendersQuery,
  useCreateCampaignSenderMutation,
  useUpdateCampaignSenderMutation,
  useDeleteCampaignSenderMutation,
} = campaignSenderApi;
