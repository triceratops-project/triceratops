/** @type {import('next').NextConfig} */
const nextConfig = {
    i18n: {
        locales: ['en'],
        defaultLocale: 'en',
    },
    images: {
        remotePatterns: [
            { // for testing images
                protocol: 'https',
                hostname: '**.com',
            },
        ],
    },
}

module.exports = nextConfig
