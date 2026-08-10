import { usePreviewTemplateMutation } from '@/app/services/TemplateApi';
import LoadingComponent from '@/components/common/Loading';
import { Button } from '@/components/ui/button'
import { DialogClose, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog'
import React, { useEffect } from 'react'


const PreviewFrame =  ({ mjml, modalTitle, modalDescription }: { mjml: string, modalTitle: string, modalDescription: string }) => {
  const [previewTemplate, {data, error, isLoading}] = usePreviewTemplateMutation();
  useEffect(() => {
    previewTemplate({mjml});
  }, [mjml, previewTemplate]);

  const resultHtml = data;

  if(error) {
    return <div>There was an error previewing the template...</div>
  }
  if(isLoading) {
    return <div className='w-full h-full flex items-center justify-center flex-col'>
      <LoadingComponent />
      <span>Parsing the template...</span>
    </div>
  } 

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
            srcDoc={resultHtml?.html ?? 'Loading...'}
        />
      </div>
      <DialogClose asChild>
        <Button type="button" variant={"outline"} className='w-full'>Close</Button>
      </DialogClose>
    </div>
  )
}

export default PreviewFrame
