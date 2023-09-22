import 'package:coolbox/cool_grid_view.dart';
import 'package:coolbox/title_bar.dart';
import 'package:flutter/material.dart';

class CoolBoxHome extends StatelessWidget {
  const CoolBoxHome({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        padding: const EdgeInsets.all(40),
        child: Column(
          children: [
            const TitleBar(),
            const SizedBox(height: 40),
            CoolGridView(),
          ],
        ),
      ),
    );
  }
}
