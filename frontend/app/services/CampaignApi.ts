import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    CampaignDTO,
    CreateCampaignRequestDTO,
    CreateCampaignResponseDTO
} from '@/lib/type';

type Campaign = z.infer<typeof CampaignDTO>;
type CreateCampaignRequest = z.infer<typeof CreateCampaignRequestDTO>; 
type CreateCampaignResponse = z.infer<typeof CreateCampaignResponseDTO>;


// Campaign API slice...
export const campaignApi = createApi({
    reducerPath: "campaignApi",
    baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/campaigns"}),
    endpoints: (builder) => ({
        // Query to fetch all lists...
        getCampaigns: builder.query<Campaign[], void>({query: () => ""}),
        createCampaign: builder.mutation<CreateCampaignResponse, CreateCampaignRequest>({
            query: (newCampaign) => ({
                url: "",
                method: "POST",
                body: newCampaign
            })
        }),
        deleteCampaign: builder.mutation<void, string>({
            query: (campaignId) => ({
                url: `/${campaignId}`,
                method: "DELETE",
            })
        }),
        startCampaign: builder.mutation<void, { campaignId: string, listId: string }>({
            query: ({campaignId, listId}) => ({
                url: `/${campaignId}/send`,
                method: "POST",
                body: { list_id: listId }
            })
        }),

    })
});

export const { 
    useGetCampaignsQuery,
    useCreateCampaignMutation,
    useDeleteCampaignMutation,
    useStartCampaignMutation
} = campaignApi;