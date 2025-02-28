import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    CampaignSenderDTO,
} from '@/lib/type';
import environments from "@/config/environments";

type CampaignSender = z.infer<typeof CampaignSenderDTO>;


// Campaign API slice...
export const campaignSenderApi = createApi({
    reducerPath: "campaignSenderApi",
    baseQuery: fetchBaseQuery({ baseUrl: environments.CAMPAIGN_SENDER_API_BASE_URL}),
    tagTypes: ["CampaignSender"],
    endpoints: (builder) => ({
        // Query to fetch all lists...
        getCampaignSenders: builder.query<CampaignSender[], void>({
            query: () => "",
            providesTags: (result) => 
                result
                ? result.map((campaignSender) => ({ type: 'CampaignSender', id: campaignSender.id }))
                : [{ type: 'CampaignSender' }]
        }),
    })
});

export const { 
    useGetCampaignSendersQuery
} = campaignSenderApi;