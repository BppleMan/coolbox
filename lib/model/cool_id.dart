import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:flutter/foundation.dart';

part 'cool_id.freezed.dart';

@freezed
class CoolID with _$CoolID {
  // final String name;
  // final String version;
  //
  // CoolID({
  //   required this.name,
  //   required this.version,
  // });

  const factory CoolID({
    required String name,
    required String version,
  }) = _CoolID;
}

extension CoolIDExtension on CoolID {
  toTomlValue() {
    return {
      "name": name,
      "version": version,
    };
  }
}
