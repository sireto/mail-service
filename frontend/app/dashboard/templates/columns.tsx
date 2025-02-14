'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import { Edit3, ScanEye, Trash2 } from 'lucide-react';
import { EditTemplateForm } from '@/components/TemplateForms';
import Modal from '@/components/Modal';
import { useDeleteTemplateMutation, useGetTemplatesQuery } from '@/app/services/TemplateApi';


const updateBtn = {
        label: "Edit",
        icon: <Edit3 size={20} />
};

export const columns: ColumnDef<any>[] = [
    {
        accessorKey: "name",
        header: "Name",
        cell: ({ row }) => <span>{row.getValue("name")}</span>,
    },
    {
        accessorKey: "created_at",
        header: "Created",
        cell: ({ row }) => <span>{formatDate(row.getValue("created_at"))}</span>,
    },
    {
        accessorKey: "updated_at",
        header: "Updated",
        cell: ({ row }) => <span>{formatDate(row.getValue("updated_at"))}</span>,
    },
    {
        accessorKey: "actions",
        header: "",
        cell: ({ row }) => {
            const templateId = row.original.id;

            const { refetch } = useGetTemplatesQuery();
            
            const [ deleteTemplate, { isLoading: isDeleting, error: deletionError } ] = useDeleteTemplateMutation();

            const deleteTemplateHandler = async (id: string) => {
                if (deletionError) {
                    return <div>Error deleting the template</div>
                }
            
                await deleteTemplate(id);
                refetch();
            }
            

            return (
                <div className='flex space-x-4 text-primary items-center'>
                    <button className='transition-all duration-300 ease-in-out hover:scale-105'><ScanEye size={20} /></button>
                        {/* <button className='transition-all duration-300 ease-in-out hover:scale-105'><Edit3 size={16} /></button> */}
                        <Modal 
                            triggerLabel={updateBtn} 
                            dialogBody={<EditTemplateForm templateId={templateId}/>}
                            dialogTitle={"Edit Template"}
                            dialogDescription={"Edit your template"}
                        />
                        <button 
                            className='transition-all duration-300 ease-in-out hover:scale-105'
                            onClick={() => {
                                deleteTemplateHandler(templateId);
                            }}
                        >
                            <Trash2 size={20} className='text-red-400' />
                    </button>
                </div>
            )
        }
            
    },
  ]

export default columns