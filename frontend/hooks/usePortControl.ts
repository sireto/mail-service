import { UseFormWatch, UseFormSetValue, UseFormTrigger } from "react-hook-form";
import { Server } from "@/lib/type";

export function usePortControls(
  watch: UseFormWatch<Server>,
  setValue: UseFormSetValue<Server>,
  trigger: UseFormTrigger<Server>,
) {
  const currentPort = watch("port");

  const handlePortChange = (action: "increase" | "decrease") => {
    const newPort =
      action === "increase" ? currentPort + 1 : Math.max(currentPort - 1, 0);
    setValue("port", newPort, {
      shouldDirty: true,
      shouldTouch: true,
      shouldValidate: true,
    });
  };

  return { handlePortChange };
}
