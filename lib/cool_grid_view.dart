import 'dart:math' as math;

import 'package:coolbox/cool_card.dart';
import 'package:coolbox/model/cool.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import 'cool_repository.dart';
import 'model/cool_id.dart';

class CoolGridView extends StatelessWidget {
  final CoolRepository coolRepository = CoolRepository();
  late final Stream<Cool> cools;

  CoolGridView({super.key});

  @override
  StatelessElement createElement() {
    cools = coolRepository.getCools();
    return super.createElement();
  }

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
        constraints: const BoxConstraints(
          minWidth: 320,
          minHeight: 188,
        ),
        child: GridView.custom(
          gridDelegate: const SliverGridDelegateWithMinCrossAxisExtent(
            minCrossAxisExtent: 320,
            mainAxisExtent: 184,
            mainAxisSpacing: 24,
            crossAxisSpacing: 24,
          ),
          childrenDelegate: SliverChildBuilderDelegate(
            childCount: 21,
            (context, index) {
              return;
            },
          ),
        ),
      ),
    );
  }
}

class SliverGridDelegateWithMinCrossAxisExtent extends SliverGridDelegate {
  /// Creates a delegate that makes grid layouts with tiles that have a maximum
  /// cross-axis extent.
  ///
  /// All of the arguments except [mainAxisExtent] must not be null.
  /// The [minCrossAxisExtent], [mainAxisExtent], [mainAxisSpacing],
  /// and [crossAxisSpacing] arguments must not be negative.
  /// The [childAspectRatio] argument must be greater than zero.
  const SliverGridDelegateWithMinCrossAxisExtent({
    required this.minCrossAxisExtent,
    this.mainAxisSpacing = 0.0,
    this.crossAxisSpacing = 0.0,
    this.childAspectRatio = 1.0,
    this.mainAxisExtent,
  })  : assert(minCrossAxisExtent > 0),
        assert(mainAxisSpacing >= 0),
        assert(crossAxisSpacing >= 0),
        assert(childAspectRatio > 0);

  /// The maximum extent of tiles in the cross axis.
  ///
  /// This delegate will select a cross-axis extent for the tiles that is as
  /// large as possible subject to the following conditions:
  ///
  ///  - The extent evenly divides the cross-axis extent of the grid.
  ///  - The extent is at most [minCrossAxisExtent].
  ///
  /// For example, if the grid is vertical, the grid is 500.0 pixels wide, and
  /// [minCrossAxisExtent] is 150.0, this delegate will create a grid with 4
  /// columns that are 125.0 pixels wide.
  final double minCrossAxisExtent;

  /// The number of logical pixels between each child along the main axis.
  final double mainAxisSpacing;

  /// The number of logical pixels between each child along the cross axis.
  final double crossAxisSpacing;

  /// The ratio of the cross-axis to the main-axis extent of each child.
  final double childAspectRatio;

  /// The extent of each tile in the main axis. If provided it would define the
  /// logical pixels taken by each tile in the main-axis.
  ///
  /// If null, [childAspectRatio] is used instead.
  final double? mainAxisExtent;

  bool _debugAssertIsValid(double crossAxisExtent) {
    assert(crossAxisExtent > 0.0);
    assert(minCrossAxisExtent > 0.0);
    assert(mainAxisSpacing >= 0.0);
    assert(crossAxisSpacing >= 0.0);
    assert(childAspectRatio > 0.0);
    return true;
  }

  @override
  SliverGridLayout getLayout(SliverConstraints constraints) {
    assert(_debugAssertIsValid(constraints.crossAxisExtent));
    int crossAxisCount =
        (constraints.crossAxisExtent / (minCrossAxisExtent + crossAxisSpacing))
            .floor();
    // Ensure a minimum count of 1, can be zero and result in an infinite extent
    // below when the window size is 0.
    crossAxisCount = math.max(1, crossAxisCount);
    final double usableCrossAxisExtent = math.max(
      0.0,
      constraints.crossAxisExtent - crossAxisSpacing * (crossAxisCount - 1),
    );
    final double childCrossAxisExtent = usableCrossAxisExtent / crossAxisCount;
    final double childMainAxisExtent =
        mainAxisExtent ?? childCrossAxisExtent / childAspectRatio;
    return SliverGridRegularTileLayout(
      crossAxisCount: crossAxisCount,
      mainAxisStride: childMainAxisExtent + mainAxisSpacing,
      crossAxisStride: childCrossAxisExtent + crossAxisSpacing,
      childMainAxisExtent: childMainAxisExtent,
      childCrossAxisExtent: childCrossAxisExtent,
      reverseCrossAxis: axisDirectionIsReversed(constraints.crossAxisDirection),
    );
  }

  @override
  bool shouldRelayout(SliverGridDelegateWithMaxCrossAxisExtent oldDelegate) {
    return oldDelegate.maxCrossAxisExtent != minCrossAxisExtent ||
        oldDelegate.mainAxisSpacing != mainAxisSpacing ||
        oldDelegate.crossAxisSpacing != crossAxisSpacing ||
        oldDelegate.childAspectRatio != childAspectRatio ||
        oldDelegate.mainAxisExtent != mainAxisExtent;
  }
}

class SliverGridDelegateWithMaxCrossAxisSpacing extends SliverGridDelegate {
  final double minCrossAxisSpacing;
  final double mainAxisSpacing;
  final double mainAxisExtend;
  final double crossAxisExtend;

  SliverGridDelegateWithMaxCrossAxisSpacing({
    required this.minCrossAxisSpacing,
    required this.mainAxisSpacing,
    required this.mainAxisExtend,
    required this.crossAxisExtend,
  });

  @override
  SliverGridLayout getLayout(SliverConstraints constraints) {
    int crossAxisCount = (constraints.crossAxisExtent ~/
        (crossAxisExtend + minCrossAxisSpacing));
    print(crossAxisCount);
    double crossAxisSpacing =
        (constraints.crossAxisExtent - (crossAxisCount * crossAxisExtend)) /
            crossAxisCount;
    print(crossAxisSpacing);
    return SliverGridRegularTileLayout(
        crossAxisCount: crossAxisCount,
        mainAxisStride: mainAxisExtend + mainAxisSpacing,
        crossAxisStride: crossAxisExtend + crossAxisSpacing,
        childMainAxisExtent: mainAxisExtend,
        childCrossAxisExtent: crossAxisExtend,
        reverseCrossAxis: false);
  }

  @override
  bool shouldRelayout(
      covariant SliverGridDelegateWithMaxCrossAxisSpacing oldDelegate) {
    Object maxCrossAxisExtent;
    return oldDelegate.minCrossAxisSpacing != minCrossAxisSpacing ||
        oldDelegate.mainAxisSpacing != mainAxisSpacing ||
        oldDelegate.mainAxisExtend != mainAxisExtend ||
        oldDelegate.crossAxisExtend != crossAxisExtend;
  }
}
