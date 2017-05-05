#!/usr/bin/env bash

cargo build && rm -f /tmp/librustexampleplugin.so && cp -f target/debug/librustexampleplugin.so /tmp/