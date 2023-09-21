import 'task.dart';

final class Install extends Task {
  Install() : super("install");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Install.fromToml(Map<String, dynamic> documents) {
    return Install();
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Install &&
          runtimeType == other.runtimeType &&
          name == other.name;

  @override
  int get hashCode => name.hashCode;

  @override
  String toString() {
    return 'Install{}';
  }
}
