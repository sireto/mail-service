'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate, formatDateWithoutDay } from '@/lib/utils';
import Link from 'next/link';
import ActionsColumn from './_components/ActionsColumn';
import ToolTip from '@/components/common/ToolTip';

const statusTagStyleMap = {
    draft: 'bg-gray-100 text-gray-800',
    pending: 'bg-yellow-100 text-yellow-800',
    sent: 'bg-blue-100 text-blue-800',
    bounced: 'bg-red-100 text-red-800',
}

export const columns = (
    deleteMailHandler: (id: string) => void,
): ColumnDef<any>[] => [
    {
        accessorKey: "email",
        header: "Email",
        cell: ({ row }) => <Link href={`/`}>{row.getValue("email")}</Link>,
        // the Link here can be useful to navigate to that specific contact mail log history...
    },
    {
        accessorKey: "status",
        header: "Status",
        cell: ({ row }) => {
            const status = row.getValue("status") as keyof typeof statusTagStyleMap;
            const bounceReason: string | null = row.original.status_reason ?? "No reason available";

            const statusStyle = statusTagStyleMap[status] || 'bg-gray-100 text-gray-800';
            return <ToolTip
                message={bounceReason ?? "Not bounced"}
                tooltipTrigger={<span className={`px-2 py-1 rounded-full text-xs ${statusStyle}`}>{status}</span>}
            />
        },
    },
    {
        accessorKey: "open",
        header: ({ table }) => {
            const rows = table.getRowModel().rows;

            const viewCount = rows.filter(row => row.getValue("open")).length;
            return `Views (${viewCount})`;
        },
        cell: ({ row }) => {
            const isOpened = row.getValue("open");
            const reason: string | null = isOpened ? "Mail has been viewed" : "Mail hasn't been viewed";

            return <ToolTip
                message={reason}
                tooltipTrigger={<div>
                    <span className={`block w-2 h-2 rounded-full ${isOpened ? 'bg-success' : 'bg-warning'}`}></span>
                </div>}
            />
        },
    },
    {
        accessorKey: "clicks",
        header: "Clicks",
        cell: ({ row }) => {
            const numberOfClicks: number = row.getValue('clicks');

            return <span 
                className={`px-2 py-1 rounded-full text-xs ${numberOfClicks > 0 ? 'opacity-100 font-bold' : 'opacity-60'}`}
            >{numberOfClicks}</span>
        },
    },
    {
        accessorKey: "sent_at",
        header: "Sent",
        cell: ({ row }) => {
            const { localDate, localTime } = formatDateWithoutDay(row.getValue("sent_at"));
            return <ToolTip
            message={localTime}
            tooltipTrigger={<span className='text-xs'>{localDate}</span>}
        />
        }
    },
    {
        accessorKey: "actions",
        header: "",
        cell: ({ row }) => <ActionsColumn row={row} deleteMailHandler={deleteMailHandler}/>
    },
];

export default columns;