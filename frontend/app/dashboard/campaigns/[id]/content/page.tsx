'use client';

import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Textarea } from '@/components/ui/textarea';
import { AddTemplateFormSchemaDTO } from '@/lib/type';
import { zodResolver } from '@hookform/resolvers/zod';
import { Input } from '@/components/ui/input';
import React from 'react'
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Save, ClipboardX } from 'lucide-react';
import { useGetTemplateByIdQuery, useGetTemplatesQuery } from '@/app/services/TemplateApi';
import { useParams } from 'next/navigation';
import { useGetCampaignByIdQuery } from '@/app/services/CampaignApi';


const Page = () => {
  const { id } = useParams<{ id: string }>();
  const { data: currentCampaign, error, isLoading } = useGetCampaignByIdQuery(id);
  const { data: currentTemplate, error: templateError, isLoading: templateLoading } = useGetTemplateByIdQuery(currentCampaign?.template_id ?? '');

  const form = useForm<z.infer<typeof AddTemplateFormSchemaDTO>>({
         resolver: zodResolver(AddTemplateFormSchemaDTO),
         defaultValues: {
             name: currentTemplate?.name ?? "",
             raw_mjml_content: currentTemplate?.content_html ?? "<mjml><mj-body>Hi, {{name}}</mj-body></mjml>",
         }
     });

  const submitHandler = () => {
    
  }

  return (
    <Form {...form}>    {/* pass on the all the form-related methods allowing child components to access the form's context... */}
                <form 
                    onSubmit={form.handleSubmit(submitHandler)}
                    className='flex flex-col space-y-4'
                >
                    <FormField
                    control={form.control}
                    name="name"
                    render={({ field, fieldState }) => (
                        <FormItem>
                            <FormLabel className='font-bold text-black'>Name</FormLabel>
                            <FormControl>
                                <Input 
                                    placeholder="Template name" {...field} 
                                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                    disabled
                                />
                            </FormControl>
                            <FormMessage>{form.formState.errors.name?.message}</FormMessage>
                        </FormItem>
                    )}
                    />
                    <FormField
                        control={form.control}
                        name="raw_mjml_content"
                        render={({ field, fieldState }) => (
                            <FormItem>
                                <FormLabel className='font-bold text-black'>MJML Content</FormLabel>
                                <FormControl>
                                    <Textarea 
                                        rows={16} 
                                        placeholder="MJML content" 
                                        {...field} 
                                        className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                    />
                                </FormControl>
                                <FormMessage>{form.formState.errors.raw_mjml_content?.message}</FormMessage>
                            </FormItem>
                        )}
                    />
                    <div className='flex items-center justify-end space-x-4'>
                      <Button 
                      type='button' 
                      variant={'outline'} 
                      className='text-primary border-primary'
                      onClick={() => form.reset()}
                      >
                        <ClipboardX size={16}/>
                        <span>Discard</span>
                      </Button>
                      <Button type="submit" className="bg-primary text-white py-2 px-4 rounded-md">
                        <Save size={16} />
                        <span>Save Changes</span>
                      </Button>
                    </div>
                </form>
            </Form>
  )
}

export default Page