import { useDeleteListMutation } from '@/app/services/ListApi';
import EditListForm from './listForms/EditListForm';
import Modal from '@/components/Modal';
import { Row } from '@tanstack/react-table';
import { Edit3, Trash2 } from 'lucide-react';
import React from 'react'

interface ActionsColumnProps {
    row: Row<any>,
    namespaceId: string,
};

const ActionsColumn = ({ 
    row,
    namespaceId,
}: ActionsColumnProps) => {
    const listId = row.original.id;
    
    const [ deleteList, { isLoading: isDeleting, error: deletionError } ] = useDeleteListMutation();

    const deleteListHandler = async (id: string) => {
        if (deletionError) {
            return <div>Error deleting the list</div>
        }
    
        await deleteList({ namespaceId, listId: id });
    }
    
    return (
        <div className='flex space-x-4 text-primary items-center'>
                {/* <button className='transition-all duration-300 ease-in-out hover:scale-105'><Edit3 size={16} /></button> */}
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
                <button 
                    className='transition-all duration-300 ease-in-out hover:scale-105'
                    onClick={() => deleteListHandler(listId)}
                    >
                    <Trash2 strokeWidth={1.5} size={20} className='text-red-400 hover:text-red-500' />
            </button>
        </div>
    );
}

export default ActionsColumn