#!/usr/bin/env just --justfile

generate:
    flutter pub run build_runner build
