import 'dart:io';

import 'package:archive/archive.dart';
import 'package:coolbox/builder/annotation.dart';

import 'task.dart';

part 'decompress.data.dart';

@data
abstract class IDecompress extends Task {
  String source;
  String destination;

  IDecompress({
    required this.source,
    required this.destination,
  }) : super("decompress");

  @override
  void execute() {
    // final source = File(this.source);
    // final destination = Directory(this.destination);
    // final bytes = source.readAsBytesSync();
    // final archive = ZipDecoder().decodeBytes(bytes);
    //
    // for (final file in archive) {
    //   final filename = file.name;
    //   if (file.isFile) {
    //     final data = file.content as List<int>;
    //     File(join(outputPath, filename))
    //       ..createSync(recursive: true)
    //       ..writeAsBytesSync(data);
    //   } else {
    //     Directory(join(outputPath, filename))..createSync(recursive: true);
    //   }
    // }
  }
}
