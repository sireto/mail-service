import React from 'react'

import { Button } from '@/components/ui/button'
import { 
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogFooter,
    DialogTrigger 
} from './ui/dialog';
import { ScanEye } from "lucide-react";

const Modal = (props: any) => {
  const { triggerLabel, dialogBody } : {
    triggerLabel: any,
    dialogBody: React.ReactNode
  } = props;
  return (
    <Dialog >
      <DialogTrigger asChild>
        <Button variant={"default"}>
          <>
            {triggerLabel.icon}
            <span className='ml-2'>{triggerLabel.label}</span>
          </>
        </Button>
      </DialogTrigger>
      <DialogContent className='[&>button]:hidden'>
        <DialogHeader className='flex flex-row justify-between items-center'>
          <div>
            <DialogTitle>
              New template
            </DialogTitle>
            <DialogDescription className='mt-1'>
              Add a new template
            </DialogDescription>
          </div>
          <Button variant={"default"} className='!mt-0'>
            <ScanEye size={16} />
            <span>Preview</span>
          </Button>
        </DialogHeader>
        <hr />
        <div>
          {dialogBody}
        </div>
      </DialogContent>
    </Dialog> 
  )
}

export default Modal