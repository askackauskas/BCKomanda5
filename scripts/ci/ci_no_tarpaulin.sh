#!/bin/sh

set -o errexit
set -o nounset

cd "$(dirname "$0")"

export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

git submodule update --init \
    ../../eth2-cache        \
    ../../eth2-extra-tests  \
    ../../eth2.0-spec-tests
(
    cd ../../eth2.0-spec-tests
    git lfs pull
)

curl                     \
    --fail               \
    --proto =https       \
    --show-error         \
    --silent             \
    --tlsv1.2            \
    https://sh.rustup.rs |
    sponge               |
    sh -s -- --no-modify-path -y

# `cargo-fmt` must be run in the root of a crate. It appears to be a regression:
# <https://github.com/rust-lang/rustfmt/issues/3647>
# Running `cargo-fmt` with `--all` works too but takes noticeably longer. This explains why:
# <https://github.com/rust-lang/rustfmt/issues/4247#issuecomment-644957261>
(
    cd ../..
    cargo fmt -- --check
)

./clippy.bash --deny warnings

cargo test --no-fail-fast
