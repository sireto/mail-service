import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { z } from 'zod';
import { 
    MailDTO
} from '@/lib/type';
import environments from "@/config/environments";

type Mail = z.infer<typeof MailDTO>;

// Mail API slice...
export const mailApi = createApi({
    reducerPath: "mailApi",
    baseQuery: fetchBaseQuery({ baseUrl: environments.MAIL_API_BASE_URL}),
    tagTypes: ["Mail"],
    endpoints: (builder) => ({
        // Query to fetch all lists...
        getMails: builder.query<Mail[], { 
            campaign_ids?: string[], 
            from?: string, 
            to?: string }>({
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
                        url: `?${params.toString()}`,  // Concatenate query parameters to the base URL
                    };
                },
                providesTags: (result) => 
                    result
                    ? result.map((mail) => ({ type: 'Mail', id: mail.id }))
                    : [{ type: 'Mail' }]
        }),
        deleteMail: builder.mutation<void, string>({
            query: (mailId) => ({
                url: `/${mailId}`,
                method: "DELETE",
            }),
            invalidatesTags: [{ type: 'Mail' }]
        })
    })
});

export const { 
    useGetMailsQuery,
    useDeleteMailMutation
} = mailApi;