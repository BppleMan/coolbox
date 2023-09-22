import 'package:coolbox/builder/annotation.dart';
import 'package:toml/toml.dart';

part 'cool_id.data.dart';

@data
abstract class ICoolID implements TomlEncodableValue {
  String name;
  String version;
  String description;

  ICoolID({
    required this.name,
    required this.version,
    required this.description,
  });
}
