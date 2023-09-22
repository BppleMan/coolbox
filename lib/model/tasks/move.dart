import 'package:coolbox/builder/annotation.dart';

import 'task.dart';

part 'move.data.dart';

@data
abstract class IMove extends Task {
  String source;
  String destination;

  IMove({
    required this.source,
    required this.destination,
  }) : super("move");

  @override
  void execute() {
    // TODO: implement execute
  }
}
