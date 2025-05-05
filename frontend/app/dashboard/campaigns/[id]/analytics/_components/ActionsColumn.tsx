import React from 'react'
import { Row } from '@tanstack/react-table';
import { Trash2 } from 'lucide-react';
import ConfirmationPopup from '@/components/common/ConfirmationPopup';
import { Button } from '@/components/ui/button';
import { Mail } from '@/lib/type/mail';

interface ActionsColumnProps {
    row: Row<Mail>,
    deleteMailHandler: (id: string) => void,
};

const ActionsColumn = ({ 
    row,
    deleteMailHandler,
}: ActionsColumnProps) => {
    const mailId = row.original.id;

    return (
        <div className='flex space-x-4 text-primary items-center'>
            <ConfirmationPopup
                title="Are you sure to delete this campaign?"
                message="The campaign will be deleted permanently."
                onConfirm={() => deleteMailHandler(mailId)}
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