#[tracks::unless_matches("^dev")]
#[tracks::if_env("NEON_UNSTABLE")]
compile_error!("Unstable features can only be used with development releases of Neon.

See https://neon-bindings.com/channels for more information.");
