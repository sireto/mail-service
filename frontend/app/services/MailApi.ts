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
        getMails: builder.query<Mail[], void>({query: () => ""}),
    })
});

export const { 
    useGetMailsQuery,
} = mailApi;