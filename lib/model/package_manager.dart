import 'dart:io';

import 'package:coolbox/model/command.dart';
import 'package:coolbox/model/cool_id.dart';

sealed class PackageManager {
  String name;
  String commandPrefix;
  bool needsSudo;
  abstract Shell installCommand;

  PackageManager({
    required this.name,
    required this.commandPrefix,
    required this.needsSudo,
  });

  void installSelf() {
    installCommand.execute();
  }

  void install(CoolID id) {
    Process.start(executable, arguments)
  }
}

class Brew extends PackageManager {
  @override
  late Shell installCommand;

  Brew() : super(name: "brew", commandPrefix: "brew install", needsSudo: false) {
    installCommand = Shell(interpreter: "brew", args: "install");
  }
}
