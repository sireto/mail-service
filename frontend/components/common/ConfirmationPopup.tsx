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
<<<<<<< HEAD
import { Button } from '../ui/button';
=======
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
=======
import { Button } from '../ui/button';
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)

interface ConfirmationPopupProps {
    title: string;
    message: string;
    onConfirm: () => void;
<<<<<<< HEAD
<<<<<<< HEAD
    popupTriggerButton: React.ReactNode;
    confirmButton?: React.ReactNode;
=======
    confirmButton: React.ReactNode;
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
=======
    popupTriggerButton: React.ReactNode;
    confirmButton?: React.ReactNode;
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
    cancelText?: string;
    isLoading ?: boolean;
}

const ConfirmationPopup: React.FC<ConfirmationPopupProps> = ({
    title,
    message,
    onConfirm,
<<<<<<< HEAD
<<<<<<< HEAD
    popupTriggerButton,
    confirmButton = <Button variant={'default'}>Continue</Button>,
=======
    confirmButton,
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
=======
    popupTriggerButton,
    confirmButton = <Button variant={'default'}>Continue</Button>,
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
    cancelText = "Cancel",
    isLoading = false
}) => {
  return (
    <AlertDialog>
        <AlertDialogTrigger asChild>
<<<<<<< HEAD
<<<<<<< HEAD
            { popupTriggerButton }
=======
            { confirmButton }
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
=======
            { popupTriggerButton }
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
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
<<<<<<< HEAD
                asChild
            >{ confirmButton }</AlertDialogAction>
=======
            >Continue</AlertDialogAction>
>>>>>>> d42c050 (feat: add the common/shared alert-dialog component)
=======
                asChild
            >{ confirmButton }</AlertDialogAction>
>>>>>>> f3ca610 (fix: add status reasons on the status tags tooltip and backend fixes)
          </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
  )
}

export default ConfirmationPopup