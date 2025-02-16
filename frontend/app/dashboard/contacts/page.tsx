"use client";

import { AddContactForm } from "@/components/AddContactForm";
import DataTable from "@/components/DataTable";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { useGetContactsQuery, useDeleteContactMutation } from "./contactApi";
import { useGetListsQuery } from "@/app/services/ListApi";
import { createColumns } from "./columns";

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
  const { data: contacts, isLoading, isError, refetch } = useGetContactsQuery();
  const { data: lists, isLoading: listsLoading } =
    useGetListsQuery(NAMESPACE_ID);
  const [deleteContact, { isLoading: isDeleting, error: deletionError }] =
    useDeleteContactMutation();

  const deleteContactHandler = async (id: string) => {
    if (deletionError) {
      console.error("Error deleting the contact:", deletionError);
      return;
    }
    await deleteContact(id);
    refetch();
  };

  if (isError) {
    return <div>Error loading contacts...</div>;
  }

  if (isLoading || listsLoading) {
    return <div>Loading...</div>;
  }
  console.log(lists);

  return (
    <div>
      <div className="p-6 flex justify-between">
        <h1 className="text-2xl font-bold">
          Contacts
          <span className="ml-2">({contacts?.length || 0})</span>
        </h1>
        <Button onClick={() => setIsOpen(true)}>+ New</Button>
      </div>
      <AddContactForm
        open={isOpen}
        onClose={() => setIsOpen(false)}
        lists={lists}
      />
      <DataTable
        data={contacts || []}
        columns={createColumns(deleteContactHandler, lists)}
        fallback="No Contacts Found"
      />
    </div>
  );
};

export default ContactsPage;
