import 'dart:io';

import 'task.dart';

final class Copy extends Task {
  String source;
  String destination;

  Copy({
    required this.source,
    required this.destination,
  }) : super("copy");

  @override
  void execute() {
    FileSystemEntity.isFileSync(path)
    var source = File(this.source);
    var destination = File(this.destination);
    if (!source.existsSync()) {
      throw Exception("Source file does not exist: ${this.source}");
    }
    if (destination.existsSync()) {
      destination.createSync(recursive: true);
    }
    if (destination.lengthSync() > 0) {
      throw Exception("Destination file is not empty: ${this.destination}");
    }
    source.copySync(destination.path);
  }

  factory Copy.fromToml(Map<String, dynamic> documents) {
    return Copy(
      source: documents["source"],
      destination: documents["destination"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "source": source,
      "destination": destination,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Copy &&
          runtimeType == other.runtimeType &&
          source == other.source &&
          destination == other.destination;

  @override
  int get hashCode => source.hashCode ^ destination.hashCode;

  @override
  String toString() {
    return 'Copy{source: $source, destination: $destination}';
  }
}
