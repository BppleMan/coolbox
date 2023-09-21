import 'dart:io';

sealed class Shell {
  String executable;
  String executableArg;
  String? args;

  Shell({
    required this.executable,
    required this.executableArg,
    this.args,
  });

  Future<ShellResult> execute(String executable, String args) async {
    List<String> arguments = [];
    if (executableArg.isNotEmpty) {
      arguments.add(executableArg);
      arguments.add((this.args ?? "{}").replaceAll("{}", "$executable $args"));
    } else {
      arguments.add(executable);
      arguments.add((this.args ?? "{}").replaceAll("{}", args));
    }
    var process = await Process.start(
      this.executable,
      arguments,
      includeParentEnvironment: true,
    );
    var results = await Future.wait([
      process.stdout.asyncMap((event) => String.fromCharCodes(event)).toList(),
      process.stderr.asyncMap((event) => String.fromCharCodes(event)).toList(),
      Future(() => [process.exitCode.toString()]),
    ]);
    var output = results[0];
    var errors = results[1];
    var exitCode = results[2];
    return ShellResult(
      output: output,
      errors: errors,
      exitCode: int.parse(exitCode[0]),
    );
  }
}

class ShellResult {
  List<String> output;
  List<String> errors;
  int exitCode;

  ShellResult({
    required this.output,
    required this.errors,
    required this.exitCode,
  });
}

final class Bash extends Shell {
  Bash() : super(executable: "bash", executableArg: "-c");
}

final class Sh extends Shell {
  Sh() : super(executable: "sh", executableArg: "-c");
}

final class MacSudo extends Shell {
  MacSudo()
      : super(
          executable: "osascript",
          executableArg: "-e",
          args: "do shell script \"{}\" with administrator privileges",
        );
}

final class LinuxSudo extends Shell {
  LinuxSudo()
      : super(
          executable: "pkexec",
          executableArg: "",
        );
}
