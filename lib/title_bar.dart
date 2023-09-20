import 'package:flutter/material.dart';

class TitleBar extends StatelessWidget {
  const TitleBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Text("CoolBox", style: Theme.of(context).textTheme.headline4),
        const Expanded(child: SizedBox()),
        ElevatedButton(onPressed: () => {}, child: const Text("Install"))
      ],
    );
  }
}
