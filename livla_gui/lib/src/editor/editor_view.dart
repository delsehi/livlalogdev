import 'package:flutter/material.dart';
import 'package:livla_log/src/charts/charts.dart';
import 'editor.dart';
import '../settings/settings_view.dart';
class EditorView extends StatelessWidget {
  const EditorView({
    super.key,
  });

  static const routeName = '/';
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SQL Query'),
        actions: [
          IconButton(
            icon: const Icon(Icons.data_thresholding_sharp),
            onPressed: () {
              Navigator.restorablePushNamed(context, ChartsView.routeName);
            },
          ),
          IconButton(
            icon: const Icon(Icons.settings),
            onPressed: () {
              // Navigate to the settings page. If the user leaves and returns
              // to the app after it has been killed while running in the
              // background, the navigation stack is restored.
              Navigator.restorablePushNamed(context, SettingsView.routeName);
            },
          ),
        ],
      ),

      // To work with lists that may contain a large number of items, it’s best
      // to use the ListView.builder constructor.
      //
      // In contrast to the default ListView constructor, which requires
      // building all Widgets up front, the ListView.builder constructor lazily
      // builds Widgets as they’re scrolled into view.
      body: Container( 
        child: LogEditor(),
      ),
      // body: ListView.builder(
      //   // Providing a restorationId allows the ListView to restore the
      //   // scroll position when a user leaves and returns to the app after it
      //   // has been killed while running in the background.
      //   restorationId: 'editorView',
      //   itemCount: 1,
      //   itemBuilder: (BuildContext context, int index) {
      //     // final item = items[index];

      //     return ListTile(
      //         title: Text('SampleItem ${item.id}'),
      //         leading: const CircleAvatar(
      //           // Display the Flutter Logo image asset.
      //           foregroundImage: AssetImage('assets/images/flutter_logo.png'),
      //         ),
      //         onTap: () {
      //           // Navigate to the details page. If the user leaves and returns to
      //           // the app after it has been killed while running in the
      //           // background, the navigation stack is restored.
      //           Navigator.restorablePushNamed(
      //             context,
      //             SampleItemDetailsView.routeName,
      //           );
      //         });
      //   },
      // ),
    );
  }
}
