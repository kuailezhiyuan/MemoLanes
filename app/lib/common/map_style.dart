import 'package:memolanes/common/map_fog_style.dart';

/// Map base style: id, renderer style, copyright, and per-fog-style opacity.
/// MMKV stores only [id]. Display labels come from map_settings translations.
class MapStyle {
  const MapStyle({
    required this.id,
    required this.url,
    required this.copyright,
    required this.fogOpacityByStyle,
  });

  final String id;

  /// Remote style URL or a built-in renderer style ID such as `pastel`.
  final String url;
  final String copyright;

  /// Overlay opacity keyed by fog style ID, with values in the inclusive range
  /// 0–1.
  final Map<MapFogStyle, double> fogOpacityByStyle;

  static const _openFreeMapCopyright =
      '[OpenFreeMap](https://openfreemap.org) [© OpenMapTiles](https://www.openmaptiles.org/) Data from [OpenStreetMap](https://www.openstreetmap.org/copyright)';

  // First one is the default.
  static const List<MapStyle> all = [
    MapStyle(
      id: 'openfreemap',
      url: 'https://tiles.openfreemap.org/styles/liberty',
      copyright: _openFreeMapCopyright,
      fogOpacityByStyle: {MapFogStyle.dark: 0.50, MapFogStyle.light: 0.50},
    ),
    MapStyle(
      id: 'pastel',
      url: 'pastel',
      copyright:
          '$_openFreeMapCopyright\n\n'
          'Style adapted from [OpenFreeMap Positron](https://github.com/hyperknot/openfreemap-styles/tree/6e5361286c3e1eac7b7c89800ff63218297b5cfd/styles/positron). '
          'Design © CARTO, Stamen, Paul Norman and OpenMapTiles contributors '
          '([CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)).',
      fogOpacityByStyle: {MapFogStyle.dark: 0.50, MapFogStyle.light: 0.50},
    ),
    MapStyle(
      id: 'maplibre',
      url: 'https://demotiles.maplibre.org/style.json',
      copyright: '[MapLibre](https://maplibre.org/)',
      fogOpacityByStyle: {MapFogStyle.dark: 0.50, MapFogStyle.light: 0.50},
    ),
  ];

  static MapStyle findById(String? id) {
    return all.firstWhere((s) => s.id == id, orElse: () => all[0]);
  }
}
