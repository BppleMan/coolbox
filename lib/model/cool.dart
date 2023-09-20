import 'command.dart';

class Cool {
  String name;
  String version;
  String description;

  Command install;
  Command uninstall;
  Command update;

  Cool({
    required this.name,
    required this.version,
    required this.description,
    required this.install,
    required this.uninstall,
    required this.update,
  });
}
