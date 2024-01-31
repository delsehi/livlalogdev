import 'package:flutter/material.dart';

class LivlaTable extends StatelessWidget {
  final List<String> headers;
  final List<DataRow> rows;
  const LivlaTable(this.headers, this.rows, {super.key});

  @override
  Widget build(BuildContext context) {
    var table = DataTable(
        columns: headers.map((e) => DataColumn(label: Text(e))).toList(),
        rows: rows);
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: SingleChildScrollView(
        child: table,
      ),
    );
  }
}
