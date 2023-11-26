import { NextRequest, NextResponse } from 'next/server'
import createMiddleware from 'next-intl/middleware';

const middleware = createMiddleware({
    locales: ['en'],
    defaultLocale: 'en'
});

const routes = ['/_next', '/public', '/assets'];
const files = ['/favicon.ico'];

export default function onRequest(req: NextRequest) {
    if (routes.some(route => req.nextUrl.pathname.startsWith(route)) || files.includes(req.nextUrl.pathname)) {
        return NextResponse.next();
    }

    return middleware(req);
}

export const config = {
    matcher: ['/:locale/:path*']
};