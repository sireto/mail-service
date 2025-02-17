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
<<<<<<< HEAD
import { Button } from '../ui/button';
=======
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)

interface ConfirmationPopupProps {
    title: string;
    message: string;
    onConfirm: () => void;
<<<<<<< HEAD
    popupTriggerButton: React.ReactNode;
    confirmButton?: React.ReactNode;
=======
    confirmButton: React.ReactNode;
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
    cancelText?: string;
    isLoading ?: boolean;
}

const ConfirmationPopup: React.FC<ConfirmationPopupProps> = ({
    title,
    message,
    onConfirm,
<<<<<<< HEAD
    popupTriggerButton,
    confirmButton = <Button variant={'default'}>Continue</Button>,
=======
    confirmButton,
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
    cancelText = "Cancel",
    isLoading = false
}) => {
  return (
    <AlertDialog>
        <AlertDialogTrigger asChild>
<<<<<<< HEAD
            { popupTriggerButton }
=======
            { confirmButton }
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
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
<<<<<<< HEAD
                asChild
            >{ confirmButton }</AlertDialogAction>
=======
            >Continue</AlertDialogAction>
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
          </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
  )
}

export default ConfirmationPopup