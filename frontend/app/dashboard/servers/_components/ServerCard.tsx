// ServerCard.tsx - Main component file
import React from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Tabs, TabsList, TabsContent, TabsTrigger } from "@/components/ui/tabs";
import { Controller } from "react-hook-form";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useServerForm } from "@/hooks/useServerForm";
import SmtpServerForm from "@/app/dashboard/servers/_components/SmtpServerForm";
import AwsServerForm from "@/app/dashboard/servers/_components/AwsServerForm";
import { type Server } from "@/lib/type";

interface ServerCardProps {
  server: Partial<Server>;
  onCancel?: () => void;
}

export default function ServerCard({ server, onCancel }: ServerCardProps) {
  const {
    control,
    register,
    handleSubmit,
    watch,
    setValue,
    errors,
    handleDelete,
    onSubmit,
    areSmtpFieldsFilled,
    areAwsCredentialsFilled,
    trigger,
  } = useServerForm(server);

  const serverType = watch("server_type");

  return (
    <Card className="w-full max-w-4xl">
      <CardContent className="p-6 space-y-6">
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {/* Header Section */}
          {server.id ? (
            <div className="flex items-center space-x-4">
              <Switch
                checked={watch("active")}
                onCheckedChange={(checked) =>
                  setValue("active", checked, {
                    shouldDirty: true,
                    shouldTouch: true,
                    shouldValidate: true,
                  })
                }
              />
              <Label>Enabled</Label>
              <Button
                type="button"
                variant="ghost"
                className="ml-auto text-destructive"
                onClick={handleDelete}
              >
                Delete
              </Button>
            </div>
          ) : (
            <div className="flex items-center justify-between">
              <p>Add your server credentials below</p>
              <Button
                type="button"
                variant="ghost"
                className="text-destructive"
                onClick={onCancel}
              >
                Cancel
              </Button>
            </div>
          )}

          {/* Server Type Selection */}
          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Server Type</Label>
              <Controller
                control={control}
                name="server_type"
                render={({ field }) => (
                  <Select {...field} onValueChange={field.onChange}>
                    <SelectTrigger>
                      <SelectValue placeholder="Select Server Type" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="SMTP">SMTP Server</SelectItem>
                      <SelectItem value="AWS">AWS SES</SelectItem>
                    </SelectContent>
                  </Select>
                )}
              />
              {errors.server_type && (
                <span className="text-sm text-destructive">
                  {errors.server_type.message}
                </span>
              )}
            </div>

            {/* Server Configuration Tabs */}
            <div className="sm:w-[700px]">
              <Tabs value={serverType} className="w-full">
                <TabsList className="grid w-full grid-cols-2">
                  <TabsTrigger
                    value="SMTP"
                    onClick={() =>
                      setValue("server_type", "SMTP", {
                        shouldValidate: true,
                      })
                    }
                  >
                    SMTP Server
                  </TabsTrigger>
                  <TabsTrigger
                    value="AWS"
                    onClick={() =>
                      setValue("server_type", "AWS", {
                        shouldValidate: true,
                      })
                    }
                  >
                    AWS SES
                  </TabsTrigger>
                </TabsList>

                {/* SMTP Form */}
                <TabsContent value="SMTP" className="space-y-4 mt-4">
                  <SmtpServerForm
                    setValue={setValue}
                    register={register}
                    errors={errors}
                    control={control}
                    trigger={trigger}
                    watch={watch}
                  />
                </TabsContent>

                {/* AWS Form */}
                <TabsContent value="AWS" className="space-y-4 mt-4">
                  <AwsServerForm
                    register={register}
                    errors={errors}
                    control={control}
                    trigger={trigger}
                  />
                </TabsContent>
              </Tabs>
            </div>
          </div>

          {/* Submit Button */}
          <div className="flex justify-end gap-4">
            <Button
              type="submit"
              disabled={
                (serverType === "SMTP" && !areSmtpFieldsFilled()) ||
                (serverType === "AWS" && !areAwsCredentialsFilled())
              }
            >
              {server.id ? "Save Changes" : "Create Server"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
