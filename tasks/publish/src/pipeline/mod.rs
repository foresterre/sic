use crate::arguments::PublishWorkspace;

pub mod commit;
pub mod publish_crate;
pub mod tag;
pub mod update_dependents;
pub mod update_manifest;

pub trait Action {
    fn run(&mut self, args: &PublishWorkspace) -> anyhow::Result<()>;
}
