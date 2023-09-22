import 'package:analyzer/dart/element/element.dart'
    show ClassElement, Element, ElementKind, FieldElement;
import 'package:build/build.dart' show BuildStep, Builder, BuilderOptions;
import 'package:coolbox/builder/clazz.dart';
import 'package:source_gen/source_gen.dart';

import 'annotation.dart';

Builder dataClassBuilder(BuilderOptions options) =>
    LibraryBuilder(DataClassGenerator(), generatedExtension: ".data.dart");

class DataClassGenerator extends GeneratorForAnnotation<Data> {
  @override
  generateForAnnotatedElement(
    Element element,
    ConstantReader annotation,
    BuildStep buildStep,
  ) {
    var interface = element.name!;
    var part = element.source!.shortName;
    var name = interface.replaceFirst("I", "");
    List<FieldElement> superFields = [];
    List<FieldElement> fields = [];
    List<FieldElement> tomlFields = [];
    List<Toml> tomls = [];
    var classElement = element as ClassElement;
    classElement.supertype?.element.fields.forEach((element) {
      if (element.name == "name") {
        superFields.add(element);
      }
    });
    classElement.fields.where((element) {
      return element.kind == ElementKind.FIELD;
    }).forEach((element) {
      if (element.metadata.any((element) =>
          element.computeConstantValue()?.type?.element?.name == "Toml")) {
        tomlFields.add(element);
        var toml = element.metadata.firstWhere(
          (e) => e.computeConstantValue()?.type?.element?.name == "Toml",
        );
        tomls.add(Toml(
          from: toml.computeConstantValue()!.getField("from")!.toStringValue()!,
          to: toml.computeConstantValue()!.getField("to")!.toStringValue()!,
        ));
      } else {
        fields.add(element);
      }
    });
    return Clazz(
      name: name,
      part: part,
      interface: interface,
      superFields: superFields,
      fields: fields,
      tomlFields: tomlFields,
      tomls: tomls,
    ).build();
  }
}
