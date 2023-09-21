import 'dart:io';

import 'package:coolbox/model/cool.dart';
import 'package:coolbox/model/cool_id.dart';
import 'package:dio/dio.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/http.dart' show Client;
import 'package:native_dio_adapter/native_dio_adapter.dart';
import 'package:toml/toml.dart';

void main() async {
  testWidgets("Test http", (WidgetTester tester) async {
    Cool brew = const Cool(
      id: CoolID(name: "brew", version: "4.1.12"),
      description: "包管理器",
      dependencies: [],
    );
    Cool wget = Cool(
      id: const CoolID(name: "wget", version: "1.0.0"),
      description: "下载器",
      dependencies: [brew],
    );

    var toml = TomlDocument.fromMap(wget.toTomlValue());
    print(toml);
  });
}
