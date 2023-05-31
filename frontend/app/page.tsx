"use client"

import Link from 'next/link'

export default function Home() {
  return (
    <main className="container flex min-h-screen flex-col items-center justify-between p-24">

      <div className="mb-32 flex text-center lg:mb-0 lg:grid-cols-4 lg:text-left">
        <Link
          href="/journeys"
          className="rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Journeys{' '}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Explore Journeys
          </p>
        </Link>

        <Link
          href="/stations"
          className="rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800 hover:dark:bg-opacity-30"
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Stations{' '}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Explore Stations!
          </p>
        </Link>


      </div>
    </main>
  )
}
