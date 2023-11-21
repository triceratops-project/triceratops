import createMiddleware from 'next-intl/middleware';
import locales from '@/i18n'

export default createMiddleware({
  locales: ['en'],

  defaultLocale: 'en'
});

export const config = {
  matcher: ['/', '/(en)/:path*']
};