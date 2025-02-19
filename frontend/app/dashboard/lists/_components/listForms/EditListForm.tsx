'use client'
/* eslint-disable @typescript-eslint/no-unused-vars */
import React, { useEffect } from 'react';
import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import {
    DialogClose,
    DialogFooter,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { AddListFormSchemaDTO } from '@/lib/type';
import { useGetListsQuery, useUpdateListMutation } from '@/app/services/ListApi';
import ListModalBody from '@/app/dashboard/lists/_components/ListModalBody';

const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";


const EditListForm = ({ listId }: { listId: string }) => {
    const { data } = useGetListsQuery(namespaceId);
    const [updateList, { isLoading: isUpdating, error: updateError }] = useUpdateListMutation();
    
    const form = useForm<z.infer<typeof AddListFormSchemaDTO>>({
        resolver: zodResolver(AddListFormSchemaDTO),
        defaultValues: {
            name: "",
            description: "",
        }
    });


    useEffect(() => {
        if (data) {
            const list = data.find((list) => list.id === listId);

            if (list) {
                form.setValue("name", list.name);
                form.setValue("description", list.description);
            }
        }
    }, [data]);

    if (updateError) {
        return <div>There was an error updating the list...</div>
    }

    async function editList(value: z.infer<typeof AddListFormSchemaDTO>) {
        const updatedList = {
            name: value.name.trim(),
            description: value.description.trim(),
        };

        await updateList({
            listId,
            namespaceId,
            updatedList
        });
        form.reset();
    }

    return (
        <ListModalBody 
            modalTitle={"Edit List"}
            modalDescription={"Edit your list"}
            form={form}
            submitHandler={editList}
            triggerButton={<DialogFooter>
                <DialogClose asChild>
                    <Button type="button" variant={"outline"}>Close</Button>
                </DialogClose>
                <Button type="submit" disabled={isUpdating}>Save</Button>
            </DialogFooter>}
        />
    )
}

export default EditListForm;