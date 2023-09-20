import 'package:coolbox/home.dart';
import 'package:flutter/material.dart';

class CoolBoxApp extends StatelessWidget {
  const CoolBoxApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'CoolBox',
      theme: ThemeData(
        brightness: Brightness.dark,
        useMaterial3: true,
      ),
      home: const CoolBoxHome(),
      debugShowCheckedModeBanner: false,
    );
  }
}
