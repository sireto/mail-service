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
                  <SelectItem value="us-east-1">
                    US East (N. Virginia)
                  </SelectItem>
                  <SelectItem value="us-east-2">US East (Ohio)</SelectItem>
                  <SelectItem value="us-west-1">
                    US West (N. California)
                  </SelectItem>
                  <SelectItem value="us-west-2">US West (Oregon)</SelectItem>
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
                  <SelectItem value="ca-central-1">Canada (Central)</SelectItem>
                  <SelectItem value="eu-central-1">
                    Europe (Frankfurt)
                  </SelectItem>
                  <SelectItem value="eu-west-1">Europe (Ireland)</SelectItem>
                  <SelectItem value="eu-west-2">Europe (London)</SelectItem>
                  <SelectItem value="eu-west-3">Europe (Paris)</SelectItem>
                  <SelectItem value="eu-north-1">Europe (Stockholm)</SelectItem>
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
    </>
  );
}
