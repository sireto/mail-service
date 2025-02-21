'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
<<<<<<< HEAD
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
=======
import { formatDate } from '@/lib/utils';
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
<<<<<<< HEAD
        accessorKey: "id",
        header: "Id",
        cell: ({ row }) => <Link href={`/`}>{row.getValue("id")}</Link>,
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
=======
        accessorKey: "email",
        header: "Email",
        cell: ({ row }) => <Link href={`/`}>{row.getValue("email")}</Link>,
        // the Link here can be useful to navigate to that specific contact mail log history...
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
    },
    {
        accessorKey: "status",
        header: "Status",
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
        cell: ({ row }) => {
            const status = row.getValue("status") as keyof typeof statusTagStyleMap;
            const bounceReason: string | null = row.original.status_reason ?? "No reason available";

            const statusStyle = statusTagStyleMap[status] || 'bg-gray-100 text-gray-800';
            return <ToolTip
                message={bounceReason ?? "Not bounced"}
                tooltipTrigger={<span className={`px-2 py-1 rounded-full text-xs ${statusStyle}`}>{status}</span>}
            />
        },
<<<<<<< HEAD
=======
        cell: ({ row }) => <span>{row.getValue("status")}</span>,
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
=======
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
    },
    {
        accessorKey: "sent_at",
        header: "Sent",
<<<<<<< HEAD
        cell: ({ row }) => {
            const { localDate, localTime } = formatDateWithoutDay(row.getValue("sent_at"));
            return <ToolTip
            message={localTime}
            tooltipTrigger={<span className='text-xs'>{localDate}</span>}
        />
        }
=======
        cell: ({ row }) => <span>{formatDate(row.getValue("sent_at"))}</span>,
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
    },
    {
        accessorKey: "actions",
        header: "",
<<<<<<< HEAD
<<<<<<< HEAD
        cell: ({ row }) => <ActionsColumn row={row} deleteMailHandler={deleteMailHandler}/>
=======
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
>>>>>>> 0b443a3 (feat: add update and analytics to the campaigns page)
=======
        cell: ({ row }) => <ActionsColumn row={row} deleteMailHandler={deleteMailHandler}/>
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
    },
];

export default columns;
