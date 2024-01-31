import 'package:flutter/material.dart';
import '../charts/charts.dart';
import '../settings/settings_service.dart';

class LogEditor extends StatelessWidget {
  LogEditor({super.key});
  static var settings = SettingsService();

  final inputController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    // return TextField(
    //   controller: inputController,
    // );
    return ListView(children: [
      TextField(
        controller: inputController,
      ),
      FloatingActionButton(
          child: const Text("Query"),
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
