import 'package:coolbox/model/shell.dart';

final class LinuxSudo extends Shell {
  LinuxSudo()
      : super(
          executable: "pkexec",
          executableArg: "",
        );
}
