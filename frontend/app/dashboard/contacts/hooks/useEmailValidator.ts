import { useEffect } from "react";
import { UseFormReturn } from "react-hook-form";
import { z } from "zod";
import { useLazyCheckEmailQuery } from "@/app/services/ContactApi";
import { ContactFormSchema } from "@/lib/type/contact";

export const useEmailValidation = (
  form: UseFormReturn<z.infer<typeof ContactFormSchema>>,
) => {
  const [triggerCheckEmail] = useLazyCheckEmailQuery();

  useEffect(() => {
    const subscription = form.watch((value, { name }) => {
      if (name === "email") {
        const email = value.email;
        const isValidEmail = z.string().email().safeParse(email).success;

        if (!isValidEmail) return;

        const handler = setTimeout(() => {
          if (email) {
            triggerCheckEmail(email)
              .unwrap()
              .then((exists) => {
                if (exists) {
                  form.setError("email", {
                    type: "manual",
                    message: "Email already exists",
                  });
                } else {
                  form.clearErrors("email");
                }
              })
              .catch((error) => {
                console.error("Email check failed:", error);
              });
          }
        }, 500);

        return () => clearTimeout(handler);
      }
    });

    return () => subscription.unsubscribe();
  }, [form, triggerCheckEmail]);
};
