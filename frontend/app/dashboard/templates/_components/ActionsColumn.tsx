import { useState } from 'react';
import { useDeleteTemplateMutation } from '@/app/services/TemplateApi';
import Modal from '@/components/Modal';
import EditTemplateForm from './templateForms/EditTemplateForm';
import { Row } from '@tanstack/react-table';
import { Edit3, ScanEye, Trash2 } from 'lucide-react';
import React from 'react';
import ConfirmationPopup from '@/components/common/ConfirmationPopup';
import { Button } from '@/components/ui/button';
import { usePreviewTemplateMutation } from "@/app/services/TemplateApi";
import PreviewFrame from './PreviewFrame';


interface ActionsColumnProps {
    row: Row<any>,
};

const ActionsColumn = ({ row }: ActionsColumnProps) => {
    const templateId = row.original.id;
    
    const [ deleteTemplate, { isLoading: isDeleting, error: deletionError } ] = useDeleteTemplateMutation();
    const [parsedHtml, setParsedHtml] = useState<string | null>(null);
    const [previewTemplate, { isLoading: isPreviewing, error: previewError }] = usePreviewTemplateMutation();
    

    const deleteTemplateHandler = async (id: string) => {
        if (deletionError) {
            return <div>Error deleting the list</div>
        }
    
        await deleteTemplate(id);
    }
    
    const previewHandler = async() => {
        const resultHtml = await previewTemplate({
            mjml: row.original.content_html.trim()
        });

        if (resultHtml && resultHtml.data) setParsedHtml(resultHtml?.data.html);
    };

    return (
        <div className='flex space-x-4 text-primary items-center'>
            <Modal
                triggerButton={
                    <ScanEye size={20} onClick={previewHandler} className='transition-all duration-300 ease-in-out hover:scale-105 cursor-pointer'/>
                }
                dialogBody={<PreviewFrame html={parsedHtml || "Loading..."} modalTitle={row.original.name} modalDescription={"Preview your template"}/>}
                dialogTitle={row.original.name}
                dialogDescription={"Preview your template"}
                classname="min-h-[80%]"
            />
            <Modal 
                triggerButton={<Edit3
                    size={20}
                    strokeWidth={1.5} 
                    className='text-lime-400 transition-all duration-300 ease-in-out hover:scale-105 hover:text-lime-500 cursor-pointer'
                />}
                dialogBody={<EditTemplateForm templateId={templateId}/>}
                dialogTitle={"Edit Template"}
                dialogDescription={"Edit your template"}
            />
            <ConfirmationPopup
                title="Are you sure to delete this template?"
                message="The template will be deleted permanently."
                onConfirm={() => deleteTemplateHandler(templateId)}
                popupTriggerButton = {
                    <Trash2 strokeWidth={1.5} size={20} className='text-red-400 hover:text-red-500 transition-all duration-300 ease-in-out hover:scale-105 cursor-pointer' />
                }
                confirmButton = {
                    <Button variant={"danger"}>Delete</Button>
                }
            />
        </div>
    )
}

export default ActionsColumn