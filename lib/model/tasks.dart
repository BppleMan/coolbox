import 'package:coolbox/builder/annotation.dart';
import 'package:coolbox/model/tasks/install.dart';
import 'package:flutter/foundation.dart';
import 'package:toml/toml.dart';

import 'tasks/copy.dart';
import 'tasks/decompress.dart';
import 'tasks/delete.dart';
import 'tasks/download.dart';
import 'tasks/move.dart';
import 'tasks/run.dart';
import 'tasks/task.dart';

export 'tasks/copy.dart';
export 'tasks/decompress.dart';
export 'tasks/delete.dart';
export 'tasks/download.dart';
export 'tasks/install.dart';
export 'tasks/move.dart';
export 'tasks/run.dart';
export 'tasks/task.dart';

part 'tasks.data.dart';

typedef TaskCreator = Task Function(Map<String, dynamic>);

@data
abstract class ITasks implements TomlEncodableValue {
  @Toml(
    from:
        "[...{}.map((task) => ITasks.taskFactory()[task[\"name\"]]!(task)).toList()]",
    to: "taskList.map((e) => e.toTomlValue()).toList()",
  )
  List<Task> taskList;

  ITasks({required this.taskList});

  static Map<String, TaskCreator> taskFactory() => {
        "copy": Copy.fromTomlValue,
        "move": Move.fromTomlValue,
        "delete": Delete.fromTomlValue,
        "download": Download.fromTomlValue,
        "run": Run.fromTomlValue,
        "decompress": Decompress.fromTomlValue,
        "install": Install.fromTomlValue,
      };

  void execute() {
    for (var task in taskList) {
      task.execute();
    }
  }

  void addTask(Task task) {
    taskList.add(task);
  }
}
