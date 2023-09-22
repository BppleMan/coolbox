// ignore_for_file: prefer_interpolation_to_compose_strings

import 'package:coolbox/builder/annotation.dart';
import 'package:analyzer/dart/element/element.dart' show FieldElement;

class Clazz {
  String name;
  String part;
  String interface;
  List<FieldElement> superFields;
  List<FieldElement> fields;
  List<FieldElement> tomlFields;
  List<Toml> tomls;
  late List<FieldElement> totalFields;

  Clazz({
    required this.name,
    required this.part,
    required this.interface,
    required this.superFields,
    required this.fields,
    required this.tomlFields,
    required this.tomls,
  }) {
    totalFields = [...fields, ...tomlFields];
  }

  // String build() => """
  // part of "$part";
  //
  // class $name extends $interface {
  //   $name(${totalFields.isNotEmpty ? "{" + totalFields.map((e) => "required super.$e").join(", ") + "}" : ""});
  //
  //   factory $name.fromTomlValue(Map<String, dynamic> document) {
  //     return $name(
  //       ${fields.map((e) => "$e: document['$e']").join(",\n")}${fields.isNotEmpty ? "," : ""}
  //       ${tomlFields.indexed.map((i) => "${i.$2}: ${tomls[i.$1].from.replaceAll("{}", "document['${i.$2}']")}").join(",\n")}${tomlFields.isNotEmpty ? "," : ""}
  //     );
  //   }
  //
  //   @override
  //   Map<String, dynamic> toTomlValue() {
  //     return {
  //       ${superFields.map((e) => "'$e': $e").join(",\n")}${superFields.isNotEmpty ? "," : ""}
  //       ${fields.map((e) => "'$e': $e").join(",\n")}${fields.isNotEmpty ? "," : ""}
  //       ${tomlFields.indexed.map((i) => "'${i.$2}': ${tomls[i.$1].to}").join(",\n")}${tomlFields.isNotEmpty ? "," : ""}
  //     };
  //   }
  //
  //   @override
  //   bool operator ==(Object other) =>
  //     identical(this, other) ||
  //     other is $name &&
  //       runtimeType == other.runtimeType${totalFields.isNotEmpty ? "&&" : ""}
  //       ${totalFields.map((e) => "$e == other.$e").join(" && ")};
  //
  //   @override
  //   int get hashCode => ${totalFields.isNotEmpty ? totalFields.map((e) => "$e.hashCode").join(" ^ ") : "1"};
  //
  //   @override
  //   String toString() {
  //     return '$name{${totalFields.map((e) => "$e: \$$e").join(", ")}}';
  //   }
  // }
  // """;

  String build() => """
    part of "$part";
    
    class $name extends $interface {
      ${constructor()}
      
      ${fromTomlValue()}
      
      ${toTomlValue()}
      
      ${operator()}
      
      ${hash()}
      
      ${string()}
    }
  """;

  String constructor() => """
    $name(${totalFields.isNotEmpty ? "{" + totalFields.map((e) => "required super.${e.name}").join(", ") + "}" : ""}); 
  """;

  String fromTomlValue() => """
    factory $name.fromTomlValue(Map<String, dynamic> document) {
      return $name(
        ${fields.map((e) => "${e.name}: document['${e.name}']").join(",\n")}${fields.isNotEmpty ? "," : ""}
        ${tomlFields.indexed.map((i) => "${i.$2.name}: ${tomls[i.$1].from.replaceAll("{}", "document['${i.$2.name}']")}").join(",\n")}${tomlFields.isNotEmpty ? "," : ""}
      );
    }
  """;

  String toTomlValue() => """
    @override
    Map<String, dynamic> toTomlValue() {
      return {
        ${superFields.map((e) => "'${e.name}': ${e.name}").join(",\n")}${superFields.isNotEmpty ? "," : ""}
        ${fields.map((e) => "'${e.name}': ${e.name}").join(",\n")}${fields.isNotEmpty ? "," : ""}
        ${tomlFields.indexed.map((i) => "'${i.$2.name}': ${tomls[i.$1].to}").join(",\n")}${tomlFields.isNotEmpty ? "," : ""}
      };
    }
  """;

  String operator() => """
    @override
    bool operator ==(Object other) =>
      identical(this, other) ||
      other is $name &&
        runtimeType == other.runtimeType${totalFields.isNotEmpty ? "&&" : ""}
        ${totalFields.map((e) => equals(e)).join(" && ")};
  """;

  String hash() => """
    @override
    int get hashCode => ${totalFields.isNotEmpty ? totalFields.map((e) => "${e.name}.hashCode").join(" ^ ") : "1"};
  """;

  String string() => """
    @override
    String toString() {
      return '$name{${totalFields.map((e) => "${e.name}: \$${e.name}").join(", ")}}';
    }
  """;

  String equals(FieldElement e) {
    if (e.type.isDartCoreList) {
      return "listEquals(${e.name}, other.${e.name})";
    } else {
      return "${e.name} == other.${e.name}";
    }
  }
}
