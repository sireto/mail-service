import React, { useState } from "react";
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
import { useCheckCredentialsMutation } from "@/app/services/ServerApi";

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
    getValues,
  } = useServerForm(server);

  const serverType = watch("server_type");
  const isAwsServer = serverType === "AWS";
  const isSmtpServer = serverType === "SMTP";

  const [checkCredentials, { isLoading: isTesting }] =
    useCheckCredentialsMutation();
  const [testSuccess, setTestSuccess] = useState<boolean | null>(null);
  const [testError, setTestError] = useState<string | null>(null);

  const testAndSaveSmtpConnection = async () => {
    try {
      const formData = getValues();
      const payload = { ...formData };

      const result = await checkCredentials(payload).unwrap();
      if (result.success) {
        setTestSuccess(true);
        setTestError(null);
        await onSubmit(formData);
      } else {
        setTestSuccess(false);
        setTestError(result.message || "SMTP connection failed.");
      }
    } catch (err) {
      const error = err as { data?: { message?: string }; message?: string };

      const message =
        error?.data?.message ||
        error?.message ||
        "Connection test failed. Please check the credentials.";

      setTestSuccess(false);
      setTestError(message);
    }
  };

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
          {testSuccess && (
            <p className="text-green-600 text-sm mt-1">
              SMTP connection successful!
            </p>
          )}
          {testSuccess === false && (
            <p className="text-red-600 text-sm mt-1">{testError}</p>
          )}

          {/* Submit and Test Buttons */}
          <div className="flex justify-end gap-4">
            {/* SMTP Only: Test Connection Button */}
            {isSmtpServer && (
              <>
                <Button
                  type="button"
                  onClick={handleSubmit(testAndSaveSmtpConnection)}
                  disabled={
                    isTesting ||
                    (isSmtpServer && !areSmtpFieldsFilled()) ||
                    (isAwsServer && !areAwsCredentialsFilled())
                  }
                >
                  {isTesting
                    ? "Testing..."
                    : server.id
                    ? "Test and Update"
                    : "Test and Create"}
                </Button>
              </>
            )}
            <Button
              type="submit"
              disabled={
                (isSmtpServer && !areSmtpFieldsFilled()) ||
                (isAwsServer && !areAwsCredentialsFilled())
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
