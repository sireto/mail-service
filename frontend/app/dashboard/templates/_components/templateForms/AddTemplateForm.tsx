'use client'
/* eslint-disable @typescript-eslint/no-unused-vars */
import React from 'react';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import {
    DialogFooter,
} from '@/components/ui/dialog';
import { AddTemplateFormSchemaDTO } from '@/lib/type';
import { useCreateTemplateMutation } from '@/app/services/TemplateApi';
import { DialogClose } from '@radix-ui/react-dialog';
import TemplateModalBody from '@/app/dashboard/templates/_components/TemplateModalBody';
import { Button } from '@/components/ui/button';

const AddTemplateForm = () => {
    // const [parsedHtml, setParsedHtml] = useState("");
    const form = useForm<z.infer<typeof AddTemplateFormSchemaDTO>>({
        resolver: zodResolver(AddTemplateFormSchemaDTO),
        defaultValues: {
            name: "",
            raw_mjml_content: "<mjml><mj-body>Hi, {{name}}</mj-body></mjml>",
        }
    });

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

export default AddTemplateForm;