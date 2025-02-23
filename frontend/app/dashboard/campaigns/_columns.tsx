'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import Link from 'next/link';
import ActionsColumn from './_components/ActionsColumn';


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
        cell: ({ row }) => <ActionsColumn row={row} startCampaignHandler={startCampaignHandler} deleteCampaignHandler={deleteCampaignHandler}/>
    },
];

export default columns;
