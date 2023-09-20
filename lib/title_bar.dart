import 'package:flutter/material.dart';

class TitleBar extends StatelessWidget {
  const TitleBar({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 58,
      child: Row(
        children: [
          const Text(
            "CoolBox",
            style: TextStyle(
              fontSize: 36,
              fontWeight: FontWeight.bold,
              height: 48 / 36,
            ),
          ),
          const Expanded(child: SizedBox()),
          SizedBox(
            width: 200,
            child: Stack(
              children: [
                SizedBox(
                  width: double.infinity,
                  height: double.infinity,
                  child: LinearProgressIndicator(
                    value: 0.5,
                    borderRadius: BorderRadius.circular(12),
                    backgroundColor: const Color(0xff333333),
                  ),
                ),
                SizedBox(
                  width: double.infinity,
                  height: double.infinity,
                  child: TextButton(
                    onPressed: () {},
                    style: ButtonStyle(
                      shape: MaterialStateProperty.all(
                        RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(12),
                        ),
                      ),
                      backgroundColor:
                          MaterialStateProperty.all(Colors.transparent),
                    ),
                    child: const Text(
                      "Install",
                      style: TextStyle(
                        color: Colors.deepPurple,
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                        height: 32 / 20,
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
