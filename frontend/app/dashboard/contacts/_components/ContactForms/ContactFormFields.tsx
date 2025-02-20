import React, { useRef } from "react";
import { UseFormReturn } from "react-hook-form";
import { z } from "zod";
import {
  Form,
  FormField,
  FormControl,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
  SelectValue,
} from "@/components/ui/select";
import { Checkbox } from "@/components/ui/checkbox";
import { ContactFormSchema } from "@/lib/type/contact";
import { MultiSelect } from "@/components/multi-select";
import { Textarea } from "@/components/ui/textarea";

interface List {
  id: string;
  name: string;
  description: string;
  namespace_id: string;
  created_at: string;
  updated_at: string;
}

interface ContactFormFieldsProps {
  form: UseFormReturn<z.infer<typeof ContactFormSchema>>;
  lists: List[] | null;
  handleListChange: (selectedLists: string[]) => void;
  onSubmit: (values: z.infer<typeof ContactFormSchema>) => Promise<void>;
}

const ContactFormFields: React.FC<ContactFormFieldsProps> = ({
  form,
  lists,
  handleListChange,
  onSubmit,
}) => {
  const multiSelectRef = useRef(null);

  // Transform lists into options format required by MultiSelect
  const listOptions =
    lists?.map((list) => ({
      label: list.name,
      value: list.id,
    })) || [];

  return (
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
                  value={field.value || ""}
                  onChange={(e) => {
                    field.onChange(e);
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
              <Select onValueChange={field.onChange} defaultValue={field.value}>
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
          name="listIds"
          render={({ field }) => (
            <FormItem>
              <div className="text-sm font-medium">Lists</div>
              <FormControl>
                <MultiSelect
                  ref={multiSelectRef}
                  options={listOptions}
                  onValueChange={handleListChange}
                  defaultValue={field.value || []}
                  placeholder="Select lists"
                  variant="default"
                  className="mt-1.5"
                  maxCount={3}
                />
              </FormControl>
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
                  Dont send opt-in e-mails and mark all list subscriptions as
                  subscribed.
                </p>
              </div>
            </FormItem>
          )}
        />

        {/* Attributes Field */}
        <FormField
          control={form.control}
          name="attribute"
          render={({ field }) => (
            <FormItem>
              <div className="text-sm font-medium">Attributes</div>
              <FormControl>
                <Textarea
                  {...field}
                  placeholder='{"key": "value"}'
                  className="mt-1.5 w-full min-h-[80px] rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
      </form>
    </Form>
  );
};

export { ContactFormFields };
