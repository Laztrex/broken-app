#!/usr/bin/env bash
set -euo pipefail

cargo build --release
perf record -g --call-graph dwarf ./target/release/demo
perf report
