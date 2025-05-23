import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:memolanes/src/rust/api/api.dart';
import 'package:memolanes/src/rust/storage.dart';
import 'package:memolanes/utils.dart';
import 'package:share_plus/share_plus.dart';

class RawDataSwitch extends StatefulWidget {
  const RawDataSwitch({super.key});

  @override
  State<RawDataSwitch> createState() => _RawDataSwitchState();
}

class _RawDataSwitchState extends State<RawDataSwitch> {
  bool enabled = false;

  @override
  initState() {
    super.initState();
    getRawDataMode().then((value) => setState(() {
          enabled = value;
        }));
  }

  @override
  Widget build(BuildContext context) {
    return Switch(
      value: enabled,
      activeColor: Colors.red,
      onChanged: (bool value) async {
        await toggleRawDataMode(enable: value);
        setState(() {
          enabled = value;
        });
      },
    );
  }
}

class RawDataPage extends StatefulWidget {
  const RawDataPage({super.key});

  @override
  State<RawDataPage> createState() => _RawDataPage();
}

class _RawDataPage extends State<RawDataPage> {
  List<RawDataFile> items = [];

  @override
  void initState() {
    super.initState();
    _loadList();
  }

  _loadList() async {
    var list = await listAllRawData();
    setState(() {
      items = list;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(context.tr("advance_settings.raw_data_mode")),
      ),
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          const SizedBox(height: 8),
          const RawDataSwitch(),
          const SizedBox(height: 16),
          Expanded(
            child: ListView(
              shrinkWrap: true,
              children: items.map((item) {
                return ListTile(
                  leading: const Icon(Icons.description),
                  title: Text(item.name),
                  onTap: () {
                    Share.shareXFiles([XFile(item.path)]);
                  },
                  trailing: ElevatedButton(
                    onPressed: () async {
                      if (await showCommonDialog(
                          context, context.tr("journey.delete_journey_message"),
                          hasCancel: true,
                          title: context.tr("journey.delete_journey_title"),
                          confirmButtonText: context.tr("journey.delete"),
                          confirmGroundColor: Colors.red,
                          confirmTextColor: Colors.white)) {
                        await deleteRawDataFile(filename: item.name);
                        _loadList();
                      }
                    },
                    child: const Icon(Icons.delete),
                  ),
                );
              }).toList(),
            ),
          ),
        ],
      ),
    );
  }
}
