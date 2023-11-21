import { Props } from '@/app/page.types'
import type { Metadata } from 'next'

import './global.css'

export const metadata: Metadata = {
	title: 'Triceratops',
	description: 'Triceratops',
}

export default function RootLayout({ children }: Props) {
	return (
		<html>
			<head />
			<body>
				{children}
			</body>
		</html>
	)
}