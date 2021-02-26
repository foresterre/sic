#![deny(clippy::all)]
#![allow(clippy::unnecessary_wraps)]

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use clap::Clap;
use guppy::graph::{PackageGraph, PackageMetadata};
use guppy::MetadataCommand;

use crate::arguments::{CargoPublishWorkspace, PublishWorkspace};
use crate::combinators::ConditionallyDo;
use crate::package::PackageWrapper;
use crate::pipeline::commit::Commit;
use crate::pipeline::publish_crate::PublishCrate;
use crate::pipeline::tag::Tag;
use crate::pipeline::update_dependents::UpdateDependents;
use crate::pipeline::update_manifest::UpdateManifest;
use crate::pipeline::Action;
use crate::topological_workspace::get_topological_workspace;
use std::path::Path;

pub(crate) mod arguments;
pub(crate) mod backup;
pub(crate) mod combinators;
pub(crate) mod package;
pub(crate) mod pipeline;
pub(crate) mod topological_workspace;

// TODO
//  * run post publish crate
//      * set all versions to next -pre (e.g. 0.1.0-pre) at once
//      * commit
fn main() -> anyhow::Result<()> {
    let fake_cargo: CargoPublishWorkspace = CargoPublishWorkspace::parse();
    let args = fake_cargo.get_arguments();

    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(&args.manifest);

    let graph = PackageGraph::from_command(&mut cmd)?;
    let workspace = graph.query_workspace();
    let set = workspace.resolve();

    // topo sorted dependencies
    let components = get_topological_workspace(&set);

    // this is a collection of dependent packages. After updating the key package, its value members
    // should update the key package version field to the new version of the key package
    let dependants_db = create_dependents_db(&components);

    // TODO: create new git branch
    new_publish(&components, &dependants_db, &args)?;

    // tag published version from current working directory
    tag_version(
        &args,
        &std::env::current_dir().with_context(|| {
            "Unable to tag: the current working directory could not be determined"
        })?,
    )?;

    Ok(())
}

// A. in topo order do, for each pkg:

// 1. (opt) backup manifest
// 2. set component to new version
// 3. (opt?) have dependents in workspace rely on any version (*), so cargo can "find a valid version"
//      Necessary since the new version is not published yet, but since we have local changes, we can't rely on the crates.io version
//      This would be less of an issue if we developed each crate as a more separate component, and update them often as such; i.e. we wouldn't
//      depend on the unpublished versions while working towards a next release
// 4. publish new version
// 5. do what we would've like to do at 3, set the version of the dependents to the new version of the component (def in 2.)
// 6. (opt) commit changes
// 7. (opt) tag changes

// B. (opt) commit changes
// C. (opt) tag changes
fn new_publish<'g>(
    components: &[PackageMetadata<'g>],
    dependents_db: &'g HashMap<&'g str, HashSet<PackageWrapper<'g>>>,
    args: &PublishWorkspace,
) -> Result<()> {
    for (i, component) in components.iter().enumerate() {
        let path = component.manifest_path();

        let crate_folder = path
            .parent()
            .with_context(|| format!("Expected parent folder for Cargo manifest at {}", path))?;

        let kickstart = Ok(());

        let _ = kickstart
            .and_then(|_| set_new_version(component, &args)) // set_new_version for component to 'version
            .and_then(|_| set_dependent_version(component, dependents_db, &args, Some("*"))) // set_dependent_version to * locally
            .do_if(|| true, |_| publish(component, &args)) // publish for component
            .and_then(|_| set_dependent_version(component, dependents_db, &args, None)) // set_dependent_version to 'version
            .do_if(
                || true,
                |_| make_commit(&args, *component, crate_folder.as_ref()),
            )?; // commit changes

        // give the index time to update, if we still have to publish another crate
        if i < components.len() {
            println!("main: sleeping for {} seconds...", args.sleep);
            std::thread::sleep(std::time::Duration::from_secs(args.sleep));
        }
    }

    Ok(())
}

// packages required to be reverse topo sorted
fn create_dependents_db<'a>(
    packages: &'a [PackageMetadata<'a>],
) -> HashMap<&'a str, HashSet<PackageWrapper<'a>>> {
    let ws = packages.iter().map(|p| p.name()).collect::<HashSet<_>>();

    packages.iter().fold(HashMap::new(), |mut map, dep| {
        dep.direct_links()
            .filter(|n| ws.contains(n.dep_name()))
            .for_each(|link| {
                let set = map.entry(link.resolved_name()).or_default();

                set.insert(PackageWrapper::new(*dep, link));
            });

        map
    })
}

fn set_new_version<'g>(component: &'g PackageMetadata<'g>, args: &PublishWorkspace) -> Result<()> {
    let mut act = UpdateManifest::new(component);
    act.run(args)
}

fn set_dependent_version<'g>(
    component: &'g PackageMetadata<'g>,
    dependents_db: &'g HashMap<&'g str, HashSet<PackageWrapper<'g>>>,
    args: &PublishWorkspace,
    override_new_version: Option<&str>,
) -> Result<()> {
    let mut act = UpdateDependents::new(dependents_db, &component, override_new_version);
    act.run(args)
}

fn publish(component: &PackageMetadata, args: &PublishWorkspace) -> Result<()> {
    let mut publisher = PublishCrate::try_new(component, args)?;
    publisher.run(args)
}

fn make_commit(
    args: &PublishWorkspace,
    component: PackageMetadata,
    crate_folder: &Path,
) -> Result<()> {
    let mut commit = Commit::from_working_dir(args, component.name(), crate_folder);
    commit.run(args)
}

fn tag_version(args: &PublishWorkspace, cwd: &Path) -> Result<()> {
    let mut act = Tag::from_working_dir(args, cwd);
    act.run(args)
}
