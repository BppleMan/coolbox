import 'task.dart';

final class Move extends Task {
  String source;
  String destination;

  Move({
    required this.source,
    required this.destination,
  }) : super("move");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Move.fromToml(Map<String, dynamic> documents) {
    return Move(
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
      other is Move &&
          runtimeType == other.runtimeType &&
          source == other.source &&
          destination == other.destination;

  @override
  int get hashCode => source.hashCode ^ destination.hashCode;

  @override
  String toString() {
    return 'Move{source: $source, destination: $destination}';
  }
}
