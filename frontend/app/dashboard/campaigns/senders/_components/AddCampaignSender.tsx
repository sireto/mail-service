"use client";

import React, { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { EditCampaignSenderFormSchemaDTO } from "@/lib/type";
import { useCreateCampaignSenderMutation } from "@/app/services/CampaignSenderApi";
import { useGetServersQuery } from "@/app/services/ServerApi";
import { useToast } from "@/components/ui/use-toast";
import { TestEmailForm } from "./TestEmailForm";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { AlertCircle } from "lucide-react";
import { CampaignSenderDialog } from "./CampaignSenderDialog";
import { CampaignSenderFields } from "./CampaignSenderFields";
import { Server } from "@/lib/type/server";

// Schema for adding a campaign sender
export const AddCampaignSenderSchema = EditCampaignSenderFormSchemaDTO.extend({
  server_id: z.string().uuid("Invalid server ID"),
});

type FormValues = z.infer<typeof AddCampaignSenderSchema>;

interface AddCampaignSenderProps {
  open?: boolean;
  onClose: () => void;
}

const AddCampaignSender: React.FC<AddCampaignSenderProps> = ({
  open,
  onClose,
}) => {
  // Hooks and API
  const { data: servers, isLoading: isLoadingServers } = useGetServersQuery();
  const [createSender, { isLoading: isCreating }] =
    useCreateCampaignSenderMutation();
  const { toast } = useToast();

  // State
  const [testEmailSent, setTestEmailSent] = useState(false);
  const [showValidationError, setShowValidationError] = useState(false);

  // Form
  const form = useForm<FormValues>({
    resolver: zodResolver(AddCampaignSenderSchema),
    mode: "onChange",
    defaultValues: {
      server_id: "",
      from_name: "",
      from_email: "",
    },
  });

  // Get selected server type
  const selectedServer = servers?.find((s) => s.id === form.watch("server_id"));
  const isAwsServer = selectedServer?.server_type === "AWS";

  // Reset validation state when form fields change
  const handleFieldChange = () => {
    if (testEmailSent) {
      setTestEmailSent(false);
    }
    if (showValidationError) {
      setShowValidationError(false);
    }
  };

  // Form submission
  const onSubmit = async (values: FormValues) => {
    // Require test email to be sent successfully first
    if (!testEmailSent) {
      setShowValidationError(true);
      return;
    }

    try {
      const payload = {
        server_id: values.server_id,
        from_name: values.from_name,
        from_email: values.from_email,
      };

      await createSender(payload).unwrap();

      toast({
        title: "Success",
        description: "Campaign sender created successfully",
      });

      form.reset();
      onClose();
    } catch (error) {
      console.error("Form submission error:", error);
      toast({
        title: "Error",
        description: "Failed to create campaign sender",
        variant: "destructive",
      });
    }
  };

  // Handle successful test email
  const handleTestEmailSuccess = () => {
    setTestEmailSent(true);
    setShowValidationError(false);
  };

  return (
    <CampaignSenderDialog
      open={open}
      onClose={onClose}
      title="Add Campaign Sender"
      description="Add a new email sender for your campaigns."
      isSubmitting={isCreating}
      onSubmit={form.handleSubmit(onSubmit)}
      submitButtonText="Create Sender"
    >
      <div className="py-4 space-y-4">
        <CampaignSenderFields
          form={form}
          servers={(servers as Server[]) || []}
          isLoadingServers={isLoadingServers}
          onFieldChange={handleFieldChange}
        />

        {/* Test Email Section */}
        <TestEmailForm
          fromName={form.watch("from_name")}
          fromEmail={form.watch("from_email")}
          serverId={form.watch("server_id")}
          isAwsServer={isAwsServer}
          onTestSuccess={handleTestEmailSuccess}
        />

        {/* Validation error message */}
        {showValidationError && (
          <Alert variant="destructive">
            <AlertCircle className="h-4 w-4" />
            <AlertDescription>
              Please send a test email successfully before creating the sender.
            </AlertDescription>
          </Alert>
        )}

        {/* Success indicator */}
        {testEmailSent && (
          <Alert
            variant="default"
            className="bg-green-50 text-green-800 border-green-200"
          >
            <AlertDescription>
              Test email sent successfully! You can now create the sender.
            </AlertDescription>
          </Alert>
        )}
      </div>
    </CampaignSenderDialog>
  );
};

export { AddCampaignSender };
