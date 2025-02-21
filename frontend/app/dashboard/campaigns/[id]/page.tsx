'use client';

import { AddCampaignFormSchemaDTO } from '@/lib/type';
import React, { useEffect } from 'react'
import { useParams, useRouter } from 'next/navigation';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useCreateCampaignMutation, useGetCampaignByIdQuery, useGetCampaignsQuery, useUpdateCampaignMutation } from '@/app/services/CampaignApi';
import { Save } from 'lucide-react';
import { Button } from '@/components/ui/button';
import CampaignForm from './_components/CampaignForm';
import SendTestMailForm from './_components/SendTestMailForm';


const page = () => {
  const { id } = useParams<{ id: string }>();
  const router = useRouter();
    const isEditing = id !== "new";  // if not new this page is opened in the editing mode...
    const form = useForm<z.infer<typeof AddCampaignFormSchemaDTO>>({
        resolver: zodResolver(AddCampaignFormSchemaDTO),
        defaultValues: {
            campaign_name: "",
            campaign_senders: "",
            namespace_id: "",
            template_id: "",
            list_id: "",
        }
    });

    const { refetch } = useGetCampaignsQuery();
    const { data: campaignData, error, isLoading } = useGetCampaignByIdQuery(id, { skip: !isEditing }); 
    const [ createCampaign, { isLoading: isCreating, error: creationError }] = useCreateCampaignMutation();
    const [ updateCampaign, { isLoading: isUpdating, error: updateError }] = useUpdateCampaignMutation();

    useEffect(() => {
      if (isEditing && campaignData) {
        form.reset(campaignData);
      }
    }, [campaignData, form, isEditing]);

    const saveCampaignChanges = async (value: z.infer<typeof AddCampaignFormSchemaDTO>) => {
        console.warn("THE FORM VALUES:", value);


        if (isEditing) {
          const updatedCampaign = {
            campaign_name: value.campaign_name.trim(),
            campaign_senders: value.campaign_senders.trim(),
            status: "draft",  // Update status if needed
            template_id: value.template_id.trim(),
            scheduled_at: "2023-01-01T00:00:00Z" // Ensure this is in the correct format
          };

          const updatedCampaignData = {
            campaignId: id,
            updatedCampaign: updatedCampaign
          };

          await updateCampaign(updatedCampaignData);
          form.reset();
        } else {
          const newCampaign = {
            campaign_name: value.campaign_name.trim(),
            campaign_senders: value.campaign_senders.trim(),
            namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
            template_id: value.template_id.trim(),
            status: "draft",
            list_id: value.list_id.trim(),
            // scheduled_at: (new Date()).toISOString(),
            scheduled_at: "2025-02-10T12:00:00"
          };

          await createCampaign(newCampaign);
          form.reset();
        }
        router.push('/dashboard/campaigns');
        refetch();
    }
    

  return (
    <div className='flex flex-col space-y-8 lg:space-y-0 lg:flex-row lg:justify-between lg:gap-x-64'>
        <CampaignForm 
            form={form}
            submitHandler={saveCampaignChanges}
            triggerButton={
                <Button type="submit" className="bg-primary text-white py-2 px-4 rounded-md">
                    <Save size={16} />
                    <span>Save Changes</span>
                </Button>
            }
        />
        <hr className='lg:hidden'/>
        <SendTestMailForm templateId={form.getValues("template_id")}/>
    </div>
  )
}

export default page