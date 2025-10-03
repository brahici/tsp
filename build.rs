use std::collections::BTreeMap;
use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub package: Package,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DependencyDetails),
}
#[derive(Debug, Deserialize)]
pub struct DependencyDetails {
    pub version: Option<String>,
}

fn dep_version(dep: &Dependency) -> String {
    match dep {
        Dependency::Detailed(details) => details.version.as_ref().unwrap().to_owned(),
        Dependency::Simple(version) => version.to_owned(),
    }
}

fn main() {
    let cargo_toml_str = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml: Manifest = toml::from_str(cargo_toml_str.as_str()).unwrap();

    println!(
        "cargo::rustc-env=PACKAGE_VERSION={}",
        cargo_toml.package.version
    );
    for (name, dep) in cargo_toml.dependencies.iter() {
        if name == "chrono" {
            println!("cargo::rustc-env=CHRONO_VERSION={}", dep_version(dep));
        }
    }
}
