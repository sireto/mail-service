"use client";

import React, { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useSendTestEmailMutation } from "@/app/services/EmailIdentityApi";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { useToast } from "@/components/ui/use-toast";

// Test email schema
const TestEmailSchema = z.object({
  to_email: z.string().email("Invalid recipient email"),
});

type TestEmailFormValues = z.infer<typeof TestEmailSchema>;

interface TestEmailFormProps {
  fromName: string;
  fromEmail: string;
  serverId: string;
  isAwsServer?: boolean;
  onTestSuccess: () => void;
  disabled?: boolean;
}

export const TestEmailForm: React.FC<TestEmailFormProps> = ({
  fromName,
  fromEmail,
  serverId,
  isAwsServer,
  onTestSuccess,
  disabled = false,
}) => {
  // Hooks
  const [sendTestEmail, { isLoading: isSendingTest }] =
    useSendTestEmailMutation();
  const { toast } = useToast();

  // State
  const [testEmailStatus, setTestEmailStatus] = useState("");

  // Form
  const form = useForm<TestEmailFormValues>({
    resolver: zodResolver(TestEmailSchema),
    mode: "onChange",
    defaultValues: {
      to_email: "",
    },
  });

  // Send test email handler
  const handleSendTestEmail = async (values: TestEmailFormValues) => {
    if (!fromEmail || !fromName || !serverId) {
      setTestEmailStatus("Please fill out all sender information above first");
      return;
    }

    try {
      const result = await sendTestEmail({
        from_email: fromEmail,
        from_name: fromName,
        to_email: values.to_email,
        server_id: serverId,
        subject: "Test Email from Mail Service",
      }).unwrap();

      if (result.success) {
        toast({
          title: "Success",
          description: "Test email sent successfully",
        });
        setTestEmailStatus("Email sent successfully");
        onTestSuccess(); // Notify parent component of success
      } else {
        setTestEmailStatus(`Failed to send email: ${result.message}`);
      }
    } catch (error: any) {
      setTestEmailStatus(
        "Failed to send email: " + (error?.data?.message || "Unknown error")
      );
      console.error("Send test email error:", error);
    }
  };

  return (
    <Accordion type="single" collapsible className="mt-6">
      <AccordionItem value="test-email">
        <AccordionTrigger>Send Test Email</AccordionTrigger>
        <AccordionContent>
          <div className="space-y-3 py-2">
            <p className="text-sm text-gray-600">
              Send a test email to verify this sender works correctly.
              {isAwsServer &&
                " This will also validate the email identity with AWS SES."}
            </p>

            <Form {...form}>
              <div className="space-y-3">
                <FormField
                  control={form.control}
                  name="to_email"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Recipient Email</FormLabel>
                      <FormControl>
                        <Input placeholder="Enter recipient email" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <Button
                  type="button"
                  variant="outline"
                  disabled={
                    disabled ||
                    isSendingTest ||
                    !fromEmail ||
                    !fromName ||
                    !serverId ||
                    !form.getValues("to_email")
                  }
                  className="w-full"
                  onClick={form.handleSubmit(handleSendTestEmail)}
                >
                  {isSendingTest ? "Sending..." : "Send Test Email"}
                </Button>

                {testEmailStatus && (
                  <p
                    className={`text-xs mt-2 ${
                      testEmailStatus.includes("success")
                        ? "text-green-500"
                        : "text-red-500"
                    }`}
                  >
                    {testEmailStatus}
                  </p>
                )}
              </div>
            </Form>
          </div>
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  );
};
