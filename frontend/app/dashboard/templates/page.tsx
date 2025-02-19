'use client';

import React from 'react'

import Modal from '@/components/Modal';
import AddTemplateForm from './_components/templateForms/AddTemplateForm';
import { useGetTemplatesQuery } from '@/app/services/TemplateApi';
import DataTable from '@/components/DataTable';
import columns from './_columns';
import AddButton from '@/components/common/AddButton';


const page = () => {
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
                triggerButton={<AddButton />}
                dialogBody={<AddTemplateForm />}
                dialogTitle={"New template"}
                dialogDescription={"Add a new template"}
            />
        </div>
        <div className='my-12'>
            <DataTable 
                data={templates || []} 
                columns={columns}
                fallback={"No templates found"}
            />
        </div>
    </div>
  )
}

export default page