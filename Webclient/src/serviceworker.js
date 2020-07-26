importScripts('https://storage.googleapis.com/workbox-cdn/releases/4.3.1/workbox-sw.js');

// Caching cachable API calls
/*
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
            }),
            new workbox.cacheableResponse.Plugin({
                statuses: [0, 200],
            }),
        ]
    })
);
 */

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
            new workbox.cacheableResponse.Plugin({
                statuses: [0, 200],
            }),
        ],
    })
);

// Caching js/css primarily
workbox.routing.registerRoute(
    /\.(?:css|html|json|js)$/,
    new workbox.strategies.StaleWhileRevalidate({
        cacheName: 'static-resources',
        plugins: [
            new workbox.cacheableResponse.Plugin({
                statuses: [0, 200],
            }),
        ],
    })
);

// Caching the index.html
workbox.routing.registerRoute(
    /[a-zA-Z0-9]+$/,
    new workbox.strategies.StaleWhileRevalidate({
        cacheName: 'static-resources',
        plugins: [
            new workbox.cacheableResponse.Plugin({
                statuses: [0, 200],
            }),
        ],
    })
);

