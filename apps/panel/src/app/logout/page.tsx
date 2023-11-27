'use client'
import { isUserAuthenticated, logoutUser } from '@/lib/auth';
import { redirect } from 'next/navigation';

export default function LoginPage({ }) {
	if (isUserAuthenticated()) {
        logoutUser()
		redirect('/');
	} else {
        redirect('/');
    }
}