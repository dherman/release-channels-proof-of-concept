# Release channels proof-of-concept

## The idea

This proof-of-concept shows an implementation technique for providing multiple release channels for a Rust library.

A **track** is a naming convention of prerelease versions.

An **unstable feature** is a naming convention for feature flags.

The example library at the workspace root uses the convention `dev*` for a development track and the convention `unstable-*` for unstable features. The library uses attribute macros `#[tracks::unless_matches()]` and `#[tracks::if_env()]` to enforce the policy that its unstable features are only allowed to be enabled when using the library's development track.

## Tests

A few manual tests to demonstrate the behavior:

- **Should succeed:** build without unstable features
  - `cargo build`
- **Should fail:** build with unstable features
  - `cargo build --no-default-features --features=unstable-futures`
- **Should succeed:** build with dev channel and unstable features
  - change the root package's version to `"0.1.0-dev.20220312"`
  - `cargo build --no-default-features --features=unstable-futures`

## How it works

Unfortunately, [Cargo feature flags are only exposed to the environment during `build.rs` scripts](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts), so the implementation requires a couple steps:

1. The library's `build.rs` checks for any unstable features by iterating through the build environment variables, and saving the result in a custom env var (`NEON_UNSTABLE`).
2. The library's `lib.rs` uses the attribute macros to check the library version and the `NEON_UNSTABLE` env var to enforce the policy at compile time.
