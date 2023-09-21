import 'package:coolbox/model/cool.dart';
import 'package:coolbox/model/cool_id.dart';
import 'package:coolbox/model/package_manager.dart';
import 'package:coolbox/model/tasks.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:toml/toml.dart';

void main() async {
  testWidgets("Test http", (WidgetTester tester) async {
    List<Task> tasks = <Task>[
      Download(
        url:
            "https://github.com/protocolbuffers/protobuf/releases/download/v24.3/protoc-24.3-osx-x86_64.zip",
        destination: "/Users/bppleman/Downloads/protoc-24.3-osx-x86_64.zip",
      ),
      Decompress(
        source: "/Users/bppleman/Downloads/protoc-24.3-osx-x86_64.zip",
        destination: "/Users/bppleman/Downloads/protoc-24.3-osx-x86_64",
      ),
      Install(),
    ];
    Cool brew = Cool(
      id: const CoolID(name: "brew", version: "4.1.12", description: "包管理器"),
      dependencies: [],
      tasks: Tasks(tasks: tasks),
    );
    Cool wget = Cool(
      id: const CoolID(name: "wget", version: "1.0.0", description: "下载器"),
      dependencies: [brew.id],
      tasks: Tasks(tasks: tasks),
      packageManager: Brew(),
    );
    var toml = TomlDocument.fromMap(wget.toTomlValue()).toString();
    print(toml);

    var tomlMap = TomlDocument.parse(toml).toMap();
    var wget2 = Cool.fromTomlValue(tomlMap);

    expect(wget2, wget);
  });
}
