#!/usr/bin/env just --justfile

generate:
    dart run build_runner build --delete-conflicting-outputs
