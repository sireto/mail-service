'use client'

import React, { useEffect, useRef } from 'react';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import {
    DialogFooter,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { AddTemplateFormSchemaDTO } from '@/lib/type';
import { useGetTemplatesQuery, useUpdateTemplateMutation } from '@/app/services/TemplateApi';
import { DialogClose } from '@radix-ui/react-dialog';
import TemplateModalBody from '@/app/dashboard/templates/_components/TemplateModalBody';

const EditTemplateForm = ({ templateId }: { templateId: string }) => {
    const { data } = useGetTemplatesQuery();
    const [updateTemplate, { isLoading: isUpdating, error: updateError }] = useUpdateTemplateMutation();
    const closeRef = useRef<HTMLButtonElement>(null);
    
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
    }, [data, form, templateId]);

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
        closeRef.current?.click();
    }

    return (
        <TemplateModalBody 
            modalTitle={"Edit Template"}
            modalDescription={"Edit your template"}
            form={form}
            submitHandler={editTemplate}
            isUpdating={isUpdating}
            triggerButton={<DialogFooter>
                <DialogClose className='mt-2 md:mt-0' asChild>
                    <Button type="button" variant={"outline"}>Close</Button>
                </DialogClose>
                <Button type="submit" disabled={isUpdating}>Save</Button>
                <DialogClose asChild>
                    <button ref={closeRef} className="hidden" />
                </DialogClose>
            </DialogFooter>}
        />
      )
}

export default EditTemplateForm;