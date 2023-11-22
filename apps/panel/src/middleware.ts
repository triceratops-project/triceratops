import { NextRequest, NextResponse } from 'next/server'
import createMiddleware from 'next-intl/middleware';

const middleware = createMiddleware({
  locales: ['en'],
  defaultLocale: 'en'
});

export default function onRequest(req: NextRequest) {
  if (req.nextUrl.pathname.startsWith('/_next') || req.nextUrl.pathname.startsWith('/public')) {
    return NextResponse.next();
  }

  return middleware(req);
}

export const config = {
  matcher: ['/:locale/:path*']
};