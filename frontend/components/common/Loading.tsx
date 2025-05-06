import React from 'react';
import loadingGif from '@/public/loading.gif';
import Image from 'next/image';

const LoadingComponent = () => {
  return (
    <div className='w-full h-full flex justify-center items-center'>
        <Image src={loadingGif} alt={'loading...'} width={60} height={60}/>
    </div>
  )
}

export default LoadingComponent;