import React, { useEffect } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormField,
  FormControl,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";
import { Checkbox } from "@/components/ui/checkbox";
import {
  useAddContactMutation,
  useUpdateContactMutation,
  useLazyCheckEmailQuery,
} from "@/app/dashboard/contacts/contactApi";

import { useAddContactsToListMutation } from "@/app/services/ListApi"; // Adjust import path as necessary

import { Contact } from "@/lib/type/contact";
import { ContactFormSchema } from "@/lib/type/contact";

interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

interface AddContactFormProps {
  open?: boolean;
  onClose: () => void;
  contactData?: Contact;
  lists: List[] | null;
}

const AddContactForm: React.FC<AddContactFormProps> = ({
  open,
  onClose,
  contactData,
  lists,
}) => {
  console.log(lists);
  const form = useForm<z.infer<typeof ContactFormSchema>>({
    resolver: zodResolver(ContactFormSchema),
    mode: "onChange",
    reValidateMode: "onChange",
    defaultValues: contactData
      ? {
          email: contactData.email,
          name: `${contactData.first_name || ""} ${
            contactData.last_name || ""
          }`.trim(),
          listId: contactData.listId || "",
          attributes: contactData.attributes || "{}",
          preconfirm: contactData.preconfirm || false,
          updated_at: contactData.updated_at,
          created_at: contactData.created_at,
        }
      : {
          email: "",
          name: "",
          status: "Enabled",
          listId: "",
          attributes: "{}",
          preconfirm: false,
        },
  });

  const [addContact, { isLoading }] = useAddContactMutation();
  const [updateContact, { isLoading: isUpdating }] = useUpdateContactMutation();
  const [addContactsToList] = useAddContactsToListMutation();

  const [triggerCheckEmail] = useLazyCheckEmailQuery();

  useEffect(() => {
    const subscription = form.watch((value, { name }) => {
      if (name === "email") {
        const email = value.email;
        const isValidEmail = z.string().email().safeParse(email).success;

        if (!isValidEmail || contactData) return; // Skip if invalid format

        const handler = setTimeout(() => {
          if (email) {
            triggerCheckEmail(email)
              .unwrap()
              .then((exists) => {
                if (exists) {
                  form.setError("email", {
                    type: "manual",
                    message: "Email already exists",
                  });
                } else {
                  form.clearErrors("email");
                }
              })
              .catch((error) => {
                console.error("Email check failed:", error);
              });
          }
        }, 500);

        return () => clearTimeout(handler);
      }
    });

    return () => subscription.unsubscribe();
  }, [form, triggerCheckEmail, contactData]);

  async function onSubmit(values: z.infer<typeof ContactFormSchema>) {
    try {
      console.log("Submitting form with values:", values);

      const nameParts = (values.name || "").split(" ");
      const payload = {
        email: values.email,
        first_name: nameParts[0] || "",
        last_name: nameParts.slice(1).join(" ") || "",
        listId: values.listId,
        attribute: values.attributes || "{}",
        created: new Date().toISOString(),
        updated: new Date().toISOString(),
      };

      if (contactData) {
        await updateContact({ id: contactData.id, data: payload }).unwrap();
      } else {
        // Get the response and handle array format
        const response = await addContact(payload).unwrap();

        // Define the expected contact shape
        interface Contact {
          id: string;
          first_name: string;
          last_name: string;
          email: string;
          attribute: string;
        }

        // Extract the contact from the array response
        let contactResponse: Contact;

        if (Array.isArray(response) && response.length > 0) {
          contactResponse = response[0]; // Take the first contact from the array
        } else if ("id" in response) {
          contactResponse = response as Contact; // Handle single object response
        } else {
          throw new Error("Invalid API response format");
        }

        // Validate the contact has an ID
        if (!contactResponse.id) {
          throw new Error("Contact ID missing in API response");
        }

        // Add contact to list
        if (values.listId) {
          try {
            await addContactsToList({
              listId: values.listId,
              contacts: [{ id: contactResponse.id }],
            }).unwrap();

            console.log("Successfully added contact to list");
          } catch (listError) {
            console.error("Error adding contact to list:", listError);
            throw listError;
          }
        }
      }
      form.reset();
      onClose?.();
    } catch (error) {
      console.error("Form submission error:", error);
      throw error;
    }
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-xl font-medium">
            {contactData ? "Edit Contact" : "New Subscriber"}
          </DialogTitle>
          <DialogDescription className="text-sm text-muted-foreground mt-1">
            {contactData
              ? "Edit the details of the contact."
              : "Add a new subscriber to your mailing list."}
          </DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            {/* Email Field */}
            <FormField
              control={form.control}
              name="email"
              render={({ field }) => (
                <FormItem>
                  <div className="text-sm font-medium">E-mail</div>
                  <FormControl>
                    <Input
                      className="mt-1.5"
                      placeholder="Enter email"
                      {...field}
                      onChange={(e) => {
                        field.onChange(e); // Ensure real-time validation
                        form.trigger("email");
                      }}
                    />
                  </FormControl>
                  <FormMessage className="text-xs text-red-500" />
                </FormItem>
              )}
            />

            {/* Name Field */}
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <div className="text-sm font-medium">Name</div>
                  <FormControl>
                    <Input
                      className="mt-1.5"
                      placeholder="Enter full name"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage className="text-xs" />
                </FormItem>
              )}
            />

            {/* Status Dropdown */}
            <FormField
              control={form.control}
              name="status"
              render={({ field }) => (
                <FormItem>
                  <div className="text-sm font-medium">Status</div>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger className="mt-1.5">
                        <SelectValue placeholder="Select Status" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      <SelectItem value="Enabled">Enabled</SelectItem>
                      <SelectItem value="Disabled">Disabled</SelectItem>
                    </SelectContent>
                  </Select>
                  <FormMessage className="text-xs" />
                </FormItem>
              )}
            />

            {/* List Select */}
            <FormField
              control={form.control}
              name="listId"
              render={({ field }) => (
                <FormItem>
                  <div className="text-sm font-medium">List</div>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger className="mt-1.5">
                        <SelectValue placeholder="Select List" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {lists?.map((list) => (
                        <SelectItem key={list.id} value={list.id}>
                          {list.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormMessage className="text-xs" />
                </FormItem>
              )}
            />

            {/* Preconfirm Checkbox */}
            <FormField
              control={form.control}
              name="preconfirm"
              render={({ field }) => (
                <FormItem className="flex flex-row items-start space-x-2 rounded-md border p-2">
                  <FormControl>
                    <Checkbox
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                  <div className="text-sm">
                    <label
                      htmlFor="preconfirm"
                      className="font-medium leading-none"
                    >
                      Preconfirm subscriptions
                    </label>
                    <p className="text-xs text-muted-foreground mt-1">
                      Dont send opt-in e-mails and mark all list subscriptions
                      as subscribed.
                    </p>
                  </div>
                </FormItem>
              )}
            />

            {/* Attributes Field */}
            <FormField
              control={form.control}
              name="attributes"
              render={({ field }) => (
                <FormItem>
                  <div className="text-sm font-medium">Attributes</div>
                  <FormControl>
                    <textarea
                      className="mt-1.5 w-full min-h-[80px] rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                      placeholder='{"key": "value"}'
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />

            {/* Submit Buttons */}
            <DialogFooter>
              <Button type="button" variant="outline" onClick={onClose}>
                Cancel
              </Button>
              <Button
                type="submit"
                disabled={isLoading || isUpdating}
                className="bg-blue-600 hover:bg-blue-700"
              >
                {isLoading || isUpdating ? "Saving..." : "Save"}
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};

export { AddContactForm };
