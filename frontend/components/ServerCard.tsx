import React from "react";
import { useForm } from "react-hook-form";
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
    formState: { errors, isValid },
  } = useForm<Server>({
    resolver: zodResolver(ServerSchema),
    defaultValues: {
      id: server.id,
      active: server.active ?? true,
      host: server.host ?? "",
      namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
      port: server.port ?? 25,
      smtp_password: server.smtp_password ?? "",
      smtp_username: server.smtp_username ?? "",
      tls_type: server.tls_type ?? "STARTTLS",
    },
    mode: "all",
    reValidateMode: "onChange",
  });

  const currentPort = watch("port");

  useEffect(() => {
    if (server) {
      reset({
        id: server.id,
        active: server.active ?? true,
        host: server.host ?? "",
        namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
        port: server.port ?? 25,
        smtp_username: server.smtp_username ?? "",
        smtp_password: server.smtp_password ?? "",
        tls_type: server.tls_type ?? "STARTTLS",
      });
    }
  }, [server, reset]);

  const onSubmit = async (data: Server) => {
    try {
      const isFormValid = await trigger();
      if (!isFormValid) {
        console.error("Form validation failed:", errors);
        return;
      }

      const serverData = {
        ...data,
        namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82", //hardcoded for now will be changed later
      };

      if (server.id) {
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
              <Select value={watch("tls_type")}>
                <SelectTrigger>
                  <SelectValue placeholder="Select TLS" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="STARTTLS">STARTTLS</SelectItem>
                  <SelectItem value="SSL/TLS">SSL/TLS</SelectItem>
                  <SelectItem value="NONE">None</SelectItem>
                </SelectContent>
              </Select>
              {errors.tls_type && (
                <span className="text-sm text-destructive">
                  {errors.tls_type.message}
                </span>
              )}
            </div>
          </div>

          <div className="flex justify-end gap-4">
            <Button type="submit" disabled={!isValid}>
              {server.id ? "Save Changes" : "Create Server"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
