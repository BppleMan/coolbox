import 'package:coolbox/model/shell.dart';

import 'cool.dart';

sealed class PackageManager {
  String name;
  String commandPrefix;
  abstract Shell shell;

  PackageManager({
    required this.name,
    required this.commandPrefix,
  });

  void installSelf();

  void install(Cool cool) async {
    var result = await shell.execute(name, "$commandPrefix ${cool.id.name}");
    print(result.output);
  }
}

class Brew extends PackageManager {
  Brew._() : super(name: "brew", commandPrefix: "install");

  static final Brew _instance = Brew._();

  factory Brew() {
    return _instance;
  }

  @override
  Shell shell = Sh();

  @override
  void installSelf() {
    // Bash().execute(executable, args)
  }
}

// enum PackageManagements {
//   Brew = "brew",
// }
