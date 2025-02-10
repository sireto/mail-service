import React from 'react'

import Modal from '@/components/Modal';
import { Plus } from "lucide-react";
import { AddTemplateForm } from '@/components/TemplateForms';

const page = () => {
    const addBtn = {
        label: "Add new",
        icon: <Plus size={24} />
    };

  return (
    <div className='border-2 border-red-700'>
        {/* Template page heading... */}
        <div className='w-full flex justify-between items-center'>
            <h1>Templates</h1>
            <Modal 
                triggerLabel={addBtn} 
                dialogBody={<AddTemplateForm />}
            />
        </div>
        <div>
            {/* Template page content... */}
            <div>

            </div>
        </div>
    </div>
  )
}

export default page