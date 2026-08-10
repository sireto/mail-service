"use client";

import React from "react";
import { ColumnDef } from "@tanstack/react-table";
import { formatDate } from "@/lib/utils";
import ActionsColumn from "./_components/ActionsColumn";
import Link from "next/link";
import { List } from "@/lib/type/list";

const namespaceId = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

export const columns: ColumnDef<List>[] = [
  {
    accessorKey: "name",
    header: "Name",
    cell: ({ row }) => (
      <Link
        href={`/dashboard/contacts?list_id=${encodeURIComponent(
          row.original.id,
        )}`}
      >
        {row.getValue("name")}
      </Link>
    ),
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
    cell: ({ row }) => <ActionsColumn row={row} namespaceId={namespaceId} />,
  },
];

export default columns;
