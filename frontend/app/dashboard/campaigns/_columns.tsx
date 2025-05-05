"use client";

import React from "react";
import { ColumnDef } from "@tanstack/react-table";
import { formatDate } from "@/lib/utils";
import Link from "next/link";
import ActionsColumn from "./_components/ActionsColumn";
import { ListDTO } from "@/lib/type";
import { z } from "zod";
import { Campaign } from "@/lib/type/campaign";

export const columns = (
  deleteCampaignHandler: (id: string) => void,
  startCampaignHandler: (id: string) => void
): ColumnDef<Campaign>[] => [
  {
    accessorKey: "campaign_name",
    header: "Name",
    cell: ({ row }) => (
      <Link href={`/dashboard/campaigns/${row.original.id}`}>
        {row.getValue("campaign_name")}
      </Link>
    ),
  },
  {
    accessorKey: "lists",
    header: "List",
    cell: ({ row }) => {
      const lists = row.getValue("lists") as z.infer<typeof ListDTO>[];

      const listTags = lists.map((list, idx) => (
        <span
          key={idx}
          className="px-2 py-1 text-xs rounded-full bg-gray-200 text-gray-700"
        >
          {list.name}
        </span>
      ));

      return <div className="flex flex-wrap gap-2 mt-1">{listTags}</div>;
    },
  },
  {
    accessorKey: "created_at",
    header: "Created",
    cell: ({ row }) => <span>{formatDate(row.getValue("created_at"))}</span>,
  },
  {
    accessorKey: "actions",
    header: "",
    cell: ({ row }) => (
      <ActionsColumn
        row={row}
        startCampaignHandler={startCampaignHandler}
        deleteCampaignHandler={deleteCampaignHandler}
      />
    ),
  },
];

export default columns;
