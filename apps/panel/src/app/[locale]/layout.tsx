import { notFound } from 'next/navigation';
import { locales } from '@/i18n';

import type { Props } from '@/page.types'
import type { Metadata } from 'next'

import '@/app/global.css'

export const metadata: Metadata = {
	title: 'Triceratops',
	description: 'Triceratops',
}

export default function LocaleLayout({ children, params: { locale } }: Props) {
	if (!locales.includes(locale as any)) notFound();

	return (
		<html lang={locale}>
			<head />
			<body>
				{children}
			</body>
		</html>
	)
}