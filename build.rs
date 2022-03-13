fn main() {
    tracks_env::set(
        "NEON_UNSTABLE",
        tracks_env::get_features(r"^unstable-"),
    );
}
