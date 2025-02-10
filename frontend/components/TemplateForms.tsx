'use client'

import React from 'react';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { 
    Form, 
    FormField ,
    FormControl,
    FormLabel,
    FormItem,
    FormMessage,
} from '@/components/ui/form';
import { Input } from './ui/input';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';

const BASE_URL = "http://localhost:8000/api";

const addTemplateFormSchema = z.object({
    name: z.string().nonempty().min(2, "Template name must be atleast 2 characters long.").max(50),
    raw_mjml_content: z.string().nonempty().refine((content) => {
        return content.includes("<mjml>") && content.includes("<mj-body>");
    }, {
        message: "MJML content must be wrapped in the <mjml> tags."
    }),
});

const AddTemplateForm = () => {
    const form = useForm<z.infer<typeof addTemplateFormSchema>>({
        resolver: zodResolver(addTemplateFormSchema),
        defaultValues: {
            name: "",
            raw_mjml_content: "<mjml><mj-body>Hi, {{name}}</mj-body></mjml>",
        }
    });

    async function addNewTemplate(value: z.infer<typeof addTemplateFormSchema>) {
        console.log(value);

        const newTemplate = {
            name: value.name.trim(),
            content_html: value.raw_mjml_content.trim(),
            namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
            content_plaintext: "Hi, {{ name }}",
            template_data: JSON.stringify({
                name: "John Doe"
            })
        };

        try {
            const response = await fetch(`${BASE_URL}/templates`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(newTemplate)
            });
    
            if (response.ok) {
                alert("New template created successfully.");
            }
            form.reset();
        } catch (err) {
            console.error("Error creating new template: ", err);
        }
    }

  return (
    <Form {...form}>    {/* pass on the all the form-related methods allowing child components to access the form's context... */}
        <form 
            onSubmit={form.handleSubmit(addNewTemplate)}
            className='flex flex-col space-y-4'
        >
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                    <FormLabel className='font-bold'>Name</FormLabel>
                    <FormControl>
                        <Input placeholder="Template name" {...field} />
                    </FormControl>
                    <FormMessage>{form.formState.errors.name?.message}</FormMessage>
                </FormItem>
              )}
            />
            <FormField
                control={form.control}
                name="raw_mjml_content"
                render={({ field }) => (
                    <FormItem>
                        <FormLabel className='font-bold'>MJML Content</FormLabel>
                        <FormControl>
                            <Textarea placeholder="MJML content" {...field} />
                        </FormControl>
                        <FormMessage>{form.formState.errors.raw_mjml_content?.message}</FormMessage>
                    </FormItem>
                )}
            />
            <Button type="submit">Create</Button>
        </form>
    </Form>
  )
}

export { AddTemplateForm }