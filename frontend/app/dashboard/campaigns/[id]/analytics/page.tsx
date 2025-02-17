'use client';

import { useGetMailsQuery } from '@/app/services/MailApi'
import DataTable from '@/components/DataTable'
import columns from './_columns';
import React from 'react'

const page = () => {
    const { data: mails, isLoading, error } = useGetMailsQuery();

    if (error) {
        return <div>There was an error fetching mails...</div>
    }

    const deleteMailHandler = async (id: string ) => {

    }

  return (
    <div className=''>
            {/* Template page heading... */}
            <div className='w-full flex justify-between items-center'>
                <h1 className='text-xl font-bold'>
                    Analytics
                </h1>
                {/* <Link href={'/dashboard/campaigns/new'}>
                    { addButton }
                </Link> */}
            </div>
            <div className='my-4'>
                <DataTable 
                    data={mails || []} 
                    columns={columns(deleteMailHandler)}
                    fallback={"No mails found"}   
                    isLoading={isLoading} 
                />
            </div>
        </div>
  )
}

export default page