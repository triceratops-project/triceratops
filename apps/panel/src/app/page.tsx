'use client'
import { isUserAuthenticated } from '@/lib/auth';
import { redirect } from 'next/navigation';

export default function Home({ }) {
	if (isUserAuthenticated()) {
		return redirect('/dashboard');
	}

	return redirect('/login');
}