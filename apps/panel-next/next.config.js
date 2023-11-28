const withNextIntl = require('next-intl/plugin')();

/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export',
    images: {
        remotePatterns: [
            { // for testing images
                protocol: 'https',
                hostname: '**.com',
            },
            { // for testing images
                protocol: 'https',
                hostname: '**.it',
            },
        ],
    },
}

module.exports = withNextIntl(nextConfig)
