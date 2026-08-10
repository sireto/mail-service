'use client';

import React from 'react'
import { ColumnDef } from '@tanstack/react-table';
import { formatDate } from '@/lib/utils';
import ActionsColumn from './_components/ActionsColumn';
import { Template } from '@/lib/type/template';


export const columns: ColumnDef<Template>[] = [
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
        cell: ({ row }) => <ActionsColumn row={row} />,
    },
  ]

export default columns;