"use client";

import React, { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { EditCampaignSenderFormSchemaDTO } from "@/lib/type";
import { useCreateCampaignSenderMutation } from "@/app/services/CampaignSenderApi";
import { useGetServersQuery } from "@/app/services/ServerApi"; // Import server query
import { ContactDialog } from "@/app/dashboard/contacts/_components/ContactForms/ContactDialog";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useValidateEmailIdentityMutation } from "@/app/services/EmailIdentityApi"; // We'll create this next

// Update schema to include server_id
const ExtendedCampaignSenderFormSchema = EditCampaignSenderFormSchemaDTO.extend(
  {
    server_id: z.string().uuid("Invalid server ID"),
  }
);

type FormValues = z.infer<typeof ExtendedCampaignSenderFormSchema>;

interface AddCampaignSenderProps {
  open?: boolean;
  onClose: () => void;
}

const AddCampaignSender: React.FC<AddCampaignSenderProps> = ({
  open,
  onClose,
}) => {
  const { data: servers, isLoading: isLoadingServers } = useGetServersQuery();
  const [validateEmail, { isLoading: isValidating }] =
    useValidateEmailIdentityMutation();
  const [emailValidated, setEmailValidated] = useState(false);
  const [emailError, setEmailError] = useState("");

  const form = useForm<FormValues>({
    resolver: zodResolver(ExtendedCampaignSenderFormSchema),
    mode: "onChange",
    defaultValues: {
      server_id: "",
      from_name: "",
      from_email: "",
    },
  });

  const [createSender, { isLoading: isCreating }] =
    useCreateCampaignSenderMutation();

  const onSubmit = async (values: FormValues) => {
    try {
      if (!emailValidated) {
        setEmailError("Please validate the email first");
        return;
      }

      const payload = {
        server_id: values.server_id,
        from_name: values.from_name,
        from_email: values.from_email,
      };
      await createSender(payload).unwrap();
      form.reset();
      onClose();
    } catch (error) {
      console.error("Form submission error:", error);
      throw error;
    }
  };

  const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEmailValidated(false);
    setEmailError("");
    form.setValue("from_email", e.target.value);
  };

  const validateEmailIdentity = async () => {
    const email = form.getValues("from_email");
    const serverId = form.getValues("server_id");

    if (!email || !serverId) {
      setEmailError("Both email and server must be selected to validate");
      return;
    }

    try {
      const result = await validateEmail({ email, serverId }).unwrap();
      if (result.isValid) {
        setEmailValidated(true);
        setEmailError("");
      } else {
        setEmailError(result.message || "Email is not verified in AWS SES");
      }
    } catch (error) {
      setEmailError("Failed to validate email. Please try again.");
      console.error("Email validation error:", error);
    }
  };

  const handleSubmitClick = () => {
    form.handleSubmit(onSubmit)();
  };

  return (
    <ContactDialog
      open={open}
      onClose={onClose}
      title="Add Campaign Sender"
      description="Add a new campaign sender."
      isSubmitting={isCreating}
      onSubmit={handleSubmitClick}
    >
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          {/* Server Selection Field */}
          <FormField
            control={form.control}
            name="server_id"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Server</FormLabel>
                <Select
                  onValueChange={field.onChange}
                  defaultValue={field.value}
                  disabled={isLoadingServers}
                >
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue placeholder="Select a server" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    {servers?.map((server) => (
                      <SelectItem key={server.id} value={server.id || ""}>
                        {server.id || "" || server.host}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* From Name Field */}
          <FormField
            control={form.control}
            name="from_name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>From Name</FormLabel>
                <FormControl>
                  <Input placeholder="Enter sender name" {...field} />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />

          {/* From Email Field with Validation Button */}
          <FormField
            control={form.control}
            name="from_email"
            render={({ field }) => (
              <FormItem>
                <FormLabel>From Email</FormLabel>
                <div className="flex space-x-2">
                  <FormControl>
                    <Input
                      placeholder="Enter sender email"
                      value={field.value}
                      onChange={handleEmailChange}
                      className={emailValidated ? "border-green-500" : ""}
                    />
                  </FormControl>
                  <button
                    type="button"
                    onClick={validateEmailIdentity}
                    disabled={isValidating || !field.value}
                    className="px-3 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300"
                  >
                    {isValidating ? "Validating..." : "Validate"}
                  </button>
                </div>
                {emailError && (
                  <p className="text-xs text-red-500 mt-1">{emailError}</p>
                )}
                {emailValidated && (
                  <p className="text-xs text-green-500 mt-1">Email verified!</p>
                )}
                <FormMessage />
              </FormItem>
            )}
          />
        </form>
      </Form>
    </ContactDialog>
  );
};

export { AddCampaignSender };
