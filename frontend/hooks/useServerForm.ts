import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ServerSchema, type Server } from "@/lib/type";
import { useEffect } from "react";
import {
  useCreateServerMutation,
  useUpdateServerMutation,
  useDeleteServerMutation,
} from "@/app/services/ServerApi";

export function useServerForm(server: Partial<Server>) {
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
    getValues,
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
      default_from_email: server.default_from_email ?? "",
      rate_limit: server.rate_limit ?? 30,
    },
    mode: "all",
    reValidateMode: "onChange",
  });

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

  // Reset form when server changes
  useEffect(() => {
    if (server) {
      reset({
        id: server.id,
        active: server.active ?? true,
        host: server.host ?? "",
        namespace_id:
          server.namespace_id ?? "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
        port: server.port ?? 25,
        smtp_username: server.smtp_username ?? "",
        smtp_password: server.smtp_password ?? "",
        tls_type: server.tls_type ?? "STARTTLS",
        server_type: server.server_type ?? "SMTP",
        aws_credentials: {
          access_key_id: server.aws_credentials?.access_key_id || "",
          secret_access_key: server.aws_credentials?.secret_access_key || "",
          region: server.aws_credentials?.region || "ap-southeast-1",
          session_token: server.aws_credentials?.session_token || null,
        },
        default_from_email: server.default_from_email ?? "",
        rate_limit: server.rate_limit ?? 30,
      });
    }
  }, [server, reset]);

  // Form submission handler
  const onSubmit = async (data: Server) => {
    try {
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
          default_from_email: server.default_from_email ?? "",
          rate_limit: server.rate_limit ?? 30,
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
          namespace_id: "e3bda5cf-760e-43ea-8e9a-c2c3c5f95b82",
        };
      }

      if (server.id) {
        await updateServer({
          id: server.id,
          ...serverData,
        }).unwrap();
      } else {
        await createServer(serverData).unwrap();
      }
    } catch (error) {
      console.error("Operation failed:", error);
    }
  };

  // Delete handler
  const handleDelete = async () => {
    if (!server.id || !confirm("Delete this server configuration?")) return;
    try {
      await deleteServer(server.id).unwrap();
    } catch (error) {
      console.error("Delete failed:", error);
    }
  };

  return {
    register,
    handleSubmit,
    setValue,
    watch,
    reset,
    trigger,
    control,
    errors,
    isValid,
    onSubmit,
    handleDelete,
    areAwsCredentialsFilled,
    areSmtpFieldsFilled,
    getValues,
  };
}
