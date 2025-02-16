"use client";

import React, { useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { Edit3, Trash2 } from "lucide-react";
import { AddContactForm } from "@/components/AddContactForm";

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
      <AddContactForm
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
  lists: List[]
): ColumnDef<Contact>[] => [
  {
    accessorKey: "email",
    header: "Email",
    cell: ({ row }) => <span>{row.getValue("email")}</span>,
  },
  {
    header: "Name",
    cell: ({ row }) => {
      const firstName = row.original.first_name;
      const lastName = row.original.last_name;
      return <span>{`${firstName || ""} ${lastName || ""}`}</span>;
    },
  },
  {
    accessorKey: "created_at",
    header: "Created",
    cell: ({ row }) => (
      <span>{new Date(row.getValue("created_at")).toLocaleDateString()}</span>
    ),
  },
  {
    accessorKey: "updated_at",
    header: "Updated",
    cell: ({ row }) => (
      <span>{new Date(row.getValue("updated_at")).toLocaleDateString()}</span>
    ),
  },
  {
    accessorKey: "actions",
    header: "",
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
