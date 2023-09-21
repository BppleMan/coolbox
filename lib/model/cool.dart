import 'package:flutter/foundation.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

import 'cool_id.dart';
import 'package_manager.dart';
import 'tasks.dart';

part 'cool.freezed.dart';

@freezed
class Cool with _$Cool {
  factory Cool({
    required CoolID id,
    required List<CoolID> dependencies,
    required Tasks tasks,
    PackageManager? packageManager,
  }) = _Cool;

  factory Cool.fromTomlValue(Map<String, dynamic> document) {
    return Cool(
      id: CoolID(
        name: document["name"],
        version: document["version"],
        description: document["description"],
      ),
      dependencies: document["dependencies"]
          .map<CoolID>((e) => CoolID.fromTomlValue(e))
          .toList(),
      tasks: Tasks.fromTomlValue(document["tasks"]),
      packageManager: document.containsKey("packageManager")
          ? PackageManager.fromTomlValue(document["packageManager"])
          : null,
    );
  }
}

extension CoolExtension on Cool {
  toTomlValue() {
    return {
      "name": id.name,
      "version": id.version,
      "description": id.description,
      "dependencies": dependencies.map((e) => e.toTomlValue()).toList(),
      "tasks": tasks.toTomlValue(),
      if (packageManager != null) "packageManager": packageManager!.name,
    };
  }
}
