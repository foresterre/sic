pub mod commit;
pub mod publish_crate;
pub mod update_dependents;
pub mod update_manifest;

// TODO
// pub trait RenderProgression {
//     fn spawn_progress(&self) -> ProgressBar;
//
//     fn render_progress(&self, progress: Arc<ProgressBar>);
// }
//
// // TODO
// pub trait PipelineStep {
//     fn run(&self, dry_run: bool) -> Result<()>;
// }
