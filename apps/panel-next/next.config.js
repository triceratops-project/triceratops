/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        unoptimized: true,
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

module.exports = nextConfig
