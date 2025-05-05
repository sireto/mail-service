import React from "react";
import { UseFormReturn } from "react-hook-form";
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
import { Server } from "@/lib/type/server";
import { AddCampaignSenderSchema } from "./AddCampaignSender";
import { z } from "zod";

type AddCampaignSenderSchemaType = z.infer<typeof AddCampaignSenderSchema>;

interface CampaignSenderFieldsProps {
  form: UseFormReturn<AddCampaignSenderSchemaType>;
  servers?: Server[];
  isLoadingServers?: boolean;
  onFieldChange?: () => void;
  showServerField?: boolean;
}

export const CampaignSenderFields: React.FC<CampaignSenderFieldsProps> = ({
  form,
  servers = [],
  isLoadingServers = false,
  onFieldChange,
  showServerField = true,
}) => {
  return (
    <Form {...form}>
      {showServerField && (
        <FormField
          control={form.control}
          name="server_id"
          render={({ field }) => (
            <FormItem className="mb-4">
              <FormLabel>Server</FormLabel>
              <Select
                onValueChange={(value) => {
                  field.onChange(value);
                  onFieldChange?.();
                }}
                defaultValue={field.value}
                disabled={isLoadingServers}
              >
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Select a server" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  {servers.map((server) => (
                    <SelectItem key={server.id} value={server.id || ""}>
                      {server.host
                        ? `${server.host} (${server.server_type})`
                        : `${server.aws_credentials?.access_key_id} (${server.server_type})`}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />
      )}

      <FormField
        control={form.control}
        name="from_name"
        render={({ field }) => (
          <FormItem className="mb-4">
            <FormLabel>From Name</FormLabel>
            <FormControl>
              <Input
                placeholder="Enter sender name"
                {...field}
                onChange={(e) => {
                  field.onChange(e);
                  onFieldChange?.();
                }}
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />

      <FormField
        control={form.control}
        name="from_email"
        render={({ field }) => (
          <FormItem className="mb-4">
            <FormLabel>From Email</FormLabel>
            <FormControl>
              <Input
                placeholder="Enter sender email"
                {...field}
                onChange={(e) => {
                  field.onChange(e);
                  onFieldChange?.();
                }}
              />
            </FormControl>
            <FormMessage />
          </FormItem>
        )}
      />
    </Form>
  );
};
