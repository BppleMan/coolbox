import 'package:coolbox/model/cool.dart';
import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:native_dio_adapter/native_dio_adapter.dart';

typedef SelectedChanged = void Function(bool selected);

class CoolCard extends StatelessWidget {
  final Cool cool;

  SelectedChanged? onSelectedChanged;

  CoolCard({
    super.key,
    required this.cool,
    this.onSelectedChanged,
  });

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => CoolCardBloc(),
      child: BlocBuilder<CoolCardBloc, bool>(
        builder: (context, state) => InkWell(
          onTap: () {
            context.read<CoolCardBloc>().add(CoolCardSelectedEvent(!state));
          },
          splashColor: Colors.transparent,
          highlightColor: Colors.transparent,
          child: ClipRRect(
            borderRadius: const BorderRadius.all(Radius.circular(12)),
            child: Container(
              decoration: BoxDecoration(
                border: Border.all(color: const Color(0xff434343)),
                borderRadius: const BorderRadius.all(Radius.circular(12)),
              ),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Expanded(
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                        vertical: 16,
                        horizontal: 24,
                      ),
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisAlignment: MainAxisAlignment.start,
                        children: [
                          titleRow(),
                          const SizedBox(height: 12),
                          Expanded(child: descriptionRow()),
                          const SizedBox(height: 16),
                          buttonRow(),
                        ],
                      ),
                    ),
                  ),
                  const LinearProgressIndicator(
                    value: 0.5,
                    valueColor:
                        AlwaysStoppedAnimation<Color>(Colors.deepPurple),
                    backgroundColor: Colors.grey,
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget titleRow() {
    return SizedBox(
      height: 32,
      child: Row(
        children: [
          BlocBuilder<CoolCardBloc, bool>(builder: (context, state) {
            return SizedBox(
              width: 24,
              height: 24,
              child: Checkbox(
                value: state,
                onChanged: (value) {
                  if (value != null) {
                    context
                        .read<CoolCardBloc>()
                        .add(CoolCardSelectedEvent(value));
                  }
                },
                hoverColor: null,
                splashRadius: 0.0,
                materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
              ),
            );
          }),
          const SizedBox(width: 10),
          Text(
            cool.id.name,
            style: const TextStyle(
              fontSize: 20,
              height: 32 / 20,
              fontWeight: FontWeight.bold,
            ),
          ),
          const SizedBox(width: 16),
          Text(
            cool.id.version,
            style: TextStyle(
              color: const Color(0xffffffff).withAlpha(40),
              fontSize: 16,
              height: 32 / 16,
            ),
          ),
        ],
      ),
    );
  }

  Widget descriptionRow() {
    return Tooltip(
      message: cool.id.description,
      verticalOffset: 20,
      waitDuration: const Duration(milliseconds: 500),
      child: Text(
        cool.id.description,
        style: const TextStyle(
          fontSize: 16,
          height: 24 / 16,
          fontWeight: FontWeight.normal,
        ),
        maxLines: 2,
        softWrap: true,
      ),
    );
  }

  Widget buttonRow() {
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        operateButton(name: "Update", onPressed: () {}),
        const SizedBox(width: 12),
        operateButton(
            name: "Install",
            onPressed: () async {
              var dio = Dio();
              dio.httpClientAdapter = NativeAdapter();
              var response = await dio.get<String>(
                "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
              );
              print(response.data);
            }),
      ],
    );
  }

  Widget operateButton({
    required String name,
    VoidCallback? onPressed,
  }) {
    return SizedBox(
      height: 36,
      child: ElevatedButton(
        onPressed: onPressed,
        style: ButtonStyle(
          shape: MaterialStateProperty.all<RoundedRectangleBorder>(
            RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12.0),
              side: const BorderSide(color: Color(0xff434343)),
            ),
          ),
        ),
        child: Text(
          name,
          style: const TextStyle(
            fontSize: 14,
            height: 22 / 14,
            fontWeight: FontWeight.w400,
          ),
        ),
      ),
    );
  }
}

sealed class CoolCardEvent {}

class CoolCardSelectedEvent extends CoolCardEvent {
  final bool selected;

  CoolCardSelectedEvent(this.selected);
}

class CoolCardBloc extends Bloc<CoolCardEvent, bool> {
  CoolCardBloc() : super(false) {
    on<CoolCardSelectedEvent>((event, emit) {
      print("CoolCardBloc: $event");
      return emit(event.selected);
    });
  }
}
