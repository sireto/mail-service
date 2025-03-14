import React from 'react'
import { 
    Dialog,
    DialogContent,
    DialogTrigger, 
} from './ui/dialog';

type TriggerLabel = {
  label: string;
  icon: React.JSX.Element;
};

interface ModalProps {
    triggerButton: React.ReactNode,
    dialogBody: React.ReactNode,
    dialogTitle: string,
    dialogDescription: string,
    classname?: string,
}

const Modal = (props: ModalProps) => {
  /* eslint-disable @typescript-eslint/no-unused-vars */
  const { triggerButton, dialogBody, dialogTitle, dialogDescription, classname = "" } = props;
  /* eslint-disable @typescript-eslint/no-unused-vars */

  return (
    <Dialog>
      <DialogTrigger asChild>
        { triggerButton }
      </DialogTrigger>
      <DialogContent className={`[&>button]:hidden min-w-[80%] overflow-x-auto rounded lg:min-w-[444px] lg:max-w-[888px] ${classname}`}>
        <div>
          {dialogBody}
        </div>
      </DialogContent>
    </Dialog> 
  )
}

export default Modal