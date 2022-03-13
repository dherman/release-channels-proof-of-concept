use std::env;
use semver::Version;

pub fn detect_unstable_features() {
    let mut unstable = false;

    // Feature flags are only exposed to the environment during build scripts,
    // so we use this script to check for the use of any unstable feature flags.
    //
    // Reference:
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    for (key, _value) in env::vars() {
        if key.starts_with("CARGO_FEATURE_UNSTABLE_") {
            unstable = true;
            break;
        }
    }

    // Set a `NEON_UNSTABLE` env var that will be available at build time when
    // the Neon library is being compiled. This will be used to ensure that unstable
    // features are only enabled for the Neon dev channel.
    //
    // Reference:
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue
    println!("cargo:rustc-env=NEON_UNSTABLE={}", unstable);
}

pub fn uses_unstable_features() -> bool {
    env::var("NEON_UNSTABLE").unwrap() == "true"
}

pub fn is_dev_channel() -> bool {
    let version = Version::parse(&env::var("CARGO_PKG_VERSION").unwrap()).unwrap();

    version.pre.as_str().starts_with("dev")
}

pub fn is_legal() -> bool {
    is_dev_channel() || !uses_unstable_features()
}
