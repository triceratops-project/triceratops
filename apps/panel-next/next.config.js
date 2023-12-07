/** @type {import('next').NextConfig} */
const nextConfig = {
    output: 'export',
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
