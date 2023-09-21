import 'package:analyzer/dart/element/element.dart';
import 'package:build/src/builder/build_step.dart';
import 'package:build/build.dart';
import 'package:coolbox/builder/toml.dart';
import 'package:source_gen/source_gen.dart';

Builder tomlBuilder(BuilderOptions options) =>
    SharedPartBuilder([TomlGenerator()], "toml_generator");

class TomlGenerator extends GeneratorForAnnotation<Toml> {
  @override
  generateForAnnotatedElement(
    Element element,
    ConstantReader annotation,
    BuildStep buildStep,
  ) {
    print(element);
  }
}
