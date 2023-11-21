import { getRequestConfig } from 'next-intl/server';

export const locales = ['en'];

export default getRequestConfig(async ({ locale }) => ({
	messages: (await import(`../locales/${locale}.json`)).default
}));