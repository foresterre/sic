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
///
/// All arguments provided after two dashes (--) will be passed on to 'cargo publish'.
/// This means, if cargo publish-workspace itself doesn't support a flag related to publishing a cargo
/// crate (yet), you can still use this method. For example, you may use a custom registry with
/// the following command `cargo publish-workspace <..options> -- --registry <registry>`. The
/// '--registry <registry>' arguments will be passed to cargo publish. Note: some arguments are also
/// passed on by cargo publish-workspace, in which case, if also provided after the two dashes may
/// be passed on twice. For example, this would be the case if we would run: `cargo publish-workspace
/// <...options> --no-verify -- --no-verify`.
///
/// By default, a tag formatted `v<version>` (e.g. v1.0.3) will be created from the current working
/// directory.
#[derive(Clap, Debug)]
#[clap(
    global_setting(AppSettings::VersionlessSubcommands),
    global_setting(AppSettings::TrailingVarArg),
    after_help("Issues, requests or questions can be submitted at: 'https://github.com/foresterre/sic/issues', please add the label 'X-cargo-release-workspace', thanks!")
)]
pub struct PublishWorkspace {
    /// Simulate running this program
    #[clap(long)]
    pub(crate) dry_run: bool,

    /// The workspace manifest file, usually the root Cargo.toml
    #[clap(long, default_value = "Cargo.toml")]
    pub(crate) manifest: String,

    /// The version to which all workspace crates will be updated
    ///
    /// Version must be semver compatible.
    #[clap(long)]
    new_version: String,

    /// Don't create a tag after publishing
    ///
    /// Tags are named after the new (workspace) version and prefixed with 'v'; for example 'v1.2.0'.
    #[clap(long)]
    pub(crate) no_git_tag: bool,

    /// Don't build the crate before publishing
    #[clap(long)]
    pub(crate) no_verify: bool,

    /// The amount of seconds to sleep between publishing of workspace packages
    ///
    /// Allows the crate registry index to update, which may be important since consecutive
    /// attempts to publish crates may contain the just published crate as dependency, and
    /// if the registry hasn't processed a crate dependency yet, publishing will fail.   
    #[clap(long, default_value = "15")]
    pub(crate) sleep: u64,

    /// Pass additional arguments to 'cargo publish' directly
    #[clap(hidden = true)]
    pub(crate) pass_on: Vec<String>,
}

impl PublishWorkspace {
    pub fn version(&self) -> &str {
        &self.new_version
    }
}
