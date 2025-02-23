import React from 'react'
import { Row } from '@tanstack/react-table';
import { Edit3, Trash2 } from 'lucide-react';
import Link from 'next/link';
import ConfirmationPopup from '@/components/common/ConfirmationPopup';
import { Button } from '@/components/ui/button';

interface ActionsColumnProps {
    row: Row<any>,
    deleteMailHandler: (id: string) => void,
};

const ActionsColumn = ({ 
    row,
    deleteMailHandler,
}: ActionsColumnProps) => {
    const mailId = row.original.id;

    return (
        <div className='flex space-x-4 text-primary items-center'>
            {/* <Link href={`/dashboard/campaigns/new`}>
                <Edit3 
                    size={20} 
                    strokeWidth={1.5} 
                    className='text-lime-400 hover:text-lime-500 transition-all duration-300 ease-in-out hover:scale-105'
                />
            </Link> */}
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