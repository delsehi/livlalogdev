import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import 'dart:convert';
import 'package:livla_log/messages/sql_query.pb.dart';
import '../settings/settings_service.dart';

class ChartsView extends StatelessWidget {
  static const routeName = '/charts';
  final String logLocation;
  final String query;
  const ChartsView(this.logLocation, this.query, {super.key});

  static var settings = SettingsService();

  @override
  Widget build(BuildContext context) {

    LogAndQueryInput(
            log:
                // r"C:\Users\DelfiSehidic\Work\Other\livlalog\livla_log_cli\test.txt.bak",
                logLocation,
            query:
                // "SELECT AVG(epley(weight, reps)) as repmax, weight from lifts where weight is not null and reps is not null group by date;")
                // "SELECT date_part('year', to_timestamp(date)) as time FROM lifts group by date_part('year', to_timestamp(date));"
                "SELECT * FROM lifts;"
                // "SELECT epley(weight, reps) repmax, avg(epley(weight, reps)) over (partition by lift) as avgmax, date_trunc('YEAR',date) as year FROM lifts where max_rep = true;")
        ).sendSignalToRust(null);


    return Scaffold(
        appBar: AppBar(
          title: const Text("Charts"),
        ),
        body: Padding(
            padding: const EdgeInsets.all(16),
            child: StreamBuilder(
                stream: QueryResultOutput.rustSignalStream,
                builder: (context, snapshot) {
                  final rustSignal = snapshot.data;
                  if (rustSignal == null) {
                    return Text("Nope nothing");
                  }
                  final message = rustSignal.message;
                  var test = message.json;
                  var schema = message.schema;
                  final List<dynamic> dataList = jsonDecode(test);
                  var headers = List<String>.empty(growable: true);
                  Map<String, dynamic> first = dataList[0];
                  var it = schema.iterator;
                  while (it.moveNext()) {
                    headers.add(it.current);
                  }
                  var it2 = dataList.iterator;
                  var rows = List<DataRow>.empty(growable: true);
                  var lineData = [];
                  while (it2.moveNext()) {
                    var values = [];
                    headers.forEach((element) {
                      values.add(it2.current[element]);
                      lineData.add(it2.current);
                    });
                    rows.add(DataRow(
                        cells: values
                            .map((e) => DataCell(Text(e.toString() != "null" &&
                                    e.toString() != "false"
                                ? e.toString()
                                : "")))
                            .toList()));
                  }
                  var table = DataTable(
                      columns: headers
                          .map((e) => DataColumn(label: Text(e)))
                          .toList(),
                      rows: rows);
                  var chart = LineChart(LineChartData(lineBarsData: [
                    LineChartBarData(
                        spots: lineData
                            .map((e) => FlSpot(
                                // (e[headers[0]] as int).toDouble(),
                                e[headers[1]] is int
                                    ? (e[headers[1]] as int).toDouble()
                                    : 10,
                                e[headers[0]] is double
                                    ? e[headers[0]]
                                    : e[headers[0]] is int
                                        ? (e[headers[0]] as int).toDouble()
                                        : 2))
                            .toList())
                  ]));
                  var tableView = SingleChildScrollView(
                    scrollDirection: Axis.horizontal,
                    child: SingleChildScrollView(
                      child: table,
                    ),
                  );
                  return ListView(
                    children: [
                      // LogEditor(),
                      Padding(
                          padding: const EdgeInsets.all(16), child: tableView),
                    ],
                  );
                })
            ));
  }
}
