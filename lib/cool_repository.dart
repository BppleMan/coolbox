import 'dart:async';

import 'package:coolbox/model/cool.dart';

class CoolRepository {
  const CoolRepository._();

  static const CoolRepository _instance = CoolRepository._();

  factory CoolRepository() => _instance;

  Stream<Cool> getCools() {
    var sb = StreamController<Cool>();
    return sb.stream;
  }
}
