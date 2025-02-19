import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    MailDTO
} from '@/lib/type';

type Mail = z.infer<typeof MailDTO>;

// Mail API slice...
export const mailApi = createApi({
    reducerPath: "mailApi",
    baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/mails"}),
    endpoints: (builder) => ({
        // Query to fetch all lists...
        getMails: builder.query<Mail[], { 
            campaign_ids?: string[], 
            from?: string, 
            to?: string }>({query: ({ campaign_ids, from, to }) => {
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
                    url: `?${params.toString()}`,  // Concatenate query parameters to the base URL
                };
        }}),
    })
});

export const { 
    useGetMailsQuery,
} = mailApi;