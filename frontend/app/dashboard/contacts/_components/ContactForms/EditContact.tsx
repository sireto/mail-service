import React from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Contact, ContactFormSchema } from "@/lib/type/contact";
import { useUpdateContactMutation } from "@/app/services/ContactApi";
import {
  useAddContactsToListMutation,
  useRemoveContactFromListMutation,
} from "@/app/services/ListApi";
import { ContactDialog } from "./ContactDialog";
import { ContactFormFields } from "./ContactFormFields";
import { List } from "@/lib/type";

interface EditContactProps {
  open?: boolean;
  onClose: () => void;
  contactData: Contact;
  lists: List[] | null;
}

const EditContact: React.FC<EditContactProps> = ({
  open,
  onClose,
  contactData,
  lists,
}) => {
  // Get current list IDs from contact data
  const currentListIds = React.useMemo(() => {
    return contactData?.lists.map((list) => list.list_id) || [];
  }, [contactData?.lists]);

  const form = useForm<z.infer<typeof ContactFormSchema>>({
    resolver: zodResolver(ContactFormSchema),
    mode: "onChange",
    reValidateMode: "onChange",
    defaultValues: {
      email: contactData?.email,
      name: `${contactData?.first_name || ""} ${
        contactData?.last_name || ""
      }`.trim(),
      listIds: currentListIds,
      attribute:
        typeof contactData?.attribute === "string"
          ? contactData.attribute
          : JSON.stringify(contactData?.attribute || {}),
      preconfirm: contactData?.preconfirm || false,
      updated_at: contactData?.updated_at,
      created_at: contactData?.created_at,
    },
  });

  const [updateContact, { isLoading: isUpdating }] = useUpdateContactMutation();
  const [addContactsToList] = useAddContactsToListMutation();
  const [removeContactFromList] = useRemoveContactFromListMutation();

  const handleListChange = (selectedLists: string[]) => {
    form.setValue("listIds", selectedLists);
  };

  async function onSubmit(values: z.infer<typeof ContactFormSchema>) {
    try {
      const nameParts = (values.name || "").split(" ");
      const payload = {
        email: values.email,
        first_name: nameParts[0] || "",
        last_name: nameParts.slice(1).join(" ") || "",
        attribute: values.attribute || "{}",
        updated: new Date().toISOString(),
      };

      await updateContact({ id: contactData.id, data: payload }).unwrap();

      const listIds = values.listIds ?? [];

      const listsToAdd = listIds.filter(
        (listId) => !currentListIds.includes(listId),
      );

      const listsToRemove = currentListIds.filter(
        (listId) => !listIds.includes(listId),
      );

      for (const listId of listsToAdd) {
        try {
          await addContactsToList({
            listId,
            contacts: [{ id: contactData.id }],
          }).unwrap();
        } catch (listError) {
          console.error(`Error adding to list ${listId}:`, listError);
        }
      }

      // Remove contact from unselected lists
      for (const listId of listsToRemove) {
        try {
          await removeContactFromList({
            listId,
            contacts: [{ id: contactData.id }],
          }).unwrap();
        } catch (listError) {
          console.error(`Error removing from list ${listId}:`, listError);
        }
      }

      form.reset();
      onClose?.();
    } catch (error) {
      console.error("Form submission error:", error);
      throw error;
    }
  }

  const handleSubmitClick = () => {
    form.handleSubmit(onSubmit)();
  };

  return (
    <ContactDialog
      open={open}
      onClose={onClose}
      title="Edit Contact"
      description="Edit the details of the contact."
      isSubmitting={isUpdating}
      onSubmit={handleSubmitClick}
    >
      <ContactFormFields
        form={form}
        lists={lists}
        handleListChange={handleListChange}
        onSubmit={onSubmit}
      />
    </ContactDialog>
  );
};

export { EditContact };
