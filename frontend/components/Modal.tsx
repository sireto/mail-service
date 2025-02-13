import React from 'react'
/* eslint-disable @typescript-eslint/no-unused-vars */
import { Button } from '@/components/ui/button'
import { 
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogFooter,
    DialogTrigger, 
} from './ui/dialog';
import { ScanEye } from "lucide-react";
/* eslint-disable @typescript-eslint/no-unused-vars */

type TriggerLabel = {
  label: string;
  icon: React.JSX.Element;
};

interface ModalProps {
    triggerButton: React.ReactNode,
    dialogBody: React.ReactNode,
    dialogTitle: string,
    dialogDescription: string,
}

const Modal = (props: ModalProps) => {
  /* eslint-disable @typescript-eslint/no-unused-vars */
  const { triggerButton, dialogBody, dialogTitle, dialogDescription } = props;
  /* eslint-disable @typescript-eslint/no-unused-vars */

  return (
    <Dialog>
      <DialogTrigger asChild>
        {triggerButton}
      </DialogTrigger>
      <DialogContent className='[&>button]:hidden min-w-[80%] overflow-x-auto rounded'>
        {/* <DialogHeader className='flex flex-row justify-between items-center'>
          <div>
            <DialogTitle>
              {dialogTitle}
            </DialogTitle>
            <DialogDescription className='mt-1'>
              {dialogDescription}
            </DialogDescription>
          </div>
          <Button variant={"default"} className='!mt-0'>
            <ScanEye size={16} />
            <span>Preview</span>
          </Button>
        </DialogHeader> */}
        {/* <hr /> */}
        <div>
          {dialogBody}
        </div>
      </DialogContent>
    </Dialog> 
  )
}

export default Modal