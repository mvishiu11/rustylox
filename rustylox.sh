#!/bin/sh

set -e # Exit early if any commands fail

(
  cd "$(dirname "$0")"
  cargo build \
      --quiet \
      --release \
      --target-dir=/tmp/rustylox-target \
      --manifest-path Cargo.toml
)

exec /tmp/rustylox-target/release/rustylox "$@"
