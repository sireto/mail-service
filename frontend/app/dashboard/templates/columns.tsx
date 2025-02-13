'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import { Edit3, ScanEye, Trash2 } from 'lucide-react';
import { EditTemplateForm } from '@/components/TemplateForms';
import Modal from '@/components/Modal';
import { useDeleteTemplateMutation, useGetTemplatesQuery } from '@/app/services/TemplateApi';


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
                    return <div>Error deleting the list</div>
                }
            
                await deleteTemplate(id);
                refetch();
            }
            

            return (
                <div className='flex space-x-4 text-primary items-center'>
                    <button className='transition-all duration-300 ease-in-out hover:scale-105'><ScanEye size={20} /></button>
                        {/* <button className='transition-all duration-300 ease-in-out hover:scale-105'><Edit3 size={16} /></button> */}
                        <Modal 
                            triggerButton={<Edit3
                                size={20}
                                strokeWidth={1.5} 
                                className='text-lime-400 transition-all duration-300 ease-in-out hover:scale-105 hover:text-lime-500'
                            />}
                            dialogBody={<EditTemplateForm templateId={templateId}/>}
                            dialogTitle={"Edit Template"}
                            dialogDescription={"Edit your template"}
                        />
                        <button 
                            className='transition-all duration-300 ease-in-out hover:scale-105 text-red-400 hover:text-red-500'
                            onClick={() => {
                                deleteTemplateHandler(templateId);
                            }}
                        >
                            <Trash2 size={20} strokeWidth={1.5} />
                    </button>
                </div>
            )
        }
            
    },
  ]

export default columns;