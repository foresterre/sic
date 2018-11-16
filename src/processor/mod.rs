use crate::config::Config;

pub(crate) mod conversion;
pub(crate) mod encoding_format;
pub(crate) mod help_display;
pub(crate) mod image_operations;
pub(crate) mod license_display;

// Process With Config
// Design perspective
//
// Current requirements:
// - Have a standard trait which can be used to create a pipeline for sic
//   - It actually has two traits, one which takes &self and one which takes &mut self.
//   - Perhaps a future trait should neccesarily not take self?
// - Be straightforward to implement
// - Be minimal, so it gives some order to sic, but not require too much time, because
//   it is not the main purpose of this patch (Issue#11, PR#60).
//
// Future impls:
// - Be able to actually create a Pipeline, i.e. not manually program the steps in main().
// - Be able to call something like iter() on the pipeline which walks through all stages.
// - Stages thus should be designed such that each can have an input from the previous stage,
//   i.e. be forward stage compatible.
// - Stages should either be validated to be applicable, be unable to be constructed if not
//   applicable. At a minimum they should be carefully documented, so the implementer can take
//   care of stage ordering.
// - Think about naming;

/// Linear application pipeline trait for immutable references.
pub trait ProcessWithConfig<T> {
    fn process(&self, config: &Config) -> T;
}

/// Linear application pipeline trait for mutable references.
pub trait ProcessMutWithConfig<T> {
    fn process_mut(&mut self, config: &Config) -> T;
}
