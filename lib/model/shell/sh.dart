import 'package:coolbox/model/shell.dart';

final class Sh extends Shell {
  Sh() : super(executable: "sh", executableArg: "-c");
}
