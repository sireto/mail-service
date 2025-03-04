import React from "react";
import { Controller, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { Minus, Plus } from "lucide-react";
import { ServerSchema, type Server } from "@/lib/type";
import {
  useCreateServerMutation,
  useUpdateServerMutation,
  useDeleteServerMutation,
} from "@/app/services/ServerApi";
import { useEffect } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface ServerCardProps {
  server: Partial<Server>;
  onCancel?: () => void;
}

export default function ServerCard({ server, onCancel }: ServerCardProps) {
  const [createServer] = useCreateServerMutation();
  const [updateServer] = useUpdateServerMutation();
  const [deleteServer] = useDeleteServerMutation();

  const {
    register,
    handleSubmit,
    setValue,
    watch,
    reset,
    trigger,
    control,
    formState: { errors, isValid },
  } = useForm<Server>({
    resolver: zodResolver(ServerSchema),
    defaultValues: {
      id: server.id,
      active: server.active ?? true,
      host: server.host ?? (server.server_type === "AWS" ? "" : ""),
      namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
      port: server.port ?? (server.server_type === "AWS" ? 1 : 25),
      smtp_username: server.smtp_username ?? "",
      smtp_password: server.smtp_password ?? "",
      tls_type: server.tls_type ?? "STARTTLS",
      server_type: server.server_type ?? "SMTP",
      aws_credentials: server.aws_credentials ?? {
        access_key_id: "",
        secret_access_key: "",
        region: "ap-southeast-1",
        session_token: null,
      },
    },
    mode: "all",
    reValidateMode: "onChange",
  });

  const currentPort = watch("port");
  const serverType = watch("server_type");

  // Check if AWS credentials are filled
  const areAwsCredentialsFilled = () => {
    const awsCredentials = watch("aws_credentials");
    return (
      serverType === "AWS" &&
      !!awsCredentials.access_key_id &&
      !!awsCredentials.secret_access_key &&
      !!awsCredentials.region
    );
  };

  // Check if SMTP fields are filled
  const areSmtpFieldsFilled = () => {
    return (
      serverType === "SMTP" && !!watch("host") && !errors.host && !errors.port
    );
  };

  useEffect(() => {
    if (server) {
      reset({
        id: server.id,
        active: server.active ?? true,
        host: server.host ?? "",
        namespace_id:
          server.namespace_id ?? "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
        port: 25,
        smtp_username: server.smtp_username ?? "",
        smtp_password: server.smtp_password ?? "",
        tls_type: server.tls_type ?? "STARTTLS",
        server_type: server.server_type ?? "SMTP",
        aws_credentials: {
          access_key_id: server.aws_credentials?.access_key_id || "",
          secret_access_key: server.aws_credentials?.secret_access_key || "",
          region: server.aws_credentials?.region || "ap-southeast-1",
        },
      });
    }
  }, [server, reset]);

  const onSubmit = async (data: Server) => {
    console.log("Submitting data:", data);
    console.log("Selected server type:", serverType);
    try {
      if (serverType === "AWS") {
        console.log("AWS Credentials:", data.aws_credentials);
      }
      // Prepare data based on server type before submission
      let serverData: Server;

      if (data.server_type === "AWS") {
        // If AWS, set default values for SMTP fields
        serverData = {
          ...data,
          host: "",
          port: 25,
          smtp_username: "",
          smtp_password: "",
          tls_type: "NONE",
          namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
          aws_credentials: {
            access_key_id: data.aws_credentials?.access_key_id || "",
            secret_access_key: data.aws_credentials?.secret_access_key || "",
            region: data.aws_credentials?.region || "ap-southeast-1",
            session_token: data.aws_credentials?.session_token || null,
          },
        };
      } else {
        // If SMTP, set default values for AWS fields
        serverData = {
          ...data,
          aws_credentials: {
            access_key_id: "",
            secret_access_key: "",
            region: "",
            session_token: null,
          },
          namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82", // hardcoded as before
        };
      }
      console.log(server.id);
      if (server.id) {
        console.log("Sending update request with data:", {
          id: server.id,
          ...serverData,
        });
        const result = await updateServer({
          id: server.id,
          ...serverData,
        }).unwrap();
        console.log("Update success:", result);
      } else {
        const result = await createServer(serverData).unwrap();
        console.log("Create success:", result);
      }
    } catch (error) {
      console.error("Operation failed:", error);
    }
  };

  const handleDelete = async () => {
    if (!server.id || !confirm("Delete this server configuration?")) return;
    try {
      await deleteServer(server.id).unwrap();
    } catch (error) {
      console.error("Delete failed:", error);
    }
  };

  const handlePortChange = (action: "increase" | "decrease") => {
    const newPort =
      action === "increase" ? currentPort + 1 : Math.max(currentPort - 1, 0);
    setValue("port", newPort, {
      shouldDirty: true,
      shouldTouch: true,
      shouldValidate: true,
    });
  };

  return (
    <Card className="w-full max-w-4xl">
      <CardContent className="p-6 space-y-6">
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {server.id && (
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
              {server.id && (
                <Button
                  type="button"
                  variant="ghost"
                  className="ml-auto text-destructive"
                  onClick={handleDelete}
                >
                  Delete
                </Button>
              )}
            </div>
          )}
          {!server.id && (
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

                <TabsContent value="SMTP" className="space-y-4 mt-4">
                  <div className="grid md:grid-cols-[2fr,1fr] gap-4">
                    <div className="space-y-2">
                      <Label>Host</Label>
                      <Input
                        {...register("host", {
                          onChange: () => trigger("host"),
                        })}
                        placeholder="smtp.example.com"
                      />
                      {errors.host && (
                        <span className="text-sm text-destructive">
                          {errors.host.message}
                        </span>
                      )}
                    </div>

                    <div className="space-y-2">
                      <Label>Port</Label>
                      <div className="flex space-x-2">
                        <Button
                          type="button"
                          variant="outline"
                          size="icon"
                          onClick={() => handlePortChange("decrease")}
                        >
                          <Minus className="h-4 w-4" />
                        </Button>
                        <Input
                          {...register("port", {
                            valueAsNumber: true,
                            onChange: () => trigger("port"),
                          })}
                          className="text-center"
                        />
                        <Button
                          type="button"
                          variant="outline"
                          size="icon"
                          onClick={() => handlePortChange("increase")}
                        >
                          <Plus className="h-4 w-4" />
                        </Button>
                      </div>
                      {errors.port && (
                        <span className="text-sm text-destructive">
                          {errors.port.message}
                        </span>
                      )}
                    </div>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <Label>SMTP Username</Label>
                      <Input
                        {...register("smtp_username", {
                          onChange: () => trigger("smtp_username"),
                        })}
                        placeholder="username"
                      />
                      {errors.smtp_username && (
                        <span className="text-sm text-destructive">
                          {errors.smtp_username.message}
                        </span>
                      )}
                    </div>

                    <div className="space-y-2">
                      <Label>SMTP Password</Label>
                      <Input
                        type="password"
                        {...register("smtp_password", {
                          onChange: () => trigger("smtp_password"),
                        })}
                      />
                      {errors.smtp_password && (
                        <span className="text-sm text-destructive">
                          {errors.smtp_password.message}
                        </span>
                      )}
                    </div>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <Label>TLS Configuration</Label>
                      <Controller
                        control={control}
                        name="tls_type"
                        render={({ field }) => (
                          <Select {...field} onValueChange={field.onChange}>
                            <SelectTrigger>
                              <SelectValue placeholder="Select TLS" />
                            </SelectTrigger>
                            <SelectContent>
                              <SelectItem value="STARTTLS">STARTTLS</SelectItem>
                              <SelectItem value="SSL/TLS">SSL/TLS</SelectItem>
                              <SelectItem value="NONE">None</SelectItem>
                            </SelectContent>
                          </Select>
                        )}
                      />
                      {errors.tls_type && (
                        <span className="text-sm text-destructive">
                          {errors.tls_type.message}
                        </span>
                      )}
                    </div>
                  </div>
                </TabsContent>

                <TabsContent value="AWS" className="space-y-4 mt-4">
                  <div className="grid md:grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <Label>AWS Access Key ID</Label>
                      <Input
                        {...register("aws_credentials.access_key_id", {
                          onChange: () =>
                            trigger("aws_credentials.access_key_id"),
                        })}
                        placeholder="AKIAIOSFODNN7EXAMPLE"
                      />
                      {errors.aws_credentials?.access_key_id && (
                        <span className="text-sm text-destructive">
                          {errors.aws_credentials.access_key_id.message}
                        </span>
                      )}
                    </div>

                    <div className="space-y-2">
                      <Label>AWS Secret Access Key</Label>
                      <Input
                        type="password"
                        {...register("aws_credentials.secret_access_key", {
                          onChange: () =>
                            trigger("aws_credentials.secret_access_key"),
                        })}
                        placeholder="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
                      />
                      {errors.aws_credentials?.secret_access_key && (
                        <span className="text-sm text-destructive">
                          {errors.aws_credentials.secret_access_key.message}
                        </span>
                      )}
                    </div>
                  </div>

                  <div className="grid md:grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <Label>AWS Region</Label>
                      <Controller
                        control={control}
                        name="aws_credentials.region"
                        render={({ field }) => (
                          <Select {...field} onValueChange={field.onChange}>
                            <SelectTrigger>
                              <SelectValue placeholder="Select Region" />
                            </SelectTrigger>
                            <SelectContent>
                              <SelectItem value="us-east-1">
                                US East (N. Virginia)
                              </SelectItem>
                              <SelectItem value="us-east-2">
                                US East (Ohio)
                              </SelectItem>
                              <SelectItem value="us-west-1">
                                US West (N. California)
                              </SelectItem>
                              <SelectItem value="us-west-2">
                                US West (Oregon)
                              </SelectItem>
                              <SelectItem value="ap-south-1">
                                Asia Pacific (Mumbai)
                              </SelectItem>
                              <SelectItem value="ap-northeast-2">
                                Asia Pacific (Seoul)
                              </SelectItem>
                              <SelectItem value="ap-southeast-1">
                                Asia Pacific (Singapore)
                              </SelectItem>
                              <SelectItem value="ap-southeast-2">
                                Asia Pacific (Sydney)
                              </SelectItem>
                              <SelectItem value="ap-northeast-1">
                                Asia Pacific (Tokyo)
                              </SelectItem>
                              <SelectItem value="ca-central-1">
                                Canada (Central)
                              </SelectItem>
                              <SelectItem value="eu-central-1">
                                Europe (Frankfurt)
                              </SelectItem>
                              <SelectItem value="eu-west-1">
                                Europe (Ireland)
                              </SelectItem>
                              <SelectItem value="eu-west-2">
                                Europe (London)
                              </SelectItem>
                              <SelectItem value="eu-west-3">
                                Europe (Paris)
                              </SelectItem>
                              <SelectItem value="eu-north-1">
                                Europe (Stockholm)
                              </SelectItem>
                              <SelectItem value="sa-east-1">
                                South America (São Paulo)
                              </SelectItem>
                            </SelectContent>
                          </Select>
                        )}
                      />
                      {errors.aws_credentials?.region && (
                        <span className="text-sm text-destructive">
                          {errors.aws_credentials.region.message}
                        </span>
                      )}
                    </div>

                    <div className="space-y-2">
                      <Label>AWS Session Token (Optional)</Label>
                      <Input
                        {...register("aws_credentials.session_token")}
                        placeholder="Optional for temporary credentials"
                      />
                    </div>
                  </div>
                </TabsContent>
              </Tabs>
            </div>
          </div>

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
