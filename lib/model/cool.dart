import 'command.dart';
import 'cool_id.dart';

class Cool {
  CoolID id;
  String description;
  List<CoolID> dependencies;

  Shell install;
  Shell uninstall;
  Shell update;

  Cool({
    required this.id,
    required this.description,
    required this.dependencies,
    required this.install,
    required this.uninstall,
    required this.update,
  });
}
