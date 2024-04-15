use semver::Version;

pub fn app_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).expect("Program version is not a valid format")
}
