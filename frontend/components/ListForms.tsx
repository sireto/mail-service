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
    DialogClose,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog';
import { Input } from './ui/input';
import { Button } from './ui/button';
import { Textarea } from './ui/textarea';
import { ScanEye } from 'lucide-react';
import { AddListFormSchemaDTO } from '@/lib/type';
import { useCreateTemplateMutation, useGetTemplatesQuery, useUpdateTemplateMutation } from '@/app/services/TemplateApi';
import { useCreateListMutation, useGetListsQuery, useUpdateListMutation } from '@/app/services/ListApi';

/* eslint-disable @typescript-eslint/no-unused-vars */
const BASE_URL = "http://localhost:8000/api";
/* eslint-disable @typescript-eslint/no-unused-vars */

const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

interface ListModalProps {
    modalTitle: string;
    modalDescription: string;
    form: UseFormReturn<z.infer<typeof AddListFormSchemaDTO>>;
    submitHandler: (value: z.infer<typeof AddListFormSchemaDTO>) => void;
    triggerButton: React.ReactNode;
}

const ListModal = ({
    modalTitle,
    modalDescription,
    form,
    submitHandler,
    triggerButton,
}: ListModalProps) => {
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
                                    placeholder="List name" 
                                    {...field} 
                                    className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                />
                            </FormControl>
                            <FormMessage>{form.formState.errors.name?.message}</FormMessage>
                        </FormItem>
                    )}
                    />
                    <FormField
                        control={form.control}
                        name="description"
                        render={({ field, fieldState }) => (
                            <FormItem>
                                <FormLabel className='font-bold text-black'>Description</FormLabel>
                                <FormControl>
                                    <Textarea 
                                        rows={8} 
                                        placeholder="Add Description here..." 
                                        {...field} 
                                        className={fieldState.invalid ? "border-red-400 focus-visible:ring-red-500" : ""}
                                    />
                                </FormControl>
                                <FormMessage>{form.formState.errors.description?.message}</FormMessage>
                            </FormItem>
                        )}
                    />
                    { triggerButton }
                </form>
            </Form>
        </>
    )
}


const AddListForm = () => {
    // const [parsedHtml, setParsedHtml] = useState("");
    const form = useForm<z.infer<typeof AddListFormSchemaDTO>>({
        resolver: zodResolver(AddListFormSchemaDTO),
        defaultValues: {
            name: "",
            description: "",
        }
    });

    const { refetch } = useGetListsQuery(namespaceId);
    const [createList, { isLoading: isCreating, error: createError }] = useCreateListMutation();

    if (createError) {
        return <div>There was an error creating list...</div>
    }

    // if (isCreating) {
    //     return <div>Creating template...</div>
    // }

    async function addNewList(value: z.infer<typeof AddListFormSchemaDTO>) {
        console.log(value);

        const newList = {
            name: value.name.trim(),
            description: value.description.trim(),
            namespace_id: namespaceId,
        };

        await createList(newList);
        form.reset();
        refetch();
    }

  return (
    <ListModal 
            modalTitle={"Add List"}
            modalDescription={"Add a new list"}
            form={form}
            submitHandler={addNewList}
            triggerButton={<DialogFooter>
                <DialogClose asChild>
                    <Button type="button" variant={"outline"}>Close</Button>
                </DialogClose>
                <Button type="submit" disabled={isCreating}>Create</Button>
            </DialogFooter>}
        />
  )
}

const EditListForm = ({ listId }: { listId: string }) => {
    const { data, refetch } = useGetListsQuery(namespaceId);
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
        refetch();
    }

    return (
        <ListModal 
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

export { 
    AddListForm,
    EditListForm
};