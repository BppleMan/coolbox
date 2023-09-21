import 'package:flutter/foundation.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'cool_id.freezed.dart';

@freezed
class CoolID with _$CoolID {
  const factory CoolID({
    required String name,
    required String version,
    required String description,
  }) = _CoolID;

  factory CoolID.fromTomlValue(Map<String, dynamic> document) {
    return CoolID(
      name: document["name"],
      version: document["version"],
      description: document["description"],
    );
  }
}

extension CoolIDExtension on CoolID {
  toTomlValue() {
    return {
      "name": name,
      "version": version,
      "description": description,
    };
  }
}
