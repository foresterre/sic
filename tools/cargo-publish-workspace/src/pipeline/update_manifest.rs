use crate::arguments::PublishWorkspace;
use crate::backup::backup_manifest;
use crate::pipeline::Action;
use guppy::graph::PackageMetadata;
use std::path::Path;
use toml_edit::value;

pub struct UpdateManifest<'g> {
    package: &'g PackageMetadata<'g>,
}

impl<'g> UpdateManifest<'g> {
    pub fn new(package: &'g PackageMetadata<'g>) -> Self {
        Self { package }
    }
}

impl Action for UpdateManifest<'_> {
    fn run(&mut self, args: &PublishWorkspace) -> anyhow::Result<()> {
        if args.dry_run {
            dry_update_dependency_version(self.package, args.version())
        } else {
            live_update_dependency_version(self.package, args.version())
        }
    }
}

fn live_update_dependency_version(pkg: &PackageMetadata, new_version: &str) -> anyhow::Result<()> {
    backup_manifest(pkg.manifest_path().as_ref())?;
    toml_update(pkg.manifest_path().as_ref(), new_version)?;

    Ok(())
}

fn dry_update_dependency_version(pkg: &PackageMetadata, new_version: &str) -> anyhow::Result<()> {
    println!(
        "update-manifest: updating crate '{}' manifest version to '{}'",
        pkg.name(),
        new_version
    );

    Ok(())
}

fn toml_update(manifest: &Path, new_version: &str) -> anyhow::Result<()> {
    let contents = std::fs::read_to_string(manifest)?;

    let mut document = contents.parse::<toml_edit::Document>()?;
    document["package"]["version"] = value(new_version);

    std::fs::write(manifest, document.to_string_in_original_order())?;

    Ok(())
}
