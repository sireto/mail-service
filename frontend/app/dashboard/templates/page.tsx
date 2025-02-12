import React from 'react'

import Modal from '@/components/Modal';
import { Plus } from "lucide-react";
import { AddTemplateForm } from '@/components/TemplateForms';
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { ScanEye, Edit3, Trash2 } from 'lucide-react';
// import Link from 'next/link';
// import { Button } from '@/components/ui/button';
import { TemplateDTO } from '@/lib/type';
import { z } from "zod";
import { formatDate } from '@/lib/utils';
import TemplateList from '@/components/TemplateList';
import { useGetTemplatesQuery } from '@/app/services/TemplateApi';

const BASE_URL = "http://localhost:8000/api";

const fetchAllTemplates = async (): Promise<z.infer<typeof TemplateDTO>[] | []> => {
    const listTemplates = await fetch(`${BASE_URL}/templates`);
    const response = await listTemplates.json();    
    return response;
}

const page = async () => {
    const addBtn = {
        label: "New",
        icon: <Plus size={24} />
    };

    // const { data, error, isLoading, refetch } = useGetTemplatesQuery();
    

    const templates: z.infer<typeof TemplateDTO>[] | [] = await fetchAllTemplates();

    return (
    <div className=''>
        {/* Template page heading... */}
        <div className='w-full flex justify-between items-center'>
            <h1 className='text-xl font-bold'>
                Templates
                <span>({templates.length})</span>
            </h1>
            <Modal 
                triggerLabel={addBtn} 
                dialogBody={<AddTemplateForm />}
                dialogTitle={"New template"}
                dialogDescription={"Add a new template"}
            />
        </div>
        <div className='my-12'>
            {/* Template page content... */}
            <Table>
                <TableCaption>A list of templates</TableCaption>
                <TableHeader>
                    <TableRow>
                        <TableHead>Name</TableHead>
                        <TableHead>Created</TableHead>
                        <TableHead>Updated</TableHead>
                        <TableHead></TableHead>
                    </TableRow>
                </TableHeader>
                {/* Custome TemplateList component for the table body... */}
                <TemplateList/>
            </Table>
        </div>
    </div>
  )
}

export default page