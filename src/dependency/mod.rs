use std::path::Path;

use semver::{Version, VersionReq};

//pub use self::bower::BowerDependency;
pub use self::cargo::CargoDependency;
pub use self::composer::ComposerDependency;
pub use self::npm::NpmDependency;
pub use self::puppet::PuppetDependency;

//mod bower;
mod cargo;
mod composer;
mod npm;
mod puppet;

pub trait Dependency : Clone + Send + 'static {
    fn to_check(dependency_file_contents: &str, path: &Path) -> Vec<Self>;
    fn name(&self) -> &String;
    fn version_req(&self) -> &VersionReq;
    fn registry_version(&self) -> Option<Version>;
}
