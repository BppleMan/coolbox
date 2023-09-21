abstract class Task {
  String name;

  Task(this.name);

  void execute();

  Map<String, dynamic> toTomlValue();
}
