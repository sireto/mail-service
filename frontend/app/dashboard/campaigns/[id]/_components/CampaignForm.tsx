import { FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form';
import { Form } from '@/components/ui/form';
import { Select, SelectTrigger, SelectValue, SelectContent } from '@/components/ui/select';
import { AddCampaignFormSchemaDTO, ListDTO, TemplateDTO } from '@/lib/type';
import { Input } from '@/components/ui/input';
import React from 'react'
import { UseFormReturn } from 'react-hook-form';
import { z } from 'zod';
import { useGetTemplatesQuery } from '@/app/services/TemplateApi';
import { ClipboardX } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useGetListsQuery } from '@/app/services/ListApi';
import DropdownItemList from './DropdownItemList';

interface CampaignFormProps {
    form: UseFormReturn<z.infer<typeof AddCampaignFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddCampaignFormSchemaDTO>) => void;
    triggerButton: React.ReactNode;
}

type Template = z.infer<typeof TemplateDTO>;
type List = z.infer<typeof ListDTO>;

const namespaceId : string | undefined = process.env.NEXT_PUBLIC_NAMESPACE_ID;


const CampaignForm = (props: CampaignFormProps) => {
    const { form, submitHandler, triggerButton } = props;

    const { data: templates, error, isLoading } = useGetTemplatesQuery();
    const { data: lists } = useGetListsQuery(namespaceId || '');
    
  
    if (error) {
        return <div>There was an error fetching templates...</div>
    }

  
    return (
      <Form {...form}>
        <form onSubmit={form.handleSubmit(submitHandler)} className="flex flex-col space-y-4 flex-1">
          
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
                      <DropdownItemList<Template> items={templates} />
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
                      <DropdownItemList<List> items={lists} />
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

export default CampaignForm;