import 'dart:io';

import 'package:archive/archive.dart';

import 'task.dart';

final class Decompress extends Task {
  String source;
  String destination;

  Decompress({
    required this.source,
    required this.destination,
  }) : super("decompress");

  @override
  void execute() {
    final source = File(this.source);
    final destination = Directory(this.destination);
    final bytes = source.readAsBytesSync();
    final archive = ZipDecoder().decodeBytes(bytes);

    for (final file in archive) {
      final filename = file.name;
      if (file.isFile) {
        final data = file.content as List<int>;
        File(join(outputPath, filename))
          ..createSync(recursive: true)
          ..writeAsBytesSync(data);
      } else {
        Directory(join(outputPath, filename))..createSync(recursive: true);
      }
    }
  }

  factory Decompress.fromToml(Map<String, dynamic> documents) {
    return Decompress(
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
      other is Decompress &&
          runtimeType == other.runtimeType &&
          source == other.source &&
          destination == other.destination;

  @override
  int get hashCode => source.hashCode ^ destination.hashCode;

  @override
  String toString() {
    return 'Decompress{source: $source, destination: $destination}';
  }
}
