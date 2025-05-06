'use client'

import React, { useRef } from 'react';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import {
    DialogClose,
    DialogFooter,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { AddListFormSchemaDTO } from '@/lib/type';
import { useCreateListMutation } from '@/app/services/ListApi';
import ListModalBody from '@/app/dashboard/lists/_components/ListModalBody';


const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";


const AddListForm = () => {
    // const [parsedHtml, setParsedHtml] = useState("");
    const form = useForm<z.infer<typeof AddListFormSchemaDTO>>({
        resolver: zodResolver(AddListFormSchemaDTO),
        defaultValues: {
            name: "",
            description: "",
        }
    });

    const [createList, { isLoading: isCreating, error: createError }] = useCreateListMutation();
    const closeRef = useRef<HTMLButtonElement>(null);

    if (createError) {
        return <div>There was an error creating list...</div>
    }

    async function addNewList(value: z.infer<typeof AddListFormSchemaDTO>) {
        console.log(value);

        const newList = {
            name: value.name.trim(),
            description: value.description.trim(),
            namespace_id: namespaceId,
        };

        await createList(newList);
        form.reset();
        closeRef.current?.click();
    }

  return (
    <ListModalBody 
            modalTitle={"Add List"}
            modalDescription={"Add a new list"}
            form={form}
            submitHandler={addNewList}
            triggerButton={<DialogFooter>
                <DialogClose asChild>
                    <Button type="button" variant={"outline"}>Close</Button>
                </DialogClose>
                <Button type="submit" disabled={isCreating}>Create</Button>
                {/* hidden button to close dialog */}
                <DialogClose asChild>
                    <button ref={closeRef} className="hidden" />
                </DialogClose>
            </DialogFooter>}
        />
  )
}

export default AddListForm;