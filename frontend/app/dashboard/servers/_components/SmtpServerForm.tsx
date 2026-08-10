// components/SmtpServerForm.tsx
import React from "react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Minus, Plus } from "lucide-react";
import {
  Controller,
  UseFormRegister,
  UseFormWatch,
  UseFormTrigger,
  Control,
  FieldErrors,
  UseFormSetValue,
} from "react-hook-form";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { usePortControls } from "@/hooks/usePortControl";
import { Server } from "@/lib/type";

interface SmtpServerFormProps {
  register: UseFormRegister<Server>;
  errors: FieldErrors<Server>;
  control: Control<Server>;
  trigger: UseFormTrigger<Server>;
  watch: UseFormWatch<Server>;
  setValue: UseFormSetValue<Server>;
}

export default function SmtpServerForm({
  register,
  setValue,
  errors,
  control,
  trigger,
  watch,
}: SmtpServerFormProps) {
  const { handlePortChange } = usePortControls(watch, setValue, trigger);

  return (
    <>
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

      <div className="grid md:grid-cols-[2fr,1fr,1fr] gap-4">
        <div className="space-y-2">
          <Label>Default From Email</Label>
          <Input
            {...register("default_from_email", {
              onChange: () => trigger("default_from_email"),
            })}
            placeholder="from.email@test.io"
          />
          {errors.default_from_email && (
            <span className="text-sm text-destructive">
              {errors.default_from_email.message}
            </span>
          )}
        </div>
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
        <div className="space-y-2">
          <Label>Rate Limit</Label>
          <Input
            {...register("rate_limit", {
              onChange: () => trigger("rate_limit"),
              valueAsNumber: true,
            })}
            placeholder="30"
          />
          {errors.rate_limit && (
            <span className="text-sm text-destructive">
              {errors.rate_limit.message}
            </span>
          )}
        </div>
      </div>
    </>
  );
}
