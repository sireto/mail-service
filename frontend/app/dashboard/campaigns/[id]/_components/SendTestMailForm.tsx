import { SendHorizonal } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useSendTemplatedEmailMutation } from "@/app/services/TemplateApi";

const SendTestMailFormDTO = z.object({
  email: z.string().email(),
});

const SendTestMailForm = ({ templateId }: { templateId: string }) => {
  // Initialize the form with default values and a Zod resolver
  const form = useForm<z.infer<typeof SendTestMailFormDTO>>({
    resolver: zodResolver(SendTestMailFormDTO),
    defaultValues: { email: "" },
  });

  const [sendTemplatedMail] = useSendTemplatedEmailMutation();

  // onSubmit handler
  const onSubmit = (data: z.infer<typeof SendTestMailFormDTO>) => {
    console.log("Sending test email to:", data.email);
    // Here, you can call your API or perform any actions needed to send the test mail.
    const payload = {
      bcc: "",
      cc: "",
      from: "ses@id21.io",
      receiver: data.email,
      subject: "Test email from ID21",
      template_data: JSON.stringify({
        first_name: "Jack",
        last_name: "Doe",
        email: "example@example.com",
      }),
    };

    sendTemplatedMail({ templateId, payload });

    form.reset();
  };

  return (
    <div className="bg-white rounded-md my-4 flex-1/2">
      <h2 className="text-lg font-bold text-black">Send Test Mail</h2>
      <form
        className="my-2 flex items-start justify-center gap-x-4 lg:flex-col lg:items-start lg:gap-y-4
            "
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <div className="w-full">
          <Input
            placeholder="Enter email address"
            {...form.register("email")}
          />
          <p className="text-error">{form.formState.errors.email?.message}</p>
        </div>
        <Button
          type="submit"
          className="bg-primary text-white py-2 px-4 rounded-md"
        >
          <span className="text-[16px]">Send</span>
          <SendHorizonal size={16} />
        </Button>
      </form>
    </div>
  );
};

export default SendTestMailForm;
