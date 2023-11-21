const withNextIntl = require('next-intl/plugin')();

/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        remotePatterns: [
            { // for testing images
                protocol: 'https',
                hostname: '**.com',
            },
        ],
    },
}

module.exports = withNextIntl(nextConfig)
