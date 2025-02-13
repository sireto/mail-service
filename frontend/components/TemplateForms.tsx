'use client'
/* eslint-disable @typescript-eslint/no-unused-vars */
import React, { useEffect } from 'react';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm, UseFormReturn } from 'react-hook-form';
import { 
    Form, 
    FormField ,
    FormControl,
    FormLabel,
    FormItem,
    FormMessage,
} from '@/components/ui/form';
import {
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';
import { Input } from './ui/input';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';
import { ScanEye } from 'lucide-react';
import { AddTemplateFormSchemaDTO } from '@/lib/type';
import { useCreateTemplateMutation, useGetTemplatesQuery, useUpdateTemplateMutation } from '@/app/services/TemplateApi';
import { DialogClose } from '@radix-ui/react-dialog';

/* eslint-disable @typescript-eslint/no-unused-vars */
const BASE_URL = "http://localhost:8000/api";
/* eslint-disable @typescript-eslint/no-unused-vars */

interface TemplateModalBodyProps {
    modalTitle: string;
    modalDescription: string;
    form: UseFormReturn<z.infer<typeof AddTemplateFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddTemplateFormSchemaDTO>) => void;
    isUpdating: boolean;
    triggerButton: React.ReactNode;
}

const TemplateModalBody = ({
    modalTitle,
    modalDescription,
    form,
    submitHandler,
    triggerButton,
}: TemplateModalBodyProps) => {
    return (
        <>
            <DialogHeader className='flex flex-row justify-between items-center'>
                      <div>
                        <DialogTitle>
                          {modalTitle}
                        </DialogTitle>
                        <DialogDescription className='mt-1'>
                          {modalDescription}
                        </DialogDescription>
                      </div>
                      <Button variant={"default"} className='!mt-0' >
                        <ScanEye size={16} />
                        <span>Preview</span>
                      </Button>
                    </DialogHeader>
            <hr className='my-4'/>
    
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
                    { triggerButton }
                </form>
            </Form>
        </>
    )
}

const AddTemplateForm = () => {
    // const [parsedHtml, setParsedHtml] = useState("");
    const form = useForm<z.infer<typeof AddTemplateFormSchemaDTO>>({
        resolver: zodResolver(AddTemplateFormSchemaDTO),
        defaultValues: {
            name: "",
            raw_mjml_content: "<mjml><mj-body>Hi, {{name}}</mj-body></mjml>",
        }
    });

    const { refetch } = useGetTemplatesQuery();
    const [createTemplate, { isLoading: isCreating, error: createError }] = useCreateTemplateMutation();

    if (createError) {
        return <div>There was an error creating template...</div>
    }

    // if (isCreating) {
    //     return <div>Creating template...</div>
    // }

    async function addNewTemplate(value: z.infer<typeof AddTemplateFormSchemaDTO>) {
        console.log(value);

        const newTemplate = {
            name: value.name.trim(),
            content_html: value.raw_mjml_content.trim(),
            namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
            content_plaintext: "Hi, {{name}}",
            template_data: JSON.stringify({
                name: "John Doe"
            })
        };

        await createTemplate(newTemplate);
        form.reset();
        refetch();
    }

    const preview = async () => {
        const mjmlContent = form.getValues("raw_mjml_content");

        if (!mjmlContent) {
            alert("Please enter MJML content to preview.");
        }

        const requestBody = JSON.stringify({ mjml_content: mjmlContent });
        console.log("Sending request with body ===> ", requestBody);

        try {
            const response = await fetch('http://localhost:3000/api/parse-mjml', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: requestBody,
            });

            const data = await response.json();

            console.log("THE parsed html is ====> ", data.html);
        } catch (err) {
            console.error("Error parsing MJML content: ", err);
        }
    }

  return (
    <TemplateModalBody 
        modalTitle={"Add Template"}
        modalDescription={"Add a new template"}
        form={form}
        submitHandler={addNewTemplate}
        isUpdating={isCreating}
        triggerButton={<DialogFooter>
            <DialogClose asChild>
                <Button type="button" variant={"outline"}>Close</Button>
            </DialogClose>
            <Button type="submit" disabled={isCreating}>Create</Button>
        </DialogFooter>}
    />  
  )
}

const EditTemplateForm = ({ templateId }: { templateId: string }) => {
    const { data, refetch } = useGetTemplatesQuery();
    const [updateTemplate, { isLoading: isUpdating, error: updateError }] = useUpdateTemplateMutation();
    
    const form = useForm<z.infer<typeof AddTemplateFormSchemaDTO>>({
        resolver: zodResolver(AddTemplateFormSchemaDTO),
        defaultValues: {
            name: "",
            raw_mjml_content: "<mjml><mj-body>Hi, {{name}}</mj-body></mjml>",
        }
    });


    useEffect(() => {
        if (data) {
            const template = data.find((template) => template.id === templateId);

            if (template) {
                form.setValue("name", template.name);
                form.setValue("raw_mjml_content", template.content_html);
            }
        }
    }, [data]);

    if (updateError) {
        return <div>There was an error updating the template...</div>
    }

    async function editTemplate(value: z.infer<typeof AddTemplateFormSchemaDTO>) {
        const updatedTemplate = {
            name: value.name.trim(),
            content_html: value.raw_mjml_content.trim(),
            namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
            content_plaintext: "Hi, {{name}}",
            template_data: JSON.stringify({
                name: "John Doe"
            })
        };

        await updateTemplate({
            templateId,
            updatedTemplate
        });
        form.reset();
        refetch();
    }

    return (
        <TemplateModalBody 
            modalTitle={"Edit Template"}
            modalDescription={"Edit your template"}
            form={form}
            submitHandler={editTemplate}
            isUpdating={isUpdating}
            triggerButton={<DialogFooter>
                <DialogClose asChild>
                    <Button type="button" variant={"outline"}>Close</Button>
                </DialogClose>
                <Button type="submit" disabled={isUpdating}>Save</Button>
            </DialogFooter>}
        />
      )
}

export { 
    AddTemplateForm,
    EditTemplateForm,
};