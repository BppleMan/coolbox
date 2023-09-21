import 'task.dart';

final class Download extends Task {
  String url;
  String destination;

  Download({
    required this.url,
    required this.destination,
  }) : super("download");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Download.fromToml(Map<String, dynamic> documents) {
    return Download(
      url: documents["url"],
      destination: documents["destination"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "url": url,
      "destination": destination,
    };
  }

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Download &&
          runtimeType == other.runtimeType &&
          url == other.url &&
          destination == other.destination;

  @override
  int get hashCode => url.hashCode ^ destination.hashCode;

  @override
  String toString() {
    return 'Download{url: $url, destination: $destination}';
  }
}
