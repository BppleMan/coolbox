import 'dart:io';

export 'shell/sh.dart';
export 'shell/bash.dart';
export 'shell/linux_sudo.dart';
export 'shell/mac_sudo.dart';

class Shell {
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Shell &&
          runtimeType == other.runtimeType &&
          executable == other.executable &&
          executableArg == other.executableArg &&
          args == other.args;

  @override
  int get hashCode =>
      executable.hashCode ^ executableArg.hashCode ^ args.hashCode;

  @override
  String toString() {
    return 'Shell{executable: $executable, executableArg: $executableArg, args: $args}';
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
