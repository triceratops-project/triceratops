'use client'
import RegisterContainer from '@/components/Register/RegisterContainer'
import Triceratops from '@/components/Triceratops'
import { isUserAuthenticated } from '@/lib/auth';
import { redirect } from 'next/navigation';

export default function RegisterPage({ }) {
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
					<RegisterContainer />
				</div>
			</div>
		</>
	)
}