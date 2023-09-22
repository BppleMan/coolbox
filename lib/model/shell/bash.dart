import 'package:coolbox/model/shell.dart';

final class Bash extends Shell {
  Bash() : super(executable: "bash", executableArg: "-c");
}
