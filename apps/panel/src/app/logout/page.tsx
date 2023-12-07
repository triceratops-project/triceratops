'use client'
import { isUserAuthenticated, logoutUser } from '@/lib/auth';
import { redirect } from 'next/navigation';

export default function LogoutPage({ }) {
	if (isUserAuthenticated()) {
        logoutUser()
		redirect('/');
	} else {
        redirect('/');
    }
}