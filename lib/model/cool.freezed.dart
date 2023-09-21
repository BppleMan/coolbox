// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'cool.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$Cool {
  CoolID get id => throw _privateConstructorUsedError;
  String get description => throw _privateConstructorUsedError;
  List<Cool> get dependencies => throw _privateConstructorUsedError;
  String? get packageManager => throw _privateConstructorUsedError;

  @JsonKey(ignore: true)
  $CoolCopyWith<Cool> get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CoolCopyWith<$Res> {
  factory $CoolCopyWith(Cool value, $Res Function(Cool) then) =
      _$CoolCopyWithImpl<$Res, Cool>;
  @useResult
  $Res call(
      {CoolID id,
      String description,
      List<Cool> dependencies,
      String? packageManager});

  $CoolIDCopyWith<$Res> get id;
}

/// @nodoc
class _$CoolCopyWithImpl<$Res, $Val extends Cool>
    implements $CoolCopyWith<$Res> {
  _$CoolCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? description = null,
    Object? dependencies = null,
    Object? packageManager = freezed,
  }) {
    return _then(_value.copyWith(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as CoolID,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      dependencies: null == dependencies
          ? _value.dependencies
          : dependencies // ignore: cast_nullable_to_non_nullable
              as List<Cool>,
      packageManager: freezed == packageManager
          ? _value.packageManager
          : packageManager // ignore: cast_nullable_to_non_nullable
              as String?,
    ) as $Val);
  }

  @override
  @pragma('vm:prefer-inline')
  $CoolIDCopyWith<$Res> get id {
    return $CoolIDCopyWith<$Res>(_value.id, (value) {
      return _then(_value.copyWith(id: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$_CoolCopyWith<$Res> implements $CoolCopyWith<$Res> {
  factory _$$_CoolCopyWith(_$_Cool value, $Res Function(_$_Cool) then) =
      __$$_CoolCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call(
      {CoolID id,
      String description,
      List<Cool> dependencies,
      String? packageManager});

  @override
  $CoolIDCopyWith<$Res> get id;
}

/// @nodoc
class __$$_CoolCopyWithImpl<$Res> extends _$CoolCopyWithImpl<$Res, _$_Cool>
    implements _$$_CoolCopyWith<$Res> {
  __$$_CoolCopyWithImpl(_$_Cool _value, $Res Function(_$_Cool) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? description = null,
    Object? dependencies = null,
    Object? packageManager = freezed,
  }) {
    return _then(_$_Cool(
      id: null == id
          ? _value.id
          : id // ignore: cast_nullable_to_non_nullable
              as CoolID,
      description: null == description
          ? _value.description
          : description // ignore: cast_nullable_to_non_nullable
              as String,
      dependencies: null == dependencies
          ? _value._dependencies
          : dependencies // ignore: cast_nullable_to_non_nullable
              as List<Cool>,
      packageManager: freezed == packageManager
          ? _value.packageManager
          : packageManager // ignore: cast_nullable_to_non_nullable
              as String?,
    ));
  }
}

/// @nodoc

class _$_Cool with DiagnosticableTreeMixin implements _Cool {
  const _$_Cool(
      {required this.id,
      required this.description,
      required final List<Cool> dependencies,
      this.packageManager})
      : _dependencies = dependencies;

  @override
  final CoolID id;
  @override
  final String description;
  final List<Cool> _dependencies;
  @override
  List<Cool> get dependencies {
    if (_dependencies is EqualUnmodifiableListView) return _dependencies;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_dependencies);
  }

  @override
  final String? packageManager;

  @override
  String toString({DiagnosticLevel minLevel = DiagnosticLevel.info}) {
    return 'Cool(id: $id, description: $description, dependencies: $dependencies, packageManager: $packageManager)';
  }

  @override
  void debugFillProperties(DiagnosticPropertiesBuilder properties) {
    super.debugFillProperties(properties);
    properties
      ..add(DiagnosticsProperty('type', 'Cool'))
      ..add(DiagnosticsProperty('id', id))
      ..add(DiagnosticsProperty('description', description))
      ..add(DiagnosticsProperty('dependencies', dependencies))
      ..add(DiagnosticsProperty('packageManager', packageManager));
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$_Cool &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.description, description) ||
                other.description == description) &&
            const DeepCollectionEquality()
                .equals(other._dependencies, _dependencies) &&
            (identical(other.packageManager, packageManager) ||
                other.packageManager == packageManager));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, description,
      const DeepCollectionEquality().hash(_dependencies), packageManager);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$_CoolCopyWith<_$_Cool> get copyWith =>
      __$$_CoolCopyWithImpl<_$_Cool>(this, _$identity);
}

abstract class _Cool implements Cool {
  const factory _Cool(
      {required final CoolID id,
      required final String description,
      required final List<Cool> dependencies,
      final String? packageManager}) = _$_Cool;

  @override
  CoolID get id;
  @override
  String get description;
  @override
  List<Cool> get dependencies;
  @override
  String? get packageManager;
  @override
  @JsonKey(ignore: true)
  _$$_CoolCopyWith<_$_Cool> get copyWith => throw _privateConstructorUsedError;
}
