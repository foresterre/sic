use crate::arguments::PublishWorkspace;
use crate::backup::backup_manifest;
use crate::pipeline::Action;
use crate::PackageWrapper;
use guppy::graph::PackageMetadata;
use std::collections::{HashMap, HashSet};
use toml_edit::{value, Document};

pub struct UpdateDependents<'g, 'version> {
    dependents_db: &'g DependentsDB<'g>,
    updated_pkg: &'g PackageMetadata<'g>,
    override_version: Option<&'version str>,
}

impl<'g, 'version> UpdateDependents<'g, 'version> {
    pub fn new(
        dependents_db: &'g DependentsDB<'g>,
        updated_pkg: &'g PackageMetadata<'g>,
        override_version: Option<&'version str>,
    ) -> Self {
        Self {
            dependents_db,
            updated_pkg,
            override_version,
        }
    }
}

impl Action for UpdateDependents<'_, '_> {
    fn run(&mut self, args: &PublishWorkspace) -> anyhow::Result<()> {
        // update the dependents manifests to the new version, unless we override it
        let version = if let Some(v) = self.override_version {
            v
        } else {
            args.version()
        };

        // if we perform a dry run, we should not actually update the manifests
        if args.dry_run {
            dry_update_all(self.dependents_db, self.updated_pkg, version)
        } else {
            live_update_all(self.dependents_db, self.updated_pkg, version)
        }
    }
}

type DependentsDB<'g> = HashMap<&'g str, HashSet<PackageWrapper<'g>>>;

fn live_update_all<'g>(
    dependents_db: &'g DependentsDB,
    updated_pkg: &'g PackageMetadata<'g>,
    new_version: &str,
) -> anyhow::Result<()> {
    if let Some(dependents) = dependents_db.get(updated_pkg.name()) {
        for dependent in dependents {
            update_dependent_manifest(dependent, new_version)?;
        }
    }

    Ok(())
}
fn update_dependent_manifest(dependent: &PackageWrapper<'_>, version: &str) -> anyhow::Result<()> {
    let manifest = dependent.metadata().manifest_path();
    let cargo_file = std::fs::read_to_string(manifest)?;
    let mut document = cargo_file.parse::<Document>()?;

    backup_manifest(manifest)?;

    let dependency = dependent.link().resolved_name();

    if dependent.link().normal().is_present() {
        document["dependencies"][dependency]["version"] = value(version);
    }

    if dependent.link().dev().is_present() {
        document["dev-dependencies"][dependency]["version"] = value(version);
    }

    if dependent.link().build().is_present() {
        document["build-dependencies"][dependency]["version"] = value(version);
    }

    std::fs::write(manifest, document.to_string_in_original_order())?;

    Ok(())
}

fn dry_update_all<'g>(
    dependents_db: &'g DependentsDB<'g>,
    updated_pkg: &'g PackageMetadata<'g>,
    new_version: &str,
) -> anyhow::Result<()> {
    if let Some(dependents) = dependents_db.get(updated_pkg.name()) {
        println!("update-dependents: updating dependency '{}' to '{}' for packages in workspace which depend on it {:?}", updated_pkg.name(), new_version, dependents);

        for dependent in dependents {
            println!(
                "update-dependents: dependency '{}' will be updated to '{}' for '{}'",
                updated_pkg.name(),
                new_version,
                dependent.metadata().name(),
            )
        }
    }

    Ok(())
}
