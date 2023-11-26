import type { Props } from '@/page.types'
import type { Metadata } from 'next'

import '@/app/global.css'

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