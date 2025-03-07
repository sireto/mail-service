"use client";

import React from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { EditCampaignSenderFormSchemaDTO } from "@/lib/type";
import { useUpdateCampaignSenderMutation } from "@/app/services/CampaignSenderApi";
import { ContactDialog } from "@/app/dashboard/contacts/_components/ContactForms/ContactDialog";
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
    created_at: string;
    updated_at: string;
  };
}

const EditCampaignSender: React.FC<EditCampaignSenderProps> = ({
  open,
  onClose,
  senderData,
}) => {
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

          {/* From Email Field */}
          <FormField
            control={form.control}
            name="from_email"
            render={({ field }) => (
              <FormItem>
                <div className="text-sm font-medium">From Email</div>
                <FormControl>
                  <Input
                    className="mt-1.5"
                    placeholder="Enter sender email"
                    {...field}
                    onChange={(e) => {
                      field.onChange(e);
                      form.trigger("from_email");
                    }}
                  />
                </FormControl>
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
