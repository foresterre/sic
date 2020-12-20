use crate::pipeline::backup::backup_manifest;
use crate::PackageWrapper;
use guppy::graph::PackageMetadata;
use std::collections::{HashMap, HashSet};
use toml_edit::{value, Document};

pub(crate) fn create_dependent_updater<'g>(
    is_dry_run: bool,
    dependents_db: &'g HashMap<&'g str, HashSet<PackageWrapper<'g>>>,
) -> Box<dyn UpdateDependents + 'g> {
    if is_dry_run {
        Box::new(DryRunUpdateDependents { dependents_db })
    } else {
        Box::new(RegularUpdateDependents { dependents_db })
    }
}

pub(crate) trait UpdateDependents {
    fn update_all<'g>(
        &self,
        updated_pkg: PackageMetadata<'g>,
        new_version: &str,
    ) -> anyhow::Result<()>;
}

pub(crate) struct RegularUpdateDependents<'g> {
    dependents_db: &'g HashMap<&'g str, HashSet<PackageWrapper<'g>>>,
}

impl UpdateDependents for RegularUpdateDependents<'_> {
    fn update_all<'gg>(
        &self,
        updated_pkg: PackageMetadata<'gg>,
        new_version: &str,
    ) -> anyhow::Result<()> {
        if let Some(dependents) = self.dependents_db.get(updated_pkg.name()) {
            for dependent in dependents {
                update_dependent_manifest(dependent, new_version)?;
            }
        }

        Ok(())
    }
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

pub(crate) struct DryRunUpdateDependents<'g> {
    dependents_db: &'g HashMap<&'g str, HashSet<PackageWrapper<'g>>>,
}

impl UpdateDependents for DryRunUpdateDependents<'_> {
    fn update_all<'g>(
        &self,
        updated_pkg: PackageMetadata<'g>,
        new_version: &str,
    ) -> anyhow::Result<()> {
        if let Some(dependents) = self.dependents_db.get(updated_pkg.name()) {
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
}
