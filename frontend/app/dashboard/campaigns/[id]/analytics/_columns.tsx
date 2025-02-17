'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import { Edit3, Trash2 } from 'lucide-react';
import Link from 'next/link';
import ConfirmationPopup from '@/components/common/ConfirmationPopup';

export const columns = (
    deleteCampaignHandler: (id: string) => void,
): ColumnDef<any>[] => [
    {
        accessorKey: "id",
        header: "Id",
        cell: ({ row }) => <Link href={`/`}>{row.getValue("id")}</Link>,
    },
    {
        accessorKey: "status",
        header: "Status",
        cell: ({ row }) => <span>{row.getValue("status")}</span>,
    },
    {
        accessorKey: "sent_at",
        header: "Sent",
        cell: ({ row }) => <span>{formatDate(row.getValue("sent_at"))}</span>,
    },
    {
        accessorKey: "actions",
        header: "",
        cell: ({ row }) => {
            const campaignId = row.original.id;

            return (
                <div className='flex space-x-4 text-primary items-center'>
                    <Link href={`/dashboard/campaigns/new`}>
                        <Edit3 
                            size={20} 
                            strokeWidth={1.5} 
                            className='text-lime-400 hover:text-lime-500 transition-all duration-300 ease-in-out hover:scale-105'
                        />
                    </Link>
                    <ConfirmationPopup
                        title="Are you sure to delete this campaign?"
                        message="The campaign will be deleted permanently."
                        onConfirm={() => deleteCampaignHandler(campaignId)}
                        confirmButton = {
                            <Trash2 strokeWidth={1.5} size={20} className='text-red-400 hover:text-red-500 transition-all duration-300 ease-in-out hover:scale-105 cursor-pointer' />
                        }
                    />
                </div>
            );
        }     
    },
];

export default columns;
