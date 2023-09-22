import 'package:toml/toml.dart';

abstract class Task implements TomlEncodableValue {
  String name;

  Task(this.name);

  void execute();

  @override
  Map<String, dynamic> toTomlValue();

  // void fromTomlValue(Map<String, dynamic> documents);
}
