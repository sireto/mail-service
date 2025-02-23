"use client";

import DataTable from "@/components/DataTable";
import { Button } from "@/components/ui/button";
import { useState, useMemo } from "react";
import {
  useGetContactsQuery,
  useDeleteContactMutation,
} from "@/app/services/ContactApi";
import { useGetListsQuery } from "@/app/services/ListApi";
import { createColumns } from "@/app/dashboard/contacts/_columns";
import { Download, Trash2 } from "lucide-react";
import NoContactsFound from "./NotFound";
import { AddContact } from "./_components/ContactForms/AddContact";
import { Contact } from "@/lib/type/contact";
import { ColumnDef } from "@tanstack/react-table";

const NAMESPACE_ID = "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82";

interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

const ContactsPage = () => {
  const [isOpen, setIsOpen] = useState(false);
  const { data: contacts, isLoading, isError } = useGetContactsQuery();
  const { data: listsData, isLoading: listsLoading } =
    useGetListsQuery(NAMESPACE_ID);
  console.log("Initial contacts data:", contacts);
  const lists: List[] = listsData ?? [];
  const [deleteContact] = useDeleteContactMutation();
  const [selectedContacts, setSelectedContacts] = useState<
    Record<string, boolean>
  >({});
  const [searchTerm, setSearchTerm] = useState("");

  const selectedCount = Object.values(selectedContacts).filter(Boolean).length;

  const handleDeleteContact = async (id: string): Promise<void> => {
    await deleteContact(id).unwrap();
  };

  const handleBulkDelete = async () => {
    const contactIds = Object.keys(selectedContacts).filter(
      (id) => selectedContacts[id]
    );
    if (contactIds.length === 0) return;

    try {
      await Promise.all(contactIds.map((id) => deleteContact(id).unwrap()));
      setSelectedContacts({});
    } catch (error) {
      console.error("Error deleting contacts:", error);
    }
  };

  const handleExport = () => {
    const selectedData = contacts?.filter(
      (contact) => selectedContacts[contact.id]
    );
    if (!selectedData?.length) return;

    const json = JSON.stringify(selectedData, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "contacts.json";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const filteredContacts: Contact[] = useMemo(() => {
    if (!contacts) return [];
    const lowerSearch = searchTerm.toLowerCase();
    return contacts.filter((contact) => {
      const emailMatch = contact.email.toLowerCase().includes(lowerSearch);
      const fullName = `${contact.first_name || ""} ${
        contact.last_name || ""
      }`.toLowerCase();
      return emailMatch || fullName.includes(lowerSearch);
    });
  }, [contacts, searchTerm]);

  const tableColumns = createColumns(
    handleDeleteContact,
    lists,
    selectedContacts,
    setSelectedContacts
  ) as unknown as ColumnDef<Contact, unknown>[];

  if (isError) {
    return <NoContactsFound lists={lists} />;
  }

  if (isLoading || listsLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div className="p-6">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-2xl font-semibold">
            Contacts{" "}
            <span className="text-gray-500">({contacts?.length || 0})</span>
          </h1>
          <Button
            variant="default"
            className="bg-blue-600 hover:bg-blue-700"
            onClick={() => setIsOpen(true)}
          >
            + New
          </Button>
        </div>
        <div className="flex justify-between items-center">
          <input
            type="text"
            placeholder="Search by name or email..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full max-w-md px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:blue-500"
          />
        </div>
      </div>

      {selectedCount > 0 && (
        <div className="flex">
          <Button
            variant="ghost"
            size="sm"
            onClick={handleExport}
            className="flex items-center gap-2"
          >
            <Download size={16} />
            Export
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={handleBulkDelete}
            className="flex items-center gap-2 text-red-600 hover:text-red-700"
          >
            <Trash2 size={16} />
            Delete
          </Button>
        </div>
      )}
      <AddContact
        open={isOpen}
        onClose={() => setIsOpen(false)}
        lists={lists}
      />

      <div className="p-6">
        <DataTable
          data={filteredContacts || []}
          columns={tableColumns}
          fallback="No Contacts Found"
        />
      </div>
    </div>
  );
};

export default ContactsPage;