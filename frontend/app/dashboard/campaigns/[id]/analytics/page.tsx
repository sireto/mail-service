'use client';

import { useDeleteMailMutation, useGetMailsQuery } from '@/app/services/MailApi';
import DataTable from '@/components/DataTable';
import columns from './_columns';
import React, { useState, useEffect, useRef } from 'react';
import { useParams } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form';
import { Search } from 'lucide-react';
import { Form } from '@/components/ui/form';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { MultiSelect } from '@/components/common/Multiselect';
import { useGetCampaignsQuery } from '@/app/services/CampaignApi';
import DatePicker from '@/components/common/DatePicker';
import { setToEndOfDay } from '@/lib/utils';

const SearchCampaignAnalyticsDTO = z.object({
    campaigns: z.array(z.string().nonempty("At least one campaign is needed")).min(1, "Select at least one campaign"),
    from: z.union([z.string().nonempty("From date is required"), z.date()]),  // Allow both string and Date
    to: z.union([z.string().nonempty("To date is required"), z.date()])
});

interface ICampaignData {
    value: string;
    label: string;
};

const Page = () => {
    const { id } = useParams(); // Get campaign ID from URL parameters
    
    // const { data: campaigns, error: fetchError, isLoading: isFetching } = useGetCampaignsQuery();
    // let [campaignList, setCampaignList] = useState<ICampaignData[] | []>([]);
    const [ deleteMail, { isLoading: isDeleting, error: deletionError }] = useDeleteMailMutation();

    const today = new Date();
    const oneWeekAgo = new Date(today);
    oneWeekAgo.setDate(today.getDate() - 7);

    const form = useForm<z.infer<typeof SearchCampaignAnalyticsDTO>>({
        resolver: zodResolver(SearchCampaignAnalyticsDTO),
        defaultValues: {
            campaigns: typeof id === 'string' ? [id] : [],
            from: oneWeekAgo,
            to: today,
        }
    });

    const [searchParams, setSearchParams] = useState({
        campaign_ids: typeof id === 'string' ? [id] : [],
        from: oneWeekAgo.toISOString(),
        to: today.toISOString(),
    });

    const datesChanged = form.formState.dirtyFields.from || form.formState.dirtyFields.to;


    // Adjust the mail query to trigger based on campaign ID (from URL params) and form inputs
    const { data: mails, isLoading, error } = useGetMailsQuery(
        searchParams, 
        // skip when the form dates are not changed here...
        { skip: !form.formState.isValid || (form.formState.isDirty && !datesChanged)  }
    );

    // useEffect(() => {
    //     if (campaigns) {
    //         setCampaignList(campaigns?.map((campaign) => ({
    //             value: campaign.id,
    //             label: campaign.campaign_name,
    //         })));
    //     }
    // }, [campaigns]);

    // useEffect(() => {
    //     if (campaignList.length > 0) {
    //         const initialValue = [campaignList[0].value];
    //         // setSelectedCampaigns(initialValue);
    //         form.setValue("campaigns", initialValue);
    //     }

    // }, [campaignList, form, mails]);

    const multiSelectRef = useRef(null);

    if (error) {
        return <div>There was an error fetching campaign mails data...</div>
    }

    const searchHandler = async (value: any) => {
        const { campaigns, from, to } = value;

        setSearchParams({
            campaign_ids: campaigns,
            from: new Date(from).toISOString(),
            to: setToEndOfDay(new Date(to)).toISOString(),
        });
    };

    const deleteMailHandler = async (id: string ) => {
        if (deletionError) {
            return <div>Error deleting the mail</div>
        }

        await deleteMail(id);
    }

    return (
        <div>
            {/* Template Page heading... */}
            <div className='w-full'>
                <h1 className='text-xl font-bold'>
                    Analytics
                </h1>
                <div className='my-4'>
                    <Form {...form}>
                        <form onSubmit={form.handleSubmit(searchHandler)} className="flex flex-col lg:flex-row gap-x-4 space-y-4 lg:items-end ">
                            {/* Campaigns Name to search for */}
                            {/* <FormField
                                control={form.control}
                                name="campaigns"
                                render={({ field, fieldState }) => (
                                    <FormItem className='flex-1'>
                                        <FormLabel className="font-bold text-black">Campaigns</FormLabel>
                                        <FormControl>
                                            <MultiSelect
                                                options={campaignList}
                                                onValueChange={(value) => {
                                                    setSelectedCampaigns(value);
                                                    form.setValue("campaigns", value); // Update React Hook Form state
                                                    form.trigger("campaigns");
                                                }}
                                                value={selectedCampaigns}
                                                placeholder="Select campaigns"
                                                variant="inverted"
                                                className='flex-1'
                                                ref={multiSelectRef}
                                                maxCount={3}
                                            />
                                        </FormControl>
                                        <FormMessage>{form.formState.errors.campaigns?.message}</FormMessage>
                                    </FormItem>
                                )}
                            /> */}

                            <div className='flex gap-x-4'>
                                <DatePicker
                                    form={form}
                                    label="From"
                                    fieldName="from"
                                    placeholder="Pick a date"
                                />
                                <DatePicker
                                    form={form}
                                    label="To"
                                    fieldName="to"
                                    placeholder="Pick a date"
                                />
                            </div>

                            <div className='flex space-x-4 justify-end'>
                                <Button 
                                    type='submit' 
                                    variant={'default'} 
                                    className=''
                                >
                                    <Search size={16}/>
                                    <span className='lg:hidden'>Search</span>
                                </Button>
                            </div>
                        </form>
                    </Form>
                </div>
            </div>

            {/* Data Table */}
            <div className='my-4 z-20 max-h-[444px] overflow-y-auto thin-scrollbar'>
                <DataTable 
                    data={mails || []} 
                    columns={columns(deleteMailHandler)}  // Pass columns directly
                    fallback={"No mails found"}   
                    isLoading={isLoading} 
                />
            </div>
        </div>
    );
};