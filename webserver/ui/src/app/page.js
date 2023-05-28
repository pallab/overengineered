'use client';
import Image from 'next/image'
import Grid from "./Grid";
import { GridContextProvider } from './GridContext';
import { connectSocket } from './websokcet';
import { useEffect } from 'react';
import { Form } from './Form';

export default function Home() {

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
     <GridContextProvider>
        <div className='outer'>
          {/* <div className='inner'> */}
          <Grid />
          </div>
        {/* </div> */}
      </GridContextProvider>

    </main>
  )
}
