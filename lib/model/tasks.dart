import 'package:coolbox/model/tasks/install.dart';
import 'package:flutter/foundation.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

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

part 'tasks.freezed.dart';

typedef TaskCreator = Task Function(Map<String, dynamic>);

@freezed
class Tasks with _$Tasks {
  factory Tasks({required List<Task> tasks}) = _Tasks;

  static final Map<String, TaskCreator> taskFactory = {
    "copy": Copy.fromToml,
    "move": Move.fromToml,
    "delete": Delete.fromToml,
    "download": Download.fromToml,
    "run": Run.fromToml,
    "decompress": Decompress.fromToml,
    "install": Install.fromToml,
  };

  factory Tasks.fromTomlValue(List<Map<String, dynamic>> documents) {
    final tasks = documents
        .map((task) => Tasks.taskFactory[task["name"]]!(task))
        .toList();
    return Tasks(tasks: tasks);
  }
}

extension TasksExtension on Tasks {
  void execute() {
    for (var task in tasks) {
      task.execute();
    }
  }

  void addTask(Task task) {
    tasks.add(task);
  }

  List<Map<String, dynamic>> toTomlValue() {
    return tasks.map((e) => e.toTomlValue()).toList();
  }
}
