import 'package:coolbox/builder/toml.dart';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:flutter/foundation.dart';

import 'cool_id.dart';

part 'cool.freezed.dart';

@freezed
@toml
class Cool with _$Cool {
  // final CoolID id;
  // final String description;
  // final List<CoolID> dependencies;
  //
  // Cool({
  //   required this.id,
  //   required this.description,
  //   required this.dependencies,
  // });

  const factory Cool({
    required CoolID id,
    required String description,
    required List<Cool> dependencies,
    String? packageManager,
  }) = _Cool;
}

extension CoolExtension on Cool {
  toTomlValue() {
    return {
      "id": id.toTomlValue(),
      "description": description,
      "dependencies": dependencies.map((e) => e.id.toTomlValue()).toList(),
      if (packageManager != null) "packageManager": packageManager,
    };
  }
}
