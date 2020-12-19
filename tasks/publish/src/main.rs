#![deny(clippy::all)]

use crate::commit::create_git;
use crate::publish_crate::create_publisher;
use crate::update_dependents::create_dependent_updater;
use crate::update_manifest::create_manifest_updater;
use anyhow::Context;
use guppy::graph::{DependencyDirection, PackageGraph, PackageLink, PackageMetadata};
use guppy::MetadataCommand;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub(crate) mod backup;
pub(crate) mod commit;
pub(crate) mod publish_crate;
pub(crate) mod update_dependents;
pub(crate) mod update_manifest;

#[derive(Clone, Debug)]
struct Args {
    dry_run: bool,
    manifest: String,
    version: String,
}

// TODO
//  * run post publish crate
//      * set all versions to next -pre (e.g. 0.1.0-pre) at once
//      * commit
//
// TODO improvements:
//  * Instead of using name().starts_with("sic"), we could also use  pkg.in_workspace() and pkg.publish().is_some()
//    which would make it re-usable for (my) projects outside sic =)
fn main() -> anyhow::Result<()> {
    let mut args = pico_args::Arguments::from_env();

    let args = Args {
        dry_run: args.contains("--dry-run"),
        manifest: args
            .opt_value_from_str("--manifest")?
            .unwrap_or_else(|| "Cargo.toml".to_string()),
        version: args
            .opt_value_from_str("--new-version")?
            .with_context(|| "--new-version is required")?,
    };

    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(&args.manifest);

    let graph = PackageGraph::from_command(&mut cmd)?;
    let topological_workspace = graph.query_workspace();
    let set = topological_workspace.resolve();

    // topo sorted dependencies
    let components_to_update = set
        .packages(DependencyDirection::Reverse)
        .filter(|pkg| pkg.name().starts_with("sic"));

    // this is a collection of dependent packages. After updating the key package, its value members
    // should update the key package version field to the new version of the key package
    let dependants_db = set
        .packages(DependencyDirection::Reverse)
        .filter(|pkg| pkg.name().starts_with("sic"))
        .fold(empty_map(), |mut map, dep| {
            dep.direct_links()
                .filter(|n| n.dep_name().starts_with("sic"))
                .for_each(|link| {
                    let set = map.entry(link.resolved_name()).or_default();

                    set.insert(PackageWrapper::new(dep, link));
                });

            map
        });

    // TODO: create new git branch

    publish_packages(
        components_to_update,
        &dependants_db,
        &args.version,
        args.dry_run,
    )?;

    Ok(())
}

fn empty_map<'a>() -> HashMap<&'a str, HashSet<PackageWrapper<'a>>> {
    HashMap::new()
}

// Exists because PackageMetadata doesn't implement Hash
struct PackageWrapper<'g> {
    pkg_metadata: PackageMetadata<'g>,
    link: PackageLink<'g>,
}

impl<'g> Debug for PackageWrapper<'g> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pkg_metadata.name())
    }
}

impl<'g> PackageWrapper<'g> {
    pub(crate) fn new(package: PackageMetadata<'g>, link: PackageLink<'g>) -> Self {
        Self {
            pkg_metadata: package,
            link,
        }
    }

    pub(crate) fn metadata(&self) -> PackageMetadata<'g> {
        self.pkg_metadata
    }

    pub(crate) fn link(&self) -> PackageLink<'g> {
        self.link
    }
}

impl<'g> Hash for PackageWrapper<'g> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pkg_metadata.name().hash(state);
    }
}

impl<'g> PartialEq for PackageWrapper<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.pkg_metadata.name().eq(other.pkg_metadata.name())
    }
}

impl<'g> Eq for PackageWrapper<'g> {}

fn publish_packages<'g>(
    components: impl Iterator<Item = PackageMetadata<'g>>,
    dependents_db: &'g HashMap<&'g str, HashSet<PackageWrapper<'g>>>,
    new_version: &str,
    dry_run: bool,
) -> anyhow::Result<()> {
    // update workspace crates in topological order
    for component in components {
        let path = component.manifest_path();

        let crate_folder = path.parent().with_context(|| {
            format!(
                "Expected parent folder for Cargo manifest at {}",
                path.display()
            )
        })?;

        // update the specific dependency
        let manifest_updater = create_manifest_updater(dry_run, component);
        manifest_updater.update_dependency_version(new_version)?;

        // FIXME{workaround}: accept any version locally before we update the package
        let dependents_updater = create_dependent_updater(dry_run, dependents_db);
        dependents_updater.update_all(component, "*")?;

        // publish changes
        let mut publisher = create_publisher(dry_run, component)?;
        publisher.publish()?;

        // update dependents to new version
        let dependents_updater = create_dependent_updater(dry_run, dependents_db);
        dependents_updater.update_all(component, new_version)?;

        // commit changes
        let mut command = create_git(dry_run, crate_folder);
        command.commit_package(component.name(), new_version);
        command.run()?;
    }

    Ok(())
}
