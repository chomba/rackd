pub mod casts;
pub mod domain;
pub mod query;

pub fn version() -> semver::Version {
    let version = env!("CARGO_PKG_VERSION");
    semver::Version::parse(version).unwrap()
}