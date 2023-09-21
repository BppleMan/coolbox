typedef TaskCreator = Task Function(Map<String, dynamic>);

class Tasks {
  List<Task> tasks;

  Tasks(this.tasks);

  static final Map<String, TaskCreator> taskFactory = {
    "copy": Copy.fromToml,
    "move": Move.fromToml,
    "delete": Delete.fromToml,
    "download": Download.fromToml,
    "run": Run.fromToml,
    "decompress": Decompress.fromToml,
  };

  factory Tasks.fromToml(List<Map<String, dynamic>> documents) {
    final tasks = documents
        .map((task) => Tasks.taskFactory[task["name"]]!(task))
        .toList();
    return Tasks(tasks);
  }

  void execute() {
    for (var task in tasks) {
      task.execute();
    }
  }

  List<Map<String, dynamic>> toTomlValue() {
    return tasks.map((e) => e.toTomlValue()).toList();
  }
}

sealed class Task {
  String name;

  Task(this.name);

  void execute();

  Map<String, dynamic> toTomlValue();
}

final class Copy extends Task {
  String source;
  String destination;

  Copy({
    required this.source,
    required this.destination,
  }) : super("copy");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Copy.fromToml(Map<String, dynamic> documents) {
    return Copy(
      source: documents["source"],
      destination: documents["destination"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "source": source,
      "destination": destination,
    };
  }
}

final class Move extends Task {
  String source;
  String destination;

  Move({
    required this.source,
    required this.destination,
  }) : super("move");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Move.fromToml(Map<String, dynamic> documents) {
    return Move(
      source: documents["source"],
      destination: documents["destination"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "source": source,
      "destination": destination,
    };
  }
}

final class Delete extends Task {
  String path;

  Delete({
    required this.path,
  }) : super("delete");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Delete.fromToml(Map<String, dynamic> documents) {
    return Delete(
      path: documents["path"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "path": path,
    };
  }
}

final class Download extends Task {
  String url;
  String destination;

  Download({
    required this.url,
    required this.destination,
  }) : super("download");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Download.fromToml(Map<String, dynamic> documents) {
    return Download(
      url: documents["url"],
      destination: documents["destination"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "url": url,
      "destination": destination,
    };
  }
}

final class Run extends Task {
  String command;
  List<String> args;

  Run({
    required this.command,
    required this.args,
  }) : super("run");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Run.fromToml(Map<String, dynamic> documents) {
    return Run(
      command: documents["command"],
      args: documents["args"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "command": command,
      "args": args,
    };
  }
}

final class Decompress extends Task {
  String source;
  String destination;

  Decompress({
    required this.source,
    required this.destination,
  }) : super("decompress");

  @override
  void execute() {
    // TODO: implement execute
  }

  factory Decompress.fromToml(Map<String, dynamic> documents) {
    return Decompress(
      source: documents["source"],
      destination: documents["destination"],
    );
  }

  @override
  Map<String, dynamic> toTomlValue() {
    return {
      "name": name,
      "source": source,
      "destination": destination,
    };
  }
}
