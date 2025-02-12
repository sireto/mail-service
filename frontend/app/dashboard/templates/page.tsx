'use client';

import React from 'react'

import Modal from '@/components/Modal';
import { Plus } from "lucide-react";
import { AddTemplateForm } from '@/components/TemplateForms';
import { useGetTemplatesQuery } from '@/app/services/TemplateApi';
import DataTable from '@/components/DataTable';
import columns from './columns';

const page = () => {
    const addBtn = {
        label: "New",
        icon: <Plus size={24} />
    };

    const { data: templates, error, isLoading } = useGetTemplatesQuery();

    if (error) {
        return <div>There was an error fetching templates...</div>
    }

    if (isLoading) {
        return <div> Loading... </div>
    }
    
    return (
    <div className=''>
        {/* Template page heading... */}
        <div className='w-full flex justify-between items-center'>
            <h1 className='text-xl font-bold'>
                Templates
                <span>({templates?.length})</span>
            </h1>
            <Modal 
                triggerLabel={addBtn} 
                dialogBody={<AddTemplateForm />}
                dialogTitle={"New template"}
                dialogDescription={"Add a new template"}
            />
        </div>
        <div className='my-12'>
            <DataTable data={templates || []} columns={columns}/>
        </div>
    </div>
  )
}

export default page