import 'package:coolbox/builder/annotation.dart';
import 'package:flutter/foundation.dart';

import 'task.dart';

part 'run.data.dart';

@data
abstract class IRun extends Task {
  String command;
  List<String> args;

  IRun({
    required this.command,
    required this.args,
  }) : super("run");

  @override
  void execute() {
    // TODO: implement execute
  }
}
