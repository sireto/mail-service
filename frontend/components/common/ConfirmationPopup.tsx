import React from 'react';
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

interface ConfirmationPopupProps {
    title: string;
    message: string;
    onConfirm: () => void;
    confirmButton: React.ReactNode;
    cancelText?: string;
    isLoading ?: boolean;
}

const ConfirmationPopup: React.FC<ConfirmationPopupProps> = ({
    title,
    message,
    onConfirm,
    confirmButton,
    cancelText = "Cancel",
    isLoading = false
}) => {
  return (
    <AlertDialog>
        <AlertDialogTrigger asChild>
            { confirmButton }
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{ title }</AlertDialogTitle>
            <AlertDialogDescription>
              { message }
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>{ cancelText }</AlertDialogCancel>
            <AlertDialogAction
                onClick={ onConfirm }
                disabled={ isLoading }
            >Continue</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
  )
}

export default ConfirmationPopup