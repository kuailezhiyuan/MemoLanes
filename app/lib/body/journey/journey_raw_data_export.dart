import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:memolanes/common/component/common_export.dart';
import 'package:memolanes/common/simple_date_utils.dart';
import 'package:memolanes/src/rust/api/api.dart' as api;
import 'package:memolanes/src/rust/journey_header.dart';
import 'package:path/path.dart' as path;
import 'package:path_provider/path_provider.dart';

Future<CommonExportResult> _generateJourneyRawDataExport(
  JourneyHeader journey,
  CommonExportFormat format,
) async {
  final temporaryDirectory = await getTemporaryDirectory();
  final date = journey.journeyDate.toSimpleDate().toString();
  final outputPath = path.join(
    temporaryDirectory.path,
    '$date-${journey.revision}-raw.${format.extension}',
  );
  final exportType = switch (format) {
    CommonExportFormat.rawDataCsv => api.RawDataExportType.csv,
    CommonExportFormat.gpx => api.RawDataExportType.gpx,
    CommonExportFormat.kml => api.RawDataExportType.kml,
    CommonExportFormat.mldx || CommonExportFormat.fwss =>
      throw UnsupportedError('Unsupported raw-data export format: $format'),
  };
  final result = await api.exportJourneyRawData(
    targetFilepath: outputPath,
    journeyId: journey.id,
    exportType: exportType,
  );
  return CommonExportResult.create(result, outputPath);
}

Future<void> showJourneyRawDataExportPicker(
  BuildContext context,
  JourneyHeader journey,
) async {
  await showCommonExportWithFormatPicker(
    context: context,
    title: context.tr('journey.export_raw_data'),
    formats: const [
      CommonExportFormat.rawDataCsv,
      CommonExportFormat.gpx,
      CommonExportFormat.kml,
    ],
    exportFile: (format) => _generateJourneyRawDataExport(journey, format),
    lossyFormatWarning: context.tr('journey.raw_data_lossy_format_warning'),
  );
}
