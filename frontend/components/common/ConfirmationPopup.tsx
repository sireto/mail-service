import React from "react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "../ui/button";

interface ConfirmationPopupProps {
  title: string;
  message: string;
  onConfirm: () => void;
  popupTriggerButton: React.ReactNode;
  confirmButton?: React.ReactNode;
  cancelText?: string;
  isLoading?: boolean;
}

const ConfirmationPopup: React.FC<ConfirmationPopupProps> = ({
  title,
  message,
  onConfirm,
  popupTriggerButton,
  confirmButton = <Button variant={"default"}>Continue</Button>,
  cancelText = "Cancel",
  isLoading = false,
}) => {
  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>{popupTriggerButton}</AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{title}</AlertDialogTitle>
          <AlertDialogDescription>{message}</AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>{cancelText}</AlertDialogCancel>
          <AlertDialogAction onClick={onConfirm} disabled={isLoading} asChild>
            {confirmButton}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
};

export default ConfirmationPopup;
