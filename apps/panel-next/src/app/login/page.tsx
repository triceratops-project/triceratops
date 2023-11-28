'use client'
import LoginContainer from '@components/Login/LoginContainer'
import Triceratops from '@/components/Triceratops'
import { redirect } from 'next/navigation';
import { isUserAuthenticated } from '@/lib/auth';

export default function LoginPage({ }) {
	if (isUserAuthenticated()) {
		redirect('/dashboard');
	}

	return (
		<>
			<div className="hidden xl:block">
				<Triceratops />
			</div>

			<div className="">
				<div className="">
					<LoginContainer />
				</div>
			</div>
		</>
	)
}