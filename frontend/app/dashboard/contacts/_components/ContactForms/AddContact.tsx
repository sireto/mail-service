import React from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { ContactFormSchema } from "@/lib/type/contact";
import { useAddContactMutation } from "@/app/services/ContactApi";
import { useAddContactsToListMutation } from "@/app/services/ListApi";
import { ContactDialog } from "./ContactDialog";
import { ContactFormFields } from "./ContactFormFields";
import { useEmailValidation } from "../../hooks/useEmailValidator";
import { List } from "@/lib/type";

interface AddContactProps {
  open?: boolean;
  onClose: () => void;
  lists: List[] | null;
}

const AddContact: React.FC<AddContactProps> = ({ open, onClose, lists }) => {
  const form = useForm<z.infer<typeof ContactFormSchema>>({
    resolver: zodResolver(ContactFormSchema),
    mode: "onChange",
    reValidateMode: "onChange",
    defaultValues: {
      email: "",
      name: "",
      listIds: [],
      attribute: "{}",
      preconfirm: false,
    },
  });

  const [addContact, { isLoading }] = useAddContactMutation();
  const [addContactsToList] = useAddContactsToListMutation();

  useEmailValidation(form);

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
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        list_names: [],
      };

      const response = await addContact(payload).unwrap();
      const listIds = values.listIds ?? [];

      interface Contact {
        id: string;
        first_name: string;
        last_name: string;
        email: string;
        attribute: string;
      }

      let contactResponse: Contact;

      if (Array.isArray(response) && response.length > 0) {
        contactResponse = response[0];
      } else if ("id" in response) {
        contactResponse = response as Contact;
      } else {
        throw new Error("Invalid API response format");
      }

      if (!contactResponse.id) {
        throw new Error("Contact ID missing in API response");
      }

      for (const listId of listIds) {
        try {
          await addContactsToList({
            listId,
            contacts: [{ id: contactResponse.id }],
          }).unwrap();
        } catch (listError) {
          console.error(`Error adding to list ${listId}:`, listError);
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
      title="New Subscriber"
      description="Add a new subscriber to your mailing list."
      isSubmitting={isLoading}
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

export { AddContact };
