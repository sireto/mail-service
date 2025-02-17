'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import { Edit3, Trash2, Rocket } from 'lucide-react';
import Link from 'next/link';
import ConfirmationPopup from '@/components/common/ConfirmationPopup';

export const columns = (
    deleteCampaignHandler: (id: string) => void,
    startCampaignHandler: (id: string) => void
): ColumnDef<any>[] => [
    {
        accessorKey: "campaign_name",
        header: "Name",
        cell: ({ row }) => <Link href={`/dashboard/campaigns/${row.original.id}`}>{row.getValue("campaign_name")}</Link>,
    },
    {
        accessorKey: "status",
        header: "Status",
        cell: ({ row }) => <span>{row.getValue("status")}</span>,
    },
    {
        accessorKey: "list",
        header: "List",
        cell: ({ row }) => <span>{row.getValue("list")}</span>,
    },
    {
        accessorKey: "created_at",
        header: "Created",
        cell: ({ row }) => <span>{formatDate(row.getValue("created_at"))}</span>,
    },
    {
        accessorKey: "actions",
        header: "",
        cell: ({ row }) => {
            const campaignId = row.original.id;

            return (
                <div className='flex space-x-4 text-primary items-center'>
                    <ConfirmationPopup
                        title="Do you wanna start campaign?"
                        message="The campaign will start right away."
                        onConfirm={() => startCampaignHandler(campaignId)}
                        confirmButton = {
                            <Rocket 
                            size={20} 
                            strokeWidth={1.5}
                            className='transition-all duration-300 ease-in-out hover:scale-105 cursor-pointer' 
                            />
                        }
                    />
                    <Link href={`/dashboard/campaigns/${campaignId}`}>
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
