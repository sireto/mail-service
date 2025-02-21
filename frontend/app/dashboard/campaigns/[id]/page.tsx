'use client';

<<<<<<< HEAD
<<<<<<< HEAD
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


const Page = () => {
  const { id } = useParams<{ id: string }>();
  const router = useRouter();
    const isEditing = id !== "new";  // if not new this Page is opened in the editing mode...
=======
import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form';
import { Form } from '@/components/ui/form';
import { Select, SelectTrigger, SelectItem, SelectValue, SelectContent } from '@/components/ui/select';
import { AddCampaignFormSchemaDTO, AddTemplateFormSchemaDTO } from '@/lib/type';
import { Input } from '@/components/ui/input';
import React, { useState, useEffect } from 'react'
=======
import { AddCampaignFormSchemaDTO } from '@/lib/type';
import React, { useEffect } from 'react'
>>>>>>> b2241a9 (fix: resolve the issue with the date picker Time Format while filtering the mails)
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
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
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

<<<<<<< HEAD
=======
    const { refetch } = useGetCampaignsQuery();
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
    const { data: campaignData, error, isLoading } = useGetCampaignByIdQuery(id, { skip: !isEditing }); 
    const [ createCampaign, { isLoading: isCreating, error: creationError }] = useCreateCampaignMutation();
    const [ updateCampaign, { isLoading: isUpdating, error: updateError }] = useUpdateCampaignMutation();

    useEffect(() => {
      if (isEditing && campaignData) {
        form.reset(campaignData);
      }
    }, [campaignData, form, isEditing]);

    const saveCampaignChanges = async (value: z.infer<typeof AddCampaignFormSchemaDTO>) => {
<<<<<<< HEAD
=======
        console.warn("THE FORM VALUES:", value);


>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
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
<<<<<<< HEAD
=======
        refetch();
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
    }
    

  return (
<<<<<<< HEAD
<<<<<<< HEAD
    <div className='flex flex-col space-y-8 lg:space-y-0 lg:flex-row lg:justify-between lg:gap-x-64'>
=======
    <>
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
=======
    <div className='flex flex-col space-y-8 lg:space-y-0 lg:flex-row lg:justify-between lg:gap-x-64'>
>>>>>>> b2241a9 (fix: resolve the issue with the date picker Time Format while filtering the mails)
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
<<<<<<< HEAD
<<<<<<< HEAD
        <hr className='lg:hidden'/>
        <SendTestMailForm templateId={form.getValues("template_id")}/>
    </div>
  )
}

export default Page
=======
    </>
=======
        <hr className='lg:hidden'/>
        <SendTestMailForm templateId={form.getValues("template_id")}/>
    </div>
>>>>>>> b2241a9 (fix: resolve the issue with the date picker Time Format while filtering the mails)
  )
}

export default page
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
