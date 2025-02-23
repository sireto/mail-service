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
