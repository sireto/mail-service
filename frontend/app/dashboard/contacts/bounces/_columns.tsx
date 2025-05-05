'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDateWithoutDay } from '@/lib/utils';
import ActionsColumn from '@/app/dashboard/campaigns/[id]/analytics/_components/ActionsColumn';
import ToolTip from '@/components/common/ToolTip';
import CampaignName from './_components/CampaignName';
import { Checkbox } from '@/components/ui/checkbox';
import { Mail } from '@/lib/type/mail';


export const columns = (
    selectedBounces: Record<string, boolean>,
    setSelectedBounces: React.Dispatch<React.SetStateAction<Record<string, boolean>>>,
    deleteMailHandler: (id: string) => void,
): ColumnDef<Mail>[] => [
    {
        id: "select",
        enableSorting: false,
        header: ({ table }) => (
          <Checkbox
            checked={
              Object.keys(selectedBounces).length > 0 &&
              Object.keys(selectedBounces).length ===
                table.getFilteredRowModel().rows.length
            }
            
            onCheckedChange={(value) => {
              const isChecked = Boolean(value);
              setSelectedBounces(
                isChecked
                  ? Object.fromEntries(
                      table
                        .getFilteredRowModel()
                        .rows.map((row) => [row.original.id, true])
                    )
                  : {}
              );
            }}
          />
        ),
        cell: ({ row }) => (
          <Checkbox
            checked={selectedBounces[row.original.id] || false}
            onCheckedChange={(value) =>
              setSelectedBounces((prev) => ({
                ...prev,
                [row.original.id]: Boolean(value),
              }))
            }
          />
        ),
    },
    {
        accessorKey: "email",
        header: "Email",
        cell: ({ row }) => <span>{row.getValue("email")}</span>,
    },
    {
        accessorKey: "campaign",
        header: "Campaign",
        cell: ({ row }) => <CampaignName id={row.original.campaign_id}/>
    },
    {
        accessorKey: "from_name",
        header: "Source",
        cell: ({ row }) => <span>{row.getValue("from_name")}</span>,
    },
    {
        accessorKey: "status_reason",
        header: "Type",
        cell: ({ row }) => <span>{row.getValue("status_reason")}</span>
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