#!/usr/bin/env -S just --justfile

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u

alias r := ready

init:
  cargo binstall watchexec-cli cargo-insta typos-cli cargo-shear -y

update-builtins:
  cargo run -p update_builtins
  cargo fmt

ready:
  git diff --exit-code --quiet
  typos
  just fmt
  just check
  just test
  just lint
  just doc

watch *args='':
  watchexec --no-vcs-ignore {{args}}

fmt:
  cargo shear --fix
  cargo fmt --all
  pnpm fmt

check:
  cargo check --workspace --all-features --all-targets --locked

watch-check:
  just watch "'cargo check; cargo clippy'"

test:
  cargo test

lint:
  cargo clippy --workspace --all-targets --all-features -- --deny warnings

[unix]
doc:
  RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items

[windows]
doc:
  $Env:RUSTDOCFLAGS='-D warnings'; cargo doc --no-deps --document-private-items
