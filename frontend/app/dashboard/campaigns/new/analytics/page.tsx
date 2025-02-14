import DataTable from '@/components/DataTable'
import React from 'react'

const page = () => {
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
            <div className='my-12'>
                {/* <DataTable 
                    data={campaigns || []} 
                    columns={columns(deleteCampaignHandler, startCampaignHandler)}
                    fallback={"No campaigns found"}    
                /> */}
            </div>
        </div>
  )
}

export default page