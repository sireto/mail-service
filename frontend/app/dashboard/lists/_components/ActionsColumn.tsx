import { useDeleteListMutation } from '@/app/services/ListApi';
import EditListForm from './listForms/EditListForm';
import Modal from '@/components/Modal';
import { Row } from '@tanstack/react-table';
import { Edit3, Trash2 } from 'lucide-react';
import React from 'react'
import ConfirmationPopup from '@/components/common/ConfirmationPopup';
import { Button } from '@/components/ui/button';
import { List } from '@/lib/type/list';

interface ActionsColumnProps {
    row: Row<List>,
    namespaceId: string,
};

const ActionsColumn = ({ 
    row,
    namespaceId,
}: ActionsColumnProps) => {
    const listId = row.original.id;
    
    const [ deleteList, { error: deletionError } ] = useDeleteListMutation();

    const deleteListHandler = async (id: string) => {
        if (deletionError) {
            return <div>Error deleting the list</div>
        }
    
        await deleteList({ namespaceId, listId: id });
    }
    
    return (
        <div className='flex space-x-4 text-primary items-center'>
                <Modal 
                    triggerButton={<Edit3 
                        size={20} 
                        strokeWidth={1.5} 
                        className='text-lime-400 hover:text-lime-500'
                    />} 
                    dialogBody={<EditListForm listId={listId}/>}
                    dialogTitle={"Edit Template"}
                    dialogDescription={"Edit your template"}
                />
                <ConfirmationPopup
                    title="Are you sure to delete this list?"
                    message="The list will be deleted permanently."
                    onConfirm={() => deleteListHandler(listId)}
                    popupTriggerButton  = {
                        <Trash2 strokeWidth={1.5} size={20} className='text-red-400 hover:text-red-500 transition-all duration-300 ease-in-out hover:scale-105 cursor-pointer' />
                    }
                    confirmButton = {
                        <Button variant={"danger"}>Delete</Button>
                    }
                />
        </div>
    );
}

export default ActionsColumn