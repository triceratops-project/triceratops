'use client'
import Logo from "@/components/Logo"
import Triceratops from "@/components/Triceratops"
import { useState } from "react"

export default function Sidebar({ }) {
    const [sidebarOpen, setSidebarOpen] = useState(false)

    return (
        <>
            <div className="bg-neutral-900 max-w-sm min-w-max h-screen rounded-tr-xl rounded-br-xl">
                <div className="flex justify-between p-6">
                    <div className="flex items-center">
                        <div className="relative w-14 h-14">
                            <Logo />
                        </div>
                        <h1 className="ml-3 mt-1 text-3xl font-bold">Triceratops</h1>
                    </div>
                    <div className="mt-5">
                        <button onClick={async () => {
                            setSidebarOpen(!sidebarOpen)
                        }}>
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                                <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </>
    )
}