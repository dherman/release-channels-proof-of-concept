use std::env;

use regex::Regex;

pub fn set(key: impl AsRef<str>, value: impl AsRef<str>) {
    println!("cargo:rustc-env={}={}", key.as_ref(), value.as_ref());
}

pub fn get_features(re: &str) -> String {
    let re = Regex::new(re).unwrap();
    let features: Vec<String> =
        env::vars()
            .filter_map(|(key, _)| {
                key.strip_prefix("CARGO_FEATURE_").map(|s| s.to_string())
            })
            .filter_map(|key| {
                let feature: String = key.to_ascii_lowercase()
                                         .replace('_', "-");
                re.find(&feature).map(|m| m.as_str().to_string())
            })
            .collect();
    features.join(",")
}
