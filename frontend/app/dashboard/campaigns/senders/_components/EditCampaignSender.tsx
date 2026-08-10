"use client";

import React from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { EditCampaignSenderFormSchemaDTO } from "@/lib/type";
import { useUpdateCampaignSenderMutation } from "@/app/services/CampaignSenderApi";
import { useToast } from "@/components/ui/use-toast";
import { TestEmailForm } from "./TestEmailForm";
import { CampaignSenderDialog } from "./CampaignSenderDialog";
import { CampaignSenderFields } from "./CampaignSenderFields";

const EditCampaignSenderSchema = EditCampaignSenderFormSchemaDTO.extend({
  server_id: z.string().uuid("Invalid server ID"),
});

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
  // Hooks and API
  const [updateSender, { isLoading: isUpdating }] =
    useUpdateCampaignSenderMutation();
  const { toast } = useToast();

  // Form
  const form = useForm<z.infer<typeof EditCampaignSenderSchema>>({
    resolver: zodResolver(EditCampaignSenderSchema),
    mode: "onChange",
    defaultValues: {
      from_name: senderData?.from_name || "",
      from_email: senderData?.from_email || "",
      server_id: senderData?.server_id || "",
    },
  });

  // Form submission
  const onSubmit = async (values: z.infer<typeof EditCampaignSenderSchema>) => {
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

      toast({
        title: "Success",
        description: "Campaign sender updated successfully",
      });

      form.reset();
      onClose();
    } catch (error) {
      console.error("Form submission error:", error);
      toast({
        title: "Error",
        description: "Failed to update campaign sender",
        variant: "destructive",
      });
    }
  };

  // Handle successful test email
  const handleTestEmailSuccess = () => {
    toast({
      title: "Success",
      description: "Test email sent successfully",
    });
  };

  return (
    <CampaignSenderDialog
      open={open}
      onClose={onClose}
      title="Edit Campaign Sender"
      description="Update the details of this email sender."
      isSubmitting={isUpdating}
      onSubmit={form.handleSubmit(onSubmit)}
      submitButtonText="Update Sender"
    >
      <div className="py-4 space-y-4">
        <CampaignSenderFields form={form} showServerField={false} />

        {/* Test Email Section */}
        <TestEmailForm
          fromName={form.watch("from_name")}
          fromEmail={form.watch("from_email")}
          serverId={senderData.server_id}
          onTestSuccess={handleTestEmailSuccess}
        />
      </div>
    </CampaignSenderDialog>
  );
};

export { EditCampaignSender };
