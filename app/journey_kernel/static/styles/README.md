# Pastel / 淡彩 (OpenFreeMap)

The app exposes this style as **Pastel** in English and **淡彩** in Chinese.
It uses the existing map-style picker and persisted style selection. Its
internal renderer and storage ID is `pastel`.

`pastel.json` is a DataV4-inspired adaptation of OpenFreeMap Positron,
including road, label and land-cover rules. It uses the existing OpenMapTiles
schema and OpenFreeMap service. It is
an independent adaptation, rather than the MapTiler Dataviz v4 style.

Upstream: [OpenFreeMap Positron at 6e536128](https://github.com/hyperknot/openfreemap-styles/tree/6e5361286c3e1eac7b7c89800ff63218297b5cfd/styles/positron).

Visual reference: the supplied [Dataviz v4 configuration](https://fog.vicc.wang/maptiler/maps/dataviz/style.v4.json?v=260928),
compared at identical centers and zooms 6, 12 and 16. The runtime resources
remain entirely on OpenFreeMap. Feature geometry, available names and coarse
land-cover coverage differ between the datasets.

Also compared [Stargazer's Dataviz Light at 642f355d](https://github.com/CodaBool/stargazer/blob/642f355dee88c2274ee582d2e9e4364c03db0da7/lib/maplibreStyles/dataviz_light.json)
using the same OpenFreeMap source, font service and Chinese label language.
Its gray water, white roads and earlier building display produce a monochrome
map, while the supplied reference uses cyan water, blue-gray roads and pink
place labels. The bundled style keeps the latter palette. Stargazer's default
tile service is CARTO; its resource URLs and style are not bundled here.

Changes from that revision:

- Replaced Positron's white road centers and gray casings with blue-gray road
  centers and white casings. Road class, ramps, bridges and tunnels have separate
  filters, colors and widths. Bridges have pale centers with gray edges, while
  tunnels use translucent strokes. Minor roads stay hidden at overview zooms,
  and service roads use the thin dashed path treatment with a soft outline.
- Replaced route shields and bilingual place labels with small named-road
  labels and one localized place name. City and village labels are pink,
  state names green and water names blue-gray, with zoom-dependent sizes.
- Opted into the renderer's browser-selected map language. Chinese script
  variants fall back to `name:zh`, and untranslated names fall back to `name`.
- Reduced vegetation opacity, separated grass and woodland, faded residential
  fills by zoom and moved building display to zoom 16. Avoided a second park
  tint over the vegetation layers.
- Matched zoom-dependent water displacement, separated intermittent rivers,
  and centered waterbody names without a white halo. Main rail and local
  transit have separate surface, bridge and tunnel treatments.
- Restyled administrative boundaries with light pink solid and dashed lines,
  selected by administrative level and zoom, including the alternating dash
  pattern at zoom 12. Existing boundary and worldview filters are retained.
- Resolved upstream domain placeholders to `tiles.openfreemap.org` for tiles
  and glyphs, and removed the unused sprite and Natural Earth raster source.
- Added the style name, provenance metadata and source attribution.
- Retained OpenMapTiles feature classes and administrative-boundary filters;
  Planet v4 source-layer names and label ranks are not interchangeable with them.

The upstream style code is BSD 3-Clause and the design is CC BY 4.0; see
`pastel.LICENSE.txt` for the complete notice. The original design is credited
to CARTO, Stamen, Paul Norman and OpenMapTiles contributors. The application
exposes this attribution in the map copyright dialog, and webpack copies the
license into the shipped WebView assets.
