'use client';

import React from 'react'

import Modal from '@/components/Modal';
import { Plus } from "lucide-react";
import { AddListForm } from '@/components/ListForms';
import { useGetListsQuery } from '@/app/services/ListApi';
import { Button } from '@/components/ui/button';
import DataTable from '@/components/DataTable';
import columns from './columns';

const addButton = (
    <Button variant={"default"}>
        <>
            <Plus size={24} />
            <span className='ml-1'>New</span>
        </>
    </Button>
);

const page = () => {
    const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

    const { data: lists, error, isLoading } = useGetListsQuery(namespaceId);

    if (error) {
        return <div>There was an error fetching lists...</div>
    }

    if (isLoading) {
        return <div> Loading... </div>
    }

    return (
        <div className=''>
            {/* Template page heading... */}
            <div className='w-full flex justify-between items-center'>
                <h1 className='text-xl font-bold'>
                    Lists
                    <span>({lists?.length})</span>
                </h1>
                <Modal 
                    triggerButton={addButton}
                    dialogBody={<AddListForm />}
                    dialogTitle={"New template"}
                    dialogDescription={"Add a new template"}
                />
            </div>
            <div className='my-12'>
                <DataTable 
                    data={lists || []} 
                    columns={columns}
                    fallback={"No lists found"}    
                />
            </div>
        </div>
      )
}

export default page