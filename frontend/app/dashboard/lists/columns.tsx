'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import { Edit3, Trash2 } from 'lucide-react';
import { EditListForm } from '@/components/ListForms';
import Modal from '@/components/Modal';
import { useDeleteListMutation, useGetListsQuery } from '@/app/services/ListApi';

const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

export const columns: ColumnDef<any>[] = [
    {
        accessorKey: "name",
        header: "Name",
        cell: ({ row }) => <span>{row.getValue("name")}</span>,
    },
    {
        accessorKey: "description",
        header: "Description",
        cell: ({ row }) => <span>{row.getValue("description")}</span>,
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
            const listId = row.original.id;

            const { refetch } = useGetListsQuery(namespaceId);
            
            const [ deleteList, { isLoading: isDeleting, error: deletionError } ] = useDeleteListMutation();

            const deleteListHandler = async (id: string) => {
                if (deletionError) {
                    return <div>Error deleting the list</div>
                }
            
                await deleteList({ namespaceId, listId: id });
                refetch();
            }
            

            return (
                <div className='flex space-x-4 text-primary items-center'>
                        {/* <button className='transition-all duration-300 ease-in-out hover:scale-105'><Edit3 size={16} /></button> */}
                        <Modal 
                            triggerButton={<Edit3 
                                size={20} 
                                strokeWidth={1.5} 
                                className='text-lime-400 hover:text-lime-500'
                            />} 
                            dialogBody={<EditListForm listId={listId}/>}
                            dialogTitle={"Edit Template"}
                            dialogDescription={"Edit your template"}
                        />
                        <button 
                            className='transition-all duration-300 ease-in-out hover:scale-105'
                            onClick={() => deleteListHandler(listId)}
                            >
                            <Trash2 strokeWidth={1.5} size={20} className='text-red-400 hover:text-red-500' />
                    </button>
                </div>
            )
        }
            
    },
  ]

export default columns