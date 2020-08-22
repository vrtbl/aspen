use toml;
use toml::map::Map;
use serde::{Serialize, Deserialize};
use semver::Version;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    package: Package,
    dependencies: Map<String, toml::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    // required keys
    name:    String,      // package name
    version: String,      // package version, using semver
    authors: Vec<String>, // package authors

    // optional keys
    readme:        Option<String>, // path to package's readme
    license:       Option<String>, // Path to package's liscense
    repository:    Option<String>, // URL to package's repository
    documentation: Option<String>, // URL to package's documentation
}

impl Manifest {
    pub fn new(name: String) -> Manifest {
        Manifest {
            package: Package {
                name,
                version: format!("{}", Version::new(0, 0, 0)),
                authors: vec![],
                readme: None,
                license: None,
                repository: None,
                documentation: None,
            },
            dependencies: Map::new(),
        }
    }

    pub fn from(source: &str) -> Option<Manifest> {
        // TODO: error handling
        toml::from_str(source).ok()
    }
}
