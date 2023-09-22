import 'package:coolbox/builder/annotation.dart';
import 'package:flutter/foundation.dart';
import 'package:toml/toml.dart';

import 'cool_id.dart';
import 'tasks.dart';

part 'cool.data.dart';

@data
abstract class ICool implements TomlEncodableValue {
  @Toml(
    from: "CoolID.fromTomlValue({})",
    to: "id.toTomlValue()",
  )
  CoolID id;

  @Toml(
    from: "{}.map<CoolID>((e) => CoolID.fromTomlValue(e)).toList()",
    to: "dependencies.map((e) => e.toTomlValue()).toList()",
  )
  List<CoolID> dependencies;

  @Toml(
    from: "Tasks.fromTomlValue({})",
    to: "tasks.toTomlValue()",
  )
  Tasks tasks;

  ICool({
    required this.id,
    required this.dependencies,
    required this.tasks,
  });
}
// factory Cool.fromTomlValue(Map<String, dynamic> document) {
//   return Cool(
//     id: CoolID.fromTomlValue(document["id"]),
//     dependencies: document["dependencies"]
//         .map<CoolID>((e) => CoolID.fromTomlValue(e))
//         .toList(),
//     tasks: Tasks.fromTomlValue(document["tasks"]),
//   );
// }

// extension CoolExtension on Cool {
//   toTomlValue() {
//     return {
//       "id": id.toTomlValue(),
//       "dependencies": dependencies.map((e) => e.toTomlValue()).toList(),
//       "tasks": tasks.toTomlValue(),
//     };
//   }
// }
