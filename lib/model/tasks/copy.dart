import 'dart:io';

import 'package:coolbox/builder/annotation.dart';

import 'task.dart';

part 'copy.data.dart';

@data
abstract class ICopy extends Task {
  String source;
  String destination;

  ICopy({
    required this.source,
    required this.destination,
  }) : super("copy");

  @override
  void execute() {
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

  // factory ICopy.fromTomlValue(Map<String, dynamic> documents) {
  //   return Copy(
  //     source: documents["source"],
  //     destination: documents["destination"],
  //   );
  // }
  //
  // @override
  // Map<String, dynamic> toTomlValue() {
  //   return {
  //     "name": name,
  //     "source": source,
  //     "destination": destination,
  //   };
  // }
}
