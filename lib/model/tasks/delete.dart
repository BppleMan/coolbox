import 'task.dart';

final class Delete extends Task {
  String path;

  Delete({
    required this.path,
  }) : super("delete");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Delete.fromToml(Map<String, dynamic> documents) {
    return Delete(
      path: documents["path"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "path": path,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Delete && runtimeType == other.runtimeType && path == other.path;

  @override
  int get hashCode => path.hashCode;

  @override
  String toString() {
    return 'Delete{path: $path}';
  }
}
