import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';

class ChartsView extends StatelessWidget {
  static const routeName = '/charts';

  const ChartsView({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Charts"),
      ),
      body: Padding(
          padding: const EdgeInsets.all(16),
          child:
              // LineChart(LineChartData(
              //   lineBarsData: [
              //     LineChartBarData(spots: const [
              //       FlSpot(1, 1),
              //       FlSpot(2, 3),
              //       FlSpot(4, 8),
              //     ])
              //   ]
              // ))
              DataTable(columns: const <DataColumn>[
                DataColumn(label: Text("Kolumn 1"))
              ],
              rows: const <DataRow>[
                DataRow(cells: <DataCell>[
                  DataCell(Text("Hello")),
                ])
              ])
    ));
  }
}
