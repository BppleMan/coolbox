import 'package:coolbox/builder/annotation.dart';

import 'task.dart';

part 'delete.data.dart';

@data
abstract class IDelete extends Task {
  String path;

  IDelete({
    required this.path,
  }) : super("delete");

  @override
  void execute() {
    // TODO: implement execute
  }
}
