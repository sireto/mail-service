'use client'

import React, { useEffect } from 'react'
import { TableBody, TableCell, TableRow } from './ui/table'
import { Edit3, ScanEye, Trash2 } from 'lucide-react'
import { formatDate } from '@/lib/utils'
import { EditTemplateForm } from './TemplateForms';
import { useDeleteTemplateMutation, useGetTemplatesQuery } from '@/app/services/TemplateApi';
import Modal from './Modal'

const TemplateList = () => {
    const { data, error, isLoading, refetch } = useGetTemplatesQuery();
    const [ deleteTemplate, { isLoading: isDeleting, error: deletionError } ] = useDeleteTemplateMutation();


    useEffect(() => {
        if (error) {
            console.error(error);
        }
    }, [data]);
    
    if (isLoading) {
        return (
            <TableBody>
                <TableRow>
                    <TableCell colSpan={4} className='text-center'>Loading...</TableCell>
                </TableRow>
            </TableBody>
        )
    }

    const deleteTemplateHandler = async (id: string) => {
        if (deletionError) {
            return <div>Error deleting the template</div>
        }
    
        await deleteTemplate(id);
        refetch();
    }

  return (
    <TableBody>
        {
            data?.map(template => (
                <TableRow key={template.id} className='cursor-pointer'>
                    <TableCell className='text-primary'>{template.name}</TableCell>
                    <TableCell>{formatDate(template.created_at)}</TableCell>
                    <TableCell>{formatDate(template.updated_at)}</TableCell>
                    <TableCell >
                        <div className='flex space-x-4 text-primary items-center'>
                            <button className='transition-all duration-300 ease-in-out hover:scale-105'><ScanEye size={20} /></button>
                            {/* <button className='transition-all duration-300 ease-in-out hover:scale-105'><Edit3 size={16} /></button> */}
                            <Modal 
                                triggerButton={<Edit3 size={20} />} 
                                dialogBody={<EditTemplateForm templateId={template.id}/>}
                                dialogTitle={"Edit Template"}
                                dialogDescription={"Edit your template"}
                            />
                            <button 
                                className='transition-all duration-300 ease-in-out hover:scale-105'
                                onClick={() => {
                                    deleteTemplateHandler(template.id);
                                }}
                            ><Trash2 size={20} /></button>
                        </div>
                    </TableCell>
                </TableRow>
            ))
        }
    </TableBody>
  )
}

export default TemplateList