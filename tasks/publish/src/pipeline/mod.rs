use crate::arguments::PublishWorkspace;

pub mod commit;
pub mod publish_crate;
pub mod update_dependents;
pub mod update_manifest;

// TODO
pub trait Action {
    fn run(&mut self, args: &PublishWorkspace) -> anyhow::Result<()>;
}
