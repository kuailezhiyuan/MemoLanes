import { JourneyBitmap } from '../pkg';
import { JourneyCanvasLayer } from './journey-canvas-layer.js';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

let currentJourneyLayer;  // Store reference to current layer
let pollingInterval;      // Store reference to polling interval
let locationMarker = null;

function getJourneyFilePathFromHash() {
    const hash = window.location.hash.slice(1);
    const params = new URLSearchParams(hash);
    return params.get('journey_id') ? `journey/${params.get('journey_id')}` : '.';
}

async function loadJourneyData(useIfNoneMatch = false) {
    const path = getJourneyFilePathFromHash();
    const filename = `${path}/journey_bitmap.bin`;
    console.log(`Fetching ${filename}`);
    const fetchOptions = {
        headers: useIfNoneMatch ? { 'If-None-Match': '*' } : {}
    };

    const response = await fetch(`${filename}`, fetchOptions);

    // If server returns 304 Not Modified, return null
    if (response.status === 304) {
        return null;
    }

    const arrayBuffer = await response.arrayBuffer();
    const journeyBitmap = JourneyBitmap.from_bytes(new Uint8Array(arrayBuffer));
    console.log(`Loaded ${filename}`);

    // Try to fetch provisioned camera location
    let cameraOptions = null;
    try {
        const cameraResponse = await fetch(`${path}/provisioned_camera_option`);
        if (cameraResponse.ok) {
            const cameraData = await cameraResponse.json();
            cameraOptions = {
                center: [cameraData.lng, cameraData.lat],
                zoom: cameraData.zoom
            };
            console.log('Using provisioned camera location:', cameraData);
        }
    } catch (error) {
        console.log('No provisioned camera location available:', error);
    }

    return { journeyBitmap, cameraOptions };
}

async function pollForUpdates(map, cached = false) {
    try {
        const result = await loadJourneyData(cached);
        if (result) {
            if (result.journeyBitmap) {
                console.log('Update detected, updating journey bitmap');
                currentJourneyLayer.updateJourneyBitmap(result.journeyBitmap);
            }
            if (result.cameraOptions) {
                console.log('Camera update detected, flying to new location');
                map.flyTo(result.cameraOptions);
            }
        }
    } catch (error) {
        console.error('Error polling for updates:', error);
    }
}

async function initializeMap() {
    // Load Mapbox token from .token.json
    const tokenResponse = await fetch('./token.json');
    const tokenData = await tokenResponse.json();
    mapboxgl.accessToken = tokenData['MAPBOX-ACCESS-TOKEN'];

    const hash = window.location.hash.slice(1);

    // Parse hash parameters for initial map view
    let initialView = {
        center: [0, 0],
        zoom: 2
    };

    if (hash) {
        const params = new URLSearchParams(hash);
        const lng = parseFloat(params.get('lng'));
        const lat = parseFloat(params.get('lat'));
        const zoom = parseFloat(params.get('zoom'));

        if (!isNaN(lng) && !isNaN(lat) && !isNaN(zoom)) {
            initialView = {
                center: [lng, lat],
                zoom: zoom
            };
        }
    }

    const map = new mapboxgl.Map({
        container: 'map',
        style: 'mapbox://styles/mapbox/streets-v12',
        center: initialView.center,
        zoom: initialView.zoom,
        maxZoom: 14,
        antialias: true,
        projection: 'mercator',
        pitch: 0,
        pitchWithRotate: false,
        touchPitch: false,
    });
    map.dragRotate.disable();
    map.touchZoomRotate.disableRotation();

    // start loading the initial version journey data
    const loadingInitJourneyData = loadJourneyData();

    map.on('style.load', async (e) => {
        // Create a DOM element for the marker
        const el = document.createElement('div');
        el.className = 'location-marker';

        // Create the marker but don't add it to the map yet
        locationMarker = new mapboxgl.Marker(el);

        // Add method to window object to update marker position
        window.updateLocationMarker = function (lng, lat, show = true, flyto = false) {
            if (show) {
                locationMarker.setLngLat([lng, lat]).addTo(map);
                if (flyto) {
                    const currentZoom = map.getZoom();
                    map.flyTo({
                        center: [lng, lat],
                        zoom: currentZoom < 14 ? 16 : currentZoom,
                        essential: true
                    });
                }
            } else {
                locationMarker.remove();
            }
        };

        const result = await loadingInitJourneyData;

        if (result) {
            const { journeyBitmap, cameraOptions } = result;

            // Update initial camera position only if cameraOptions is provided
            if (cameraOptions) {
                map.setCenter(cameraOptions.center);
                map.setZoom(cameraOptions.zoom);
            }

            // Create and store journey layer
            currentJourneyLayer = new JourneyCanvasLayer(map, journeyBitmap);

            map.addSource("main-canvas-source", currentJourneyLayer.getSourceConfig());
            map.addLayer({
                id: "main-canvas-layer",
                source: "main-canvas-source",
                type: "raster",
                paint: {
                    "raster-fade-duration": 0,
                },
            });
            currentJourneyLayer.render();

            map.on("move", () => currentJourneyLayer.render());
            map.on("moveend", () => currentJourneyLayer.render());

            // Set up polling for updates
            pollingInterval = setInterval(() => pollForUpdates(map, true), 1000);

            // give the map a little time to render before notifying Flutter
            setTimeout(() => {
                if (window.readyForDisplay) {
                    window.readyForDisplay.postMessage('');
                }
            }, 200);
        }
    });

    // Replace the simple movestart listener with dragstart
    map.on('dragstart', () => {
        // Only notify Flutter when user drags the map
        if (window.onMapMoved) {
            window.onMapMoved.postMessage('');
        }
    });

    // Listen for zoom changes
    map.on('zoomstart', (event) => {
        let fromUser = event.originalEvent && (event.originalEvent.type !== 'resize')
        if (fromUser && window.onMapMoved) {
            window.onMapMoved.postMessage('');
        }
    });

    // Add method to window object to get current map view
    window.getCurrentMapView = function () {
        const center = map.getCenter();
        return JSON.stringify({
            lng: center.lng,
            lat: center.lat,
            zoom: map.getZoom()
        });
    };

    window.addEventListener('hashchange', () => pollForUpdates(map, false));

    // Add method to window object to trigger manual update
    window.triggerJourneyUpdate = function () {
        return pollForUpdates(map);
    };
}

// Start initialization
initializeMap().catch(console.error);