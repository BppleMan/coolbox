import 'package:coolbox/model/shell.dart';

class MacSudo extends Shell {
  MacSudo()
      : super(
          executable: "osascript",
          executableArg: "-e",
          args: "do shell script \"{}\" with administrator privileges",
        );
}
