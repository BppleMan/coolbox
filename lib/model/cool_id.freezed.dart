// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'cool_id.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$CoolID {
  String get name => throw _privateConstructorUsedError;
  String get version => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $CoolIDCopyWith<CoolID> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CoolIDCopyWith<$Res> {
  factory $CoolIDCopyWith(CoolID value, $Res Function(CoolID) then) =
      _$CoolIDCopyWithImpl<$Res, CoolID>;
  @useResult
  $Res call({String name, String version});
}

/// @nodoc
class _$CoolIDCopyWithImpl<$Res, $Val extends CoolID>
    implements $CoolIDCopyWith<$Res> {
  _$CoolIDCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? version = null,
  }) {
    return _then(_value.copyWith(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      version: null == version
          ? _value.version
          : version // ignore: cast_nullable_to_non_nullable
              as String,
    ) as $Val);
  }
}

/// @nodoc
abstract class _$$_CoolIDCopyWith<$Res> implements $CoolIDCopyWith<$Res> {
  factory _$$_CoolIDCopyWith(_$_CoolID value, $Res Function(_$_CoolID) then) =
      __$$_CoolIDCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String name, String version});
}

/// @nodoc
class __$$_CoolIDCopyWithImpl<$Res>
    extends _$CoolIDCopyWithImpl<$Res, _$_CoolID>
    implements _$$_CoolIDCopyWith<$Res> {
  __$$_CoolIDCopyWithImpl(_$_CoolID _value, $Res Function(_$_CoolID) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? name = null,
    Object? version = null,
  }) {
    return _then(_$_CoolID(
      name: null == name
          ? _value.name
          : name // ignore: cast_nullable_to_non_nullable
              as String,
      version: null == version
          ? _value.version
          : version // ignore: cast_nullable_to_non_nullable
              as String,
    ));
  }
}

/// @nodoc

class _$_CoolID with DiagnosticableTreeMixin implements _CoolID {
  const _$_CoolID({required this.name, required this.version});

  @override
  final String name;
  @override
  final String version;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'CoolID(name: $name, version: $version)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'CoolID'))
      ..add(DiagnosticsProperty('name', name))
      ..add(DiagnosticsProperty('version', version));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_CoolID &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.version, version) || other.version == version));
  }

  @override
  int get hashCode => Object.hash(runtimeType, name, version);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_CoolIDCopyWith<_$_CoolID> get copyWith =>
      __$$_CoolIDCopyWithImpl<_$_CoolID>(this, _$identity);
}

abstract class _CoolID implements CoolID {
  const factory _CoolID(
      {required final String name, required final String version}) = _$_CoolID;

  @override
  String get name;
  @override
  String get version;
  @override
  @JsonKey(ignore: true)
  _$$_CoolIDCopyWith<_$_CoolID> get copyWith =>
      throw _privateConstructorUsedError;
}
