import 'package:coolbox/builder/annotation.dart';
import 'package:coolbox/model/package_manager.dart';

import 'task.dart';

part 'install.data.dart';

@data
abstract class IInstall extends Task {
  @Toml(
    from: "PackageManager.fromTomlValue({})",
    to: "packageManager.name",
  )
  PackageManager packageManager;

  IInstall({required this.packageManager}) : super("install");

  @override
  void execute() {
    // TODO: implement execute
  }
}
