"use client";

import React, { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { EditCampaignSenderFormSchemaDTO } from "@/lib/type";
import { useUpdateCampaignSenderMutation } from "@/app/services/CampaignSenderApi";
import { ContactDialog } from "@/app/dashboard/contacts/_components/ContactForms/ContactDialog";
import { useValidateEmailIdentityMutation } from "@/app/services/EmailIdentityApi";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";

interface EditCampaignSenderProps {
  open?: boolean;
  onClose: () => void;
  senderData: {
    id: string;
    from_name: string;
    from_email: string;
    server_id: string;
    created_at: string;
    updated_at: string;
  };
}

const EditCampaignSender: React.FC<EditCampaignSenderProps> = ({
  open,
  onClose,
  senderData,
}) => {
  const [emailValidated, setEmailValidated] = useState(true); // Default to true for existing emails
  const [emailError, setEmailError] = useState("");
  const [validateEmail, { isLoading: isValidating }] =
    useValidateEmailIdentityMutation();

  const form = useForm<z.infer<typeof EditCampaignSenderFormSchemaDTO>>({
    resolver: zodResolver(EditCampaignSenderFormSchemaDTO),
    mode: "onChange",
    defaultValues: {
      from_name: senderData?.from_name || "",
      from_email: senderData?.from_email || "",
    },
  });

  const [updateSender, { isLoading: isUpdating }] =
    useUpdateCampaignSenderMutation();

  const onSubmit = async (
    values: z.infer<typeof EditCampaignSenderFormSchemaDTO>
  ) => {
    try {
      // Check if email is validated when it's different from the original
      if (values.from_email !== senderData.from_email && !emailValidated) {
        setEmailError("Please validate the email first");
        return;
      }

      const payload = {
        from_name: values.from_name,
        from_email: values.from_email,
        updated_at: new Date().toISOString(),
      };

      await updateSender({
        senderId: senderData.id,
        updatedSender: payload,
      }).unwrap();
      form.reset();
      onClose();
    } catch (error) {
      console.error("Form submission error:", error);
      throw error;
    }
  };

  const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newEmail = e.target.value;
    form.setValue("from_email", newEmail);

    // Only require validation if the email has changed
    if (newEmail !== senderData.from_email) {
      setEmailValidated(false);
      setEmailError("");
    } else {
      setEmailValidated(true);
      setEmailError("");
    }
  };

  const validateEmailIdentity = async () => {
    const email = form.getValues("from_email");

    if (!email) {
      setEmailError("Email is required for validation");
      return;
    }

    try {
      const result = await validateEmail({
        email,
        serverId: senderData.server_id,
      }).unwrap();

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
      title="Edit Campaign Sender"
      description="Edit the details of the campaign sender."
      isSubmitting={isUpdating}
      onSubmit={handleSubmitClick}
    >
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
          {/* From Name Field */}
          <FormField
            control={form.control}
            name="from_name"
            render={({ field }) => (
              <FormItem>
                <div className="text-sm font-medium">From Name</div>
                <FormControl>
                  <Input
                    className="mt-1.5"
                    placeholder="Enter sender name"
                    {...field}
                  />
                </FormControl>
                <FormMessage className="text-xs text-red-500" />
              </FormItem>
            )}
          />

          {/* From Email Field with Validation Button */}
          <FormField
            control={form.control}
            name="from_email"
            render={({ field }) => (
              <FormItem>
                <div className="text-sm font-medium">From Email</div>
                <div className="flex space-x-2 mt-1.5">
                  <FormControl>
                    <Input
                      placeholder="Enter sender email"
                      value={field.value}
                      onChange={handleEmailChange}
                      className={emailValidated ? "border-green-500" : ""}
                    />
                  </FormControl>
                  {field.value !== senderData.from_email && (
                    <button
                      type="button"
                      onClick={validateEmailIdentity}
                      disabled={isValidating || !field.value}
                      className="px-3 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300"
                    >
                      {isValidating ? "Validating..." : "Validate"}
                    </button>
                  )}
                </div>
                {emailError && (
                  <p className="text-xs text-red-500 mt-1">{emailError}</p>
                )}
                {emailValidated && field.value !== senderData.from_email && (
                  <p className="text-xs text-green-500 mt-1">Email verified!</p>
                )}
                <FormMessage className="text-xs text-red-500" />
              </FormItem>
            )}
          />
        </form>
      </Form>
    </ContactDialog>
  );
};

export { EditCampaignSender };
