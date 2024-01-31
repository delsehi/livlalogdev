import 'package:flutter/material.dart';
import '../charts/charts.dart';
import '../settings/settings_service.dart';

class LogEditor extends StatelessWidget {
  LogEditor({super.key});
  static var settings = SettingsService();

  final inputController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return ListView(children: [
      Padding(
        padding: const EdgeInsets.all(20),
        child: TextField(
          keyboardType: TextInputType.multiline,
          maxLines: null,
          controller: inputController,
        ),
      ),
      FloatingActionButton(
          child: const Text("Run query"),
          onPressed: () {
            Navigator.push(
                context,
                MaterialPageRoute(
                    builder: (context) => ChartsView(
                        r"C:\Users\DelfiSehidic\Work\Other\livlalog\livla_log_cli\test.txt.bak",
                        inputController.text)));
          })
    ]);
  }
}
