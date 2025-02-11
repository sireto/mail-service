"use client";
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
import { Minus, Plus, TestTube2 } from "lucide-react";
import { ServerSchema, type Server } from "@/lib/type";

interface ServerCardProps {
  server: Partial<Server>;
  onUpdate: (id: string, data: Server) => Promise<void>;
  onDelete: (id: string) => Promise<void>;
  onCreate: (data: Omit<Server, "id">) => Promise<void>;
}

export default function ServerCard({
  server,
  onUpdate,
  onDelete,
  onCreate,
}: ServerCardProps) {
  const {
    register,
    handleSubmit,
    setValue,
    watch,
    formState: { errors },
  } = useForm<Server>({
    resolver: zodResolver(ServerSchema),
    defaultValues: {
      active: server.active || true,
      host: server.host || "",
      namespace_id: server.namespace_id || "",
      port: server.port || 25,
      smtp_password: server.smtp_password || "",
      smtp_username: server.smtp_username || "",
      tls_type: server.tls_type || "STARTTLS",
    },
  });

  const port = watch("port");

  const onSubmit = async (data: Server) => {
    const serverData = {
      ...data,
      namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82", //namespace id currently present in the db(hardcoded for now), will be changed later
    };

    if (server.id) {
      await onUpdate(server.id, serverData);
    } else {
      const { id, ...createData } = serverData; //id will be generated by server
      await onCreate(createData);
    }
  };

  const handleDelete = async () => {
    if (
      !server.id ||
      !confirm("Are you sure you want to delete this configuration?")
    )
      return;
    await onDelete(server.id);
  };

  const handlePortChange = (action: "increase" | "decrease") => {
    const newPort = action === "increase" ? port + 1 : Math.max(port - 1, 0);
    setValue("port", newPort);
  };

  return (
    <Card className="w-full max-w-4xl">
      <CardContent className="p-6 space-y-6">
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <div className="flex items-center space-x-4">
            <Switch
              id="enabled"
              checked={watch("active")} // Bind switch to active field
              onCheckedChange={(checked) => setValue("active", checked)} // Update active on switch change
            />
            <Label htmlFor="enabled">Enabled</Label>
            {server.id && (
              <Button
                type="button"
                variant="ghost"
                className="text-blue-600 hover:text-blue-800 ml-auto"
                onClick={handleDelete}
              >
                Delete
              </Button>
            )}
          </div>

          <div className="grid md:grid-cols-[2fr,1fr] gap-4">
            <div className="space-y-2">
              <Label htmlFor="host">Host</Label>
              <Input
                id="host"
                {...register("host")}
                placeholder="smtp.yoursite.com"
              />
              {errors.host && (
                <p className="text-sm text-red-500">{errors.host.message}</p>
              )}
            </div>
            <div className="space-y-2">
              <Label htmlFor="port">Port</Label>
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
                  id="port"
                  {...register("port", { valueAsNumber: true })}
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
                <p className="text-sm text-red-500">{errors.port.message}</p>
              )}
            </div>
          </div>

          <div className="grid md:grid-cols-[2fr,2fr] gap-4">
            <div className="space-y-2">
              <Label htmlFor="smtp_username">User Name</Label>
              <Input
                id="smtp_username"
                {...register("smtp_username")}
                placeholder="username"
              />
              {errors.smtp_username && (
                <p className="text-sm text-red-500">
                  {errors.smtp_username.message}
                </p>
              )}
            </div>
            <div className="space-y-2">
              <Label htmlFor="smtp_password">Password</Label>
              <Input
                id="smtp_password"
                type="password"
                {...register("smtp_password")}
              />
              {errors.smtp_password && (
                <p className="text-sm text-red-500">
                  {errors.smtp_password.message}
                </p>
              )}
            </div>
          </div>

          <div className="grid md:grid-cols-[2fr,1fr,1fr] gap-4">
            <div className="space-y-2">
              <Label>TLS</Label>
              <Select
                defaultValue={server.tls_type || "STARTTLS"}
                onValueChange={(value) =>
                  setValue("tls_type", value as Server["tls_type"])
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="STARTTLS">STARTTLS</SelectItem>
                  <SelectItem value="SSL/TLS">SSL/TLS</SelectItem>
                  <SelectItem value="NONE">NONE</SelectItem>
                </SelectContent>
              </Select>
              {errors.tls_type && (
                <p className="text-sm text-red-500">
                  {errors.tls_type.message}
                </p>
              )}
            </div>
          </div>

          <div className="flex justify-end space-x-4">
            <Button type="button">
              <TestTube2 className="mr-2 h-4 w-4" />
              Test connection
            </Button>
            <Button type="submit">
              {server.id ? "Update Server" : "Create Server"}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
