'use client';

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
import { useParams, useRouter } from 'next/navigation';
import { UseFormReturn, useForm } from 'react-hook-form';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useCreateTemplateMutation, useGetTemplatesQuery } from '@/app/services/TemplateApi';
import { useCreateCampaignMutation, useGetCampaignByIdQuery, useGetCampaignsQuery, useUpdateCampaignMutation } from '@/app/services/CampaignApi';
import { Save, ClipboardX } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useGetListsQuery } from '@/app/services/ListApi';


interface CampaignFormProps {
    form: UseFormReturn<z.infer<typeof AddCampaignFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddCampaignFormSchemaDTO>) => void;
    triggerButton: React.ReactNode;
}

const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";


const CampaignForm = (props: CampaignFormProps) => {
    const { form, submitHandler, triggerButton } = props;
    const [selectedTemplate, setSelectedTemplate] = useState<string | null>(null);

    const { data: templates, error, isLoading } = useGetTemplatesQuery();
    const { data: lists } = useGetListsQuery(namespaceId);
    
  
    if (error) {
        return <div>There was an error fetching templates...</div>
    }

    const templatesComponent = templates?.map((template) => (
        <SelectItem key={template.id} value={template.id}>
          {template.name}
        </SelectItem>
    ));

    const listComponent = lists?.map((list) => (
        <SelectItem key={list.id} value={list.id}>
          {list.name}
        </SelectItem>
    ));


    return (
      <Form {...form}>
        <form onSubmit={form.handleSubmit(submitHandler)} className="flex flex-col space-y-4">
          
          {/* Campaign Name */}
          <FormField
            control={form.control}
            name="campaign_name"
            render={({ field, fieldState }) => (
              <FormItem>
                <FormLabel className="font-bold text-black">Campaign Name</FormLabel>
                <FormControl>
                  <Input
                    placeholder="Enter campaign name"
                    {...field}
                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                  />
                </FormControl>
                <FormMessage>{form.formState.errors.campaign_name?.message}</FormMessage>
              </FormItem>
            )}
          />
  
          {/* Campaign Senders */}
          <FormField
            control={form.control}
            name="campaign_senders"
            render={({ field, fieldState }) => (
              <FormItem>
                <FormLabel className="font-bold text-black">Sender</FormLabel>
                <FormControl>
                  <Input
                    placeholder="Enter sender email"
                    {...field}
                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                  />
                </FormControl>
                <FormMessage>{form.formState.errors.campaign_senders?.message}</FormMessage>
              </FormItem>
            )}
          />

          {/* Namespaces */}
          {/* <FormField
            control={form.control}
            name="namespace_id"
            render={({ field, fieldState }) => (
              <FormItem>
                <FormLabel className="font-bold text-black">Namespace</FormLabel>
                <FormControl>
                  <Input
                    placeholder="Enter sender email"
                    {...field}
                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                  />
                </FormControl>
                <FormMessage>{form.formState.errors.namespace_id?.message}</FormMessage>
              </FormItem>
            )}
          /> */}
  
          {/* Templates */}
          <FormField
            control={form.control}
            name="template_id"
            render={({ field, fieldState }) => (
              <FormItem>
                <FormLabel className="font-bold text-black">Template</FormLabel>
                <FormControl>
                <Select onValueChange={field.onChange} value={field.value}>
                    <SelectTrigger className="w-[180px]">
                      <SelectValue placeholder="Select your Template" />
                    </SelectTrigger>
                    <SelectContent>
                      { templatesComponent }
                    </SelectContent>
                </Select>
                </FormControl>
                <FormMessage>{form.formState.errors.template_id?.message}</FormMessage>
              </FormItem>
            )}
          />

        <FormField
            control={form.control}
            name="list_id"
            render={({ field, fieldState }) => (
              <FormItem>
                <FormLabel className="font-bold text-black">Lists</FormLabel>
                <FormControl>
                <Select onValueChange={field.onChange} value={field.value}>
                    <SelectTrigger className="w-[180px]">
                      <SelectValue placeholder="Select your Lists" />
                    </SelectTrigger>
                    <SelectContent>
                      { listComponent }
                    </SelectContent>
                </Select>
                </FormControl>
                <FormMessage>{form.formState.errors.list_id?.message}</FormMessage>
              </FormItem>
            )}
          />
  
            <div className='flex space-x-4 justify-end'>
                {/* add the reset function onClick to this button... */}
                <Button 
                    type='button' 
                    variant={'outline'} 
                    className='text-primary border-primary'
                    onClick={() => form.reset()}
                    >
                    <ClipboardX size={16}/>
                    <span>Discard</span>
                </Button>
                {triggerButton}
          </div>
        </form>
      </Form>
    );
  };

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
    <div className='flex flex-col space-y-8 lg:space-y-0 lg:flex-row lg:justify-between lg:gap-x-64'>
=======
    <>
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
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
        <hr className='lg:hidden'/>
        <SendTestMailForm templateId={form.getValues("template_id")}/>
    </div>
  )
}

export default Page
=======
    </>
  )
}

export default page
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
