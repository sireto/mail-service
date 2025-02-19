"use client";

import { AddContactForm } from "@/components/AddContactForm";
import DataTable from "@/components/DataTable";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import {
  useGetContactsQuery,
  useDeleteContactMutation,
} from "@/app/services/ContactApi";
import { useGetListsQuery } from "@/app/services/ListApi";
import { createColumns } from "./columns";
import { Download, Trash2 } from "lucide-react";
import NoContactsFound from "./NotFound";

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

  if (isError) {
    return <NoContactsFound lists={lists} />;
  }

  if (isLoading || listsLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div className="p-6 flex justify-between items-center">
        <div className="flex items-center gap-4">
          <h1 className="text-2xl font-semibold">
            Contacts{" "}
            <span className="text-gray-500">({contacts?.length || 0})</span>
          </h1>
          {selectedCount > 0 && (
            <div className="flex items-center gap-2 text-sm text-gray-500">
              {selectedCount} subscriber(s) selected —{" "}
              <button
                className="text-blue-600 hover:underline"
                onClick={() =>
                  setSelectedContacts(
                    Object.fromEntries(contacts?.map((c) => [c.id, true]) ?? [])
                  )
                }
              >
                Select all {contacts?.length}
              </button>
            </div>
          )}
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="default"
            className="bg-blue-600 hover:bg-blue-700"
            onClick={() => setIsOpen(true)}
          >
            + New
          </Button>
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
      <AddContactForm
        open={isOpen}
        onClose={() => setIsOpen(false)}
        lists={lists}
      />

      <div className="p-6">
        <DataTable
          data={contacts || []}
          columns={createColumns(
            handleDeleteContact,
            lists,
            selectedContacts,
            setSelectedContacts
          )}
          fallback="No Contacts Found"
        />
      </div>
    </div>
  );
};

export default ContactsPage;
