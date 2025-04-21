"use client";

import React, { useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { Edit3, Trash2 } from "lucide-react";
import { Checkbox } from "@/components/ui/checkbox";
import { Contact } from "@/lib/type/contact";
import { EditContact } from "./_components/ContactForms/EditContact";
import Link from "next/link";

interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

interface ContactActionsProps {
  contactData: Contact;
  lists: List[];
}

const ContactActions = ({ contactData, lists }: ContactActionsProps) => {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="flex space-x-4 items-center">
      <button onClick={() => setIsOpen(true)}>
        <Edit3 className="w-5 h-5 text-blue-500" />
      </button>
      <EditContact
        open={isOpen}
        onClose={() => setIsOpen(false)}
        contactData={contactData}
        lists={lists}
      />
    </div>
  );
};

export const createColumns = (
  deleteHandler: (id: string) => Promise<void>,
  lists: List[],
  selectedContacts: Record<string, boolean>,
  setSelectedContacts: React.Dispatch<
    React.SetStateAction<Record<string, boolean>>
  >
): ColumnDef<Contact, unknown>[] => [
  {
    id: "select",
    enableSorting: false,
    header: ({ table }) => (
      <Checkbox
        checked={
          Object.keys(selectedContacts).length > 0 &&
          Object.keys(selectedContacts).length ===
            table.getFilteredRowModel().rows.length
        }
        onCheckedChange={(value) => {
          const isChecked = Boolean(value);
          setSelectedContacts(
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
        checked={selectedContacts[row.original.id] || false}
        onCheckedChange={(value) =>
          setSelectedContacts((prev) => ({
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
    cell: ({ row }) => (
      <div>
        <Link 
          href={`/dashboard/contacts/${row.original.id}/mails`}
          className="text-blue-600 hover:underline cursor-pointer w-max"
        >
          {row.original.email}
        </Link>
        {Array.isArray(row.original.lists) && row.original.lists.length > 0 && (
          <div className="flex flex-wrap gap-2 mt-1">
            {row.original.lists.map((list, index) => (
              <span
                key={index}
                className="px-2 py-1 text-xs rounded-full bg-gray-200 text-gray-700"
              >
                {list.list_name}
              </span>
            ))}
          </div>
        )}
      </div>
    ),
  },
  {
    id: "full_name",
    header: () => "Name",
    accessorFn: (row) =>
      `${row.first_name || ""} ${row.last_name || ""}`.trim(),
    cell: ({ row }) => <span>{row.getValue("full_name")}</span>,
  },
  {
    accessorKey: "created_at",
    header: () => "Created",
    cell: ({ row }) => (
      <span>{new Date(row.getValue("created_at")).toLocaleDateString()}</span>
    ),
  },
  {
    accessorKey: "updated_at",
    header: () => "Updated",
    cell: ({ row }) => (
      <span>{new Date(row.getValue("updated_at")).toLocaleDateString()}</span>
    ),
  },
  {
    id: "actions",
    enableSorting: false,
    header: () => "Actions",
    cell: ({ row }) => {
      const contactId = row.original.id;
      return (
        <div className="flex space-x-4 text-primary items-center">
          <ContactActions contactData={row.original} lists={lists} />
          <button
            className="transition-all duration-300 ease-in-out hover:scale-105"
            onClick={() => deleteHandler(contactId)}
          >
            <Trash2 size={20} strokeWidth={1.5} className="text-red-400" />
          </button>
        </div>
      );
    },
  },
];
