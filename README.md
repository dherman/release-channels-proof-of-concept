# Release channels proof-of-concept

## The idea

An _unstable feature_ is any feature flag with the prefix `unstable-`.

A _dev channel_ is a package version whose prerelease has prefix `dev`.

This proof-of-concept shows how you can implement a macro `dev_channel_macro::enforce!()` that enforces the policy that a library's unstable features are only allowed to be enabled when using the library's dev channel.

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
2. The library's `lib.rs` calls the `dev_channel_macro::enforce!()` proc macro, which checks the library version and the `NEON_UNSTABLE` env var to enforce the policy at compile time.
