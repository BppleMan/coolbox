import 'task.dart';

final class Run extends Task {
  String command;
  List<String> args;

  Run({
    required this.command,
    required this.args,
  }) : super("run");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Run.fromToml(Map<String, dynamic> documents) {
    return Run(
      command: documents["command"],
      args: documents["args"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "command": command,
      "args": args,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Run &&
          runtimeType == other.runtimeType &&
          command == other.command &&
          args == other.args;

  @override
  int get hashCode => command.hashCode ^ args.hashCode;

  @override
  String toString() {
    return 'Run{command: $command, args: $args}';
  }
}
