import { Button } from '@/components/ui/button'
import { DialogClose, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog'
import React from 'react'

const PreviewFrame = ({ html, modalTitle, modalDescription }: { html: string, modalTitle: string, modalDescription: string }) => {
  return (
    <div className='flex flex-col h-full gap-y-4'>
      <DialogHeader className='flex flex-row justify-between items-center'>
        <div>
          <DialogTitle>
            {modalTitle}
          </DialogTitle>
          <DialogDescription className='mt-1 mb-4'>
            {modalDescription}
          </DialogDescription>
        </div>
      </DialogHeader>
      <div className='p-1 w-full h-full'>
        <iframe
            className='w-full h-full bg-gray-50 shadow-lg rounded-md'
            srcDoc={html}
        />
      </div>
      <DialogClose asChild>
        <Button type="button" variant={"outline"} className='w-full'>Close</Button>
      </DialogClose>
    </div>
  )
}

export default PreviewFrame
