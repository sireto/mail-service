import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ServerSchema, type Server } from "@/lib/type";
import { NAMESPACE_ID } from "@/config/namespace";
import { useEffect } from "react";
import {
  useCreateServerMutation,
  useUpdateServerMutation,
  useDeleteServerMutation,
} from "@/app/services/ServerApi";

/**
 * The API masks secrets on read (a run of asterisks). Seeding a form field with that mask
 * and submitting it wrote the mask back as the real credential, because PATCH replaces
 * every column. Blank the field instead: the backend treats blank as "leave unchanged".
 */
const isMaskedSecret = (value?: string | null): boolean =>
  !!value && value.trim().length > 0 && /^\*+$/.test(value.trim());

const unmask = (value?: string | null): string =>
  !value || isMaskedSecret(value) ? "" : value;

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
      namespace_id: NAMESPACE_ID,
      port: server.port ?? (server.server_type === "AWS" ? 1 : 25),
      smtp_username: server.smtp_username ?? "",
      smtp_password: unmask(server.smtp_password),
      tls_type: server.tls_type ?? "STARTTLS",
      server_type: server.server_type ?? "SMTP",
      aws_credentials: {
        access_key_id: unmask(server.aws_credentials?.access_key_id),
        secret_access_key: unmask(server.aws_credentials?.secret_access_key),
        region: server.aws_credentials?.region || "ap-southeast-1",
        session_token: unmask(server.aws_credentials?.session_token) || null,
      },
      default_from_email: server.default_from_email ?? "",
      rate_limit: server.rate_limit ?? 60,
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
        namespace_id: server.namespace_id ?? NAMESPACE_ID,
        port: server.port ?? 25,
        smtp_username: server.smtp_username ?? "",
        smtp_password: unmask(server.smtp_password),
        tls_type: server.tls_type ?? "STARTTLS",
        server_type: server.server_type ?? "SMTP",
        aws_credentials: {
          access_key_id: unmask(server.aws_credentials?.access_key_id),
          secret_access_key: unmask(server.aws_credentials?.secret_access_key),
          region: server.aws_credentials?.region || "ap-southeast-1",
          session_token: unmask(server.aws_credentials?.session_token) || null,
        },
        default_from_email: server.default_from_email ?? "",
        rate_limit: server.rate_limit ?? 60,
      });
    }
  }, [server, reset]);

  // Form submission handler
  const onSubmit = async (data: Server) => {
    try {
      // Prepare data based on server type before submission
      let serverData: Server;

      const scrubbed: Server = {
        ...data,
        smtp_password: unmask(data.smtp_password),
        aws_credentials: {
          ...data.aws_credentials,
          access_key_id: unmask(data.aws_credentials?.access_key_id),
          secret_access_key: unmask(data.aws_credentials?.secret_access_key),
          session_token: unmask(data.aws_credentials?.session_token) || null,
        },
      };

      if (scrubbed.server_type === "AWS") {
        // If AWS, set default values for SMTP fields
        serverData = {
          ...scrubbed,
          host: "",
          port: 25,
          smtp_username: "",
          smtp_password: "",
          tls_type: "NONE",
          namespace_id: NAMESPACE_ID,
          aws_credentials: {
            access_key_id: scrubbed.aws_credentials?.access_key_id || "",
            secret_access_key: scrubbed.aws_credentials?.secret_access_key || "",
            region: scrubbed.aws_credentials?.region || "ap-southeast-1",
            session_token: scrubbed.aws_credentials?.session_token || null,
          },
          // These previously read from the stale `server` prop rather than the submitted
          // form, so edits to either field were silently discarded.
          default_from_email: scrubbed.default_from_email,
          rate_limit: scrubbed.rate_limit,
        };
      } else {
        // If SMTP, set default values for AWS fields
        serverData = {
          ...scrubbed,
          aws_credentials: {
            access_key_id: "",
            secret_access_key: "",
            region: "",
            session_token: null,
          },
          namespace_id: NAMESPACE_ID,
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
