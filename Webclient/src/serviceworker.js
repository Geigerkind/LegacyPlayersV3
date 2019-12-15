importScripts('https://storage.googleapis.com/workbox-cdn/releases/4.3.1/workbox-sw.js');

// Caching cachable API calls
workbox.routing.registerRoute(
    new RegExp('/API/'),
    new workbox.strategies.CacheFirst({
        cacheName: 'api-cache',
        plugins: [
            new workbox.expiration.Plugin({
                maxEntries: 1000,
                maxAgeSeconds: 2 * 60 * 60, // 2 Hours
            }),
            new workbox.cacheableResponse.Plugin({
                statuses: [200],
                headers: {
                    'X-Is-Cachable': 'true',
                },
            })
        ]
    })
);

// Caching images
workbox.routing.registerRoute(
    /\.(?:png|gif|jpg|jpeg|svg|webp)$/,
    new workbox.strategies.CacheFirst({
        cacheName: 'images',
        plugins: [
            new workbox.expiration.Plugin({
                maxEntries: 1000,
                maxAgeSeconds: 30 * 24 * 60 * 60, // 30 Days
            }),
        ],
    })
);

// Caching js/css primarily
workbox.routing.registerRoute(
    /\.(?:css|html|json)$/,
    new workbox.strategies.StaleWhileRevalidate({
        cacheName: 'static-resources',
    })
);

// Caching the index.html
workbox.routing.registerRoute(
    /\/$/,
    new workbox.strategies.StaleWhileRevalidate({
        cacheName: 'static-resources',
    })
);
