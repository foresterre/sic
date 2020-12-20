use clap::{AppSettings, Clap};

/// Topologically publish a complete workspace
///
/// If you see this message, you probably want to use this command instead by executing:
/// 'cargo publish-workspace <options, ...>'. Alternatively, you can also run:
/// 'cargo-publish-workspace publish-workspace <options, ...>'
#[derive(Debug, Clap)]
#[clap(name = "cargo-publish-workspace", bin_name = "cargo")]
pub enum CargoPublishWorkspace {
    PublishWorkspace(PublishWorkspace),
}

impl CargoPublishWorkspace {
    pub fn get_arguments(self) -> PublishWorkspace {
        match self {
            Self::PublishWorkspace(opts) => opts,
        }
    }
}

/// Topologically publish a complete workspace
#[derive(Clap, Debug)]
#[clap(global_setting(AppSettings::VersionlessSubcommands))]
pub struct PublishWorkspace {
    /// Simulate running this program
    #[clap(long)]
    pub(crate) dry_run: bool,

    /// The workspace manifest file, usually the root Cargo.toml
    #[clap(long, default_value = "Cargo.toml")]
    pub(crate) manifest: String,

    /// The version to which all workspace crates will be updated
    #[clap(long)]
    pub(crate) new_version: String,
}
