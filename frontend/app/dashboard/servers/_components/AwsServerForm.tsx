// components/AwsServerForm.tsx
import React from "react";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Controller,
  UseFormRegister,
  UseFormTrigger,
  Control,
  FieldErrors,
} from "react-hook-form";
import { Server } from "@/lib/type";
import serverRegions from "../_constants/server";

interface AwsServerFormProps {
  register: UseFormRegister<Server>;
  errors: FieldErrors<Server>;
  control: Control<Server>;
  trigger: UseFormTrigger<Server>;
}

export default function AwsServerForm({
  register,
  errors,
  control,
  trigger,
}: AwsServerFormProps) {
  return (
    <>
      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label>AWS Access Key ID</Label>
          <Input
            {...register("aws_credentials.access_key_id", {
              onChange: () => trigger("aws_credentials.access_key_id"),
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
              onChange: () => trigger("aws_credentials.secret_access_key"),
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
                  {
                    serverRegions.map(region => (
                      <SelectItem key={region.label} value={region.value}>
                        { region.label }
                      </SelectItem>
                    ))
                  }
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
      
      <div className="grid md:grid-cols-[2fr,1fr] gap-4">
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
