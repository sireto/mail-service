'use client';

import React from 'react'

import Modal from '@/components/Modal';
import { Plus } from "lucide-react";
import { AddListForm } from '@/components/ListForms';
import { Button } from '@/components/ui/button';
import DataTable from '@/components/DataTable';
import columns from './columns';
import { useDeleteCampaignMutation, useGetCampaignsQuery, useStartCampaignMutation } from '@/app/services/CampaignApi';
import Link from 'next/link';

const addButton = (
    <Button variant={"default"}>
        <>
            <Plus size={24} />
            <span className='ml-1'>New</span>
        </>
    </Button>
);

const Page = () => {
    const { data: campaigns, error, isLoading, refetch } = useGetCampaignsQuery();
    const [ deleteCampaign, { isLoading: isDeleting, error: deletionError }] = useDeleteCampaignMutation();
    const [ startCampaign, { isLoading: isStarting, error: startingError }] = useStartCampaignMutation();
    

    if (error) {
        return <div>There was an error fetching campaigns...</div>
    }

    if (isLoading) {
        return <div> Loading... </div>
    }
    
    const deleteCampaignHandler = async (id: string) => {
        if (deletionError) {
            return <div>Error deleting the campaign</div>
        }
    
        await deleteCampaign(id);
        refetch();
    }

    const startCampaignHandler = async (id: string) => {
        if (startingError) {
            return <div>Error starting the campaign</div>
        }

        const campaignData = {
            campaignId: id,
            listId: "0a48a82f-ec04-4c19-904c-48dcebc80e49"
        };

        await startCampaign(campaignData);
    }

    return (
        <div className=''>
            {/* Template page heading... */}
            <div className='w-full flex justify-between items-center'>
                <h1 className='text-xl font-bold'>
                    Campaigns
                    <span>({campaigns?.length})</span>
                </h1>
                <Link href={'/dashboard/campaigns/new'}>
                    { addButton }
                </Link>
            </div>
            <div className='my-12'>
                <DataTable 
                    data={campaigns || []} 
                    columns={columns(deleteCampaignHandler, startCampaignHandler)}
                    fallback={"No campaigns found"}    
                />
            </div>
        </div>
      )
}

export default Page