import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import '../../models/input_model.dart';

class RawKeyFocusScope extends StatelessWidget {
  final FocusNode? focusNode;
  final ValueChanged<bool>? onFocusChange;
  final InputModel inputModel;
  final Widget child;

  RawKeyFocusScope({
    this.focusNode,
    this.onFocusChange,
    required this.inputModel,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return FocusScope(
        autofocus: true,
        child: Focus(
            autofocus: true,
            canRequestFocus: true,
            focusNode: focusNode,
            onFocusChange: onFocusChange,
            onKey: inputModel.handleRawKeyEvent,
            child: child));
  }
}

class RawPointerMouseRegion extends StatelessWidget {
  final InputModel inputModel;
  final Widget child;
  final MouseCursor? cursor;
  final PointerEnterEventListener? onEnter;
  final PointerExitEventListener? onExit;
  final PointerDownEventListener? onPointerDown;
  final PointerUpEventListener? onPointerUp;

  RawPointerMouseRegion(
      {this.onEnter,
      this.onExit,
      this.cursor,
      this.onPointerDown,
      this.onPointerUp,
      required this.inputModel,
      required this.child});

  @override
  Widget build(BuildContext context) {
    return Listener(
        onPointerHover: inputModel.onPointHoverImage,
        onPointerDown: (evt) {
          onPointerDown?.call(evt);
          inputModel.onPointDownImage(evt);
        },
        onPointerUp: (evt) {
          onPointerUp?.call(evt);
          inputModel.onPointUpImage(evt);
        },
        onPointerMove: inputModel.onPointMoveImage,
        onPointerSignal: inputModel.onPointerSignalImage,
        onPointerPanZoomStart: inputModel.onPointerPanZoomStart,
        onPointerPanZoomUpdate: inputModel.onPointerPanZoomUpdate,
        onPointerPanZoomEnd: inputModel.onPointerPanZoomEnd,
        child: MouseRegion(
            cursor: cursor ?? MouseCursor.defer,
            onEnter: onEnter,
            onExit: onExit,
            child: child));
  }
}
