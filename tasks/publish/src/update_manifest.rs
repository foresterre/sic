use crate::backup::backup_manifest;
use guppy::graph::PackageMetadata;
use std::path::Path;
use toml_edit::value;

pub(crate) fn create_manifest_updater<'g>(
    is_dry_run: bool,
    pkg: PackageMetadata<'g>,
) -> Box<dyn UpdateManifest + 'g> {
    if is_dry_run {
        Box::new(DryRunManifestUpdate { pkg })
    } else {
        Box::new(RegularManifestUpdate { pkg })
    }
}

pub(crate) trait UpdateManifest {
    fn update_dependency_version(&self, new_version: &str) -> anyhow::Result<()>;
}

pub(crate) struct RegularManifestUpdate<'g> {
    pkg: PackageMetadata<'g>,
}

impl UpdateManifest for RegularManifestUpdate<'_> {
    fn update_dependency_version(&self, new_version: &str) -> anyhow::Result<()> {
        backup_manifest(self.pkg.manifest_path())?;
        toml_update(self.pkg.manifest_path(), new_version)?;

        Ok(())
    }
}

pub(crate) struct DryRunManifestUpdate<'g> {
    pkg: PackageMetadata<'g>,
}

impl UpdateManifest for DryRunManifestUpdate<'_> {
    fn update_dependency_version(&self, new_version: &str) -> anyhow::Result<()> {
        println!(
            "update-manifest: updating crate '{}' manifest version to '{}'",
            self.pkg.name(),
            new_version
        );

        Ok(())
    }
}

fn toml_update(manifest: &Path, new_version: &str) -> anyhow::Result<()> {
    let contents = std::fs::read_to_string(manifest)?;

    let mut document = contents.parse::<toml_edit::Document>()?;
    document["package"]["version"] = value(new_version);

    std::fs::write(manifest, document.to_string_in_original_order())?;

    Ok(())
}
