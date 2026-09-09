"use client";

import DataTable from "@/components/DataTable";
import { Button } from "@/components/ui/button";
import { useState, Suspense } from "react";
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
import { useSearchParams } from "next/navigation";
import { NAMESPACE_ID } from "@/config/namespace";



interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

const ContactsPageContent = () => {
  const [isOpen, setIsOpen] = useState(false);
  // const { data: contacts, isLoading, isError } = useGetContactsQuery({});
  const { data: listsData, isLoading: listsLoading } =
    useGetListsQuery(NAMESPACE_ID);
  // console.log("Initial contacts data:", contacts);
  const lists: List[] = listsData ?? [];
  const [deleteContact] = useDeleteContactMutation();
  const [selectedContacts, setSelectedContacts] = useState<
    Record<string, boolean>
  >({});
  const [searchTerm, setSearchTerm] = useState("");

  const searchParams = useSearchParams();
  const listId = searchParams.get("list_id") ?? undefined;

  const [offset, setOffset] = useState(0);
  const {
    data: contacts,
    isLoading,
    isError,
  } = useGetContactsQuery({
    list_id: listId,
    search: searchTerm,
    offset,
  });

  const selectedCount = Object.values(selectedContacts).filter(Boolean).length;

  const handleDeleteContact = async (id: string): Promise<void> => {
    await deleteContact(id).unwrap();
  };

  const handleBulkDelete = async () => {
    const contactIds = Object.keys(selectedContacts).filter(
      (id) => selectedContacts[id],
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
    // Only exports the selection visible on this page. Whole-table export lives on
    // /dashboard/contacts/export, which walks every page.
    const selectedData = contacts?.items.filter(
      (contact) => selectedContacts[contact.id],
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

  const tableColumns = createColumns(
    handleDeleteContact,
    lists,
    selectedContacts,
    setSelectedContacts,
  ) as unknown as ColumnDef<Contact, unknown>[];

  if (isError) {
    return <NoContactsFound lists={lists} />;
  }

  if (isLoading || listsLoading) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div>
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-2xl font-semibold">
            Contacts{" "}
            <span className="text-gray-500">({contacts?.total ?? 0})</span>
          </h1>
          <Button
            variant="default"
            className="bg-blue-600 hover:bg-blue-700"
            onClick={() => setIsOpen(true)}
          >
            + New
          </Button>
        </div>
        <div className="flex items-center">
          <input
            type="text"
            placeholder="Search by name or email..."
            value={searchTerm}
            onChange={(e) => {
              setSearchTerm(e.target.value);
              // A new search starts at the first page; keeping the offset would show an
              // empty page 3 of a smaller result set.
              setOffset(0);
            }}
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

      <div className="py-6 mb-6">
        <DataTable
          data={contacts?.items ?? []}
          columns={tableColumns}
          pagination={
            contacts ? { page: contacts, onOffsetChange: setOffset } : undefined
          }
          fallback="No Contacts Found"
        />
      </div>
    </div>
  );
};

const ContactsPage = () => {
  return (
    <Suspense fallback={<div>Loading contacts...</div>}>
      <ContactsPageContent />
    </Suspense>
  );
};

export default ContactsPage;
