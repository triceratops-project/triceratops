import type { Props } from '@/page.types'
import type { Metadata } from 'next'

import '@/app/global.css'

import { Cabin } from 'next/font/google'

const cabin = Cabin({
	display: 'swap',
	subsets: ['latin'],
	variable: '--font-cabin'
})


export const metadata: Metadata = {
	title: 'Triceratops',
	description: 'Triceratops',
}

export default function RootLayout({ children, params: { locale } }: Props) {

	return (
		<html lang={locale}>
			<head />
			<body>
				{children}
			</body>
		</html>
	)
}