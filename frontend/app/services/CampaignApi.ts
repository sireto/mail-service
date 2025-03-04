import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    CampaignDTO,
    CreateCampaignRequestDTO,
    CreateCampaignResponseDTO,
    UpdateCampaignRequestDTO,
    UpdateCampaignResponseDTO
} from '@/lib/type';
import environments from "@/config/environments";
import { listApi } from "./ListApi";

type Campaign = z.infer<typeof CampaignDTO>;
type CreateCampaignRequest = z.infer<typeof CreateCampaignRequestDTO>; 
type CreateCampaignResponse = z.infer<typeof CreateCampaignResponseDTO>;
type UpdateCampaignRequest = z.infer<typeof UpdateCampaignRequestDTO>;
type UpdateCampaignResponse = z.infer<typeof UpdateCampaignResponseDTO>;

// Campaign API slice...
export const campaignApi = createApi({
    reducerPath: "campaignApi",
    baseQuery: fetchBaseQuery({ baseUrl: environments.CAMPAIGN_API_BASE_URL}),
    tagTypes: ["Campaign"],
    endpoints: (builder) => ({
        // Query to fetch all lists...
        getCampaigns: builder.query<Campaign[], void>({
            query: () => "",
            providesTags: (result) => 
                result
                ? result.map((campaign) => ({ type: 'Campaign', id: campaign.id }))
                : [{ type: 'Campaign' }]
        }),
        getCampaignById: builder.query<Campaign, string>({query: (campaignId) => `/${campaignId}`}),
        createCampaign: builder.mutation<CreateCampaignResponse, CreateCampaignRequest>({
            query: (newCampaign) => ({
                url: "",
                method: "POST",
                body: newCampaign
            }),
            invalidatesTags: [{ type: 'Campaign' }]
        }),
        updateCampaign: builder.mutation<UpdateCampaignResponse, {campaignId: string, updatedCampaign: UpdateCampaignRequest}>({
            query: ({campaignId, updatedCampaign}) => ({
                url: `/${campaignId}`,
                method: "PATCH",
                body: updatedCampaign
            }),
            async onQueryStarted(_, { dispatch, queryFulfilled }) {
                try {
                  await queryFulfilled;
                //   dispatch(campaignApi.util.invalidateTags([{ type: "Campaign" }]));
                  dispatch(listApi.util.invalidateTags(["List"])); // also invalidate the lists after updating a campaign...
                } catch (err) {
                  console.error("Error updating campaign:", err);
                }
            },
            invalidatesTags: [{ type: 'Campaign' }]
        }),
        deleteCampaign: builder.mutation<void, string>({
            query: (campaignId) => ({
                url: `/${campaignId}`,
                method: "DELETE",
            }),
            invalidatesTags: [{ type: 'Campaign' }]
        }),
        startCampaign: builder.mutation<void, { campaignId: string, listId: string }>({
            query: ({campaignId, listId}) => ({
                url: `/${campaignId}/send`,
                method: "POST",
                body: { list_id: listId }
            }),
            invalidatesTags: [{ type: 'Campaign' }]
        }),

    })
});

export const { 
    useGetCampaignsQuery,
    useGetCampaignByIdQuery,
    useCreateCampaignMutation,
    useUpdateCampaignMutation,
    useDeleteCampaignMutation,
    useStartCampaignMutation
} = campaignApi;