import 'package:coolbox/builder/annotation.dart';

import 'task.dart';

part 'download.data.dart';

@data
abstract class IDownload extends Task {
  String url;
  String destination;

  IDownload({
    required this.url,
    required this.destination,
  }) : super("download");

  @override
  void execute() {
    // TODO: implement execute
  }
}
