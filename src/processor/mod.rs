use crate::config::Config;
use image::DynamicImage;
use std::cell::RefCell;

pub(crate) mod conversion;
pub(crate) mod encoding_format;
pub(crate) mod help_display;
pub(crate) mod image_operations;
pub(crate) mod license_display;

#[cfg(test)]
pub(in crate::processor) mod mod_test_includes;

// Process With Config
// Design perspective
//
// Current requirements:
// - Have a standard trait which can be used to create a pipeline for sic
//   - It actually has two traits, one which takes &self and one which takes &mut self.
//   - Perhaps a future trait should necessarily not take self?
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

// =================================
// ========== Prototype ============
// =================================

// TODO Let pipeline execute run something more useful (for example the last state) to test on.
//      Right now can just see (from a test) that something executed with Ok or Err.
// TODO Add iterator trait implementations for single steps, such that single
//                    steps can be checked separately.
// TODO Add post stage.

// TODO{option2}:
// Look at: https://rust-lang-nursery.github.io/edition-guide/rust-2018/trait-system/impl-trait-for-returning-complex-types-with-ease.html#impl-trait-and-closures
// Can it be used to replace dynamic dispatch?

type Signal = Result<Success, Failure>;

#[derive(Debug, PartialEq)]
enum Success {
    Empty,
    Continue,
    Stop, // Signal a (potentially early) stop of the pipeline.
}

#[derive(Debug, PartialEq)]
enum Failure {
    TodoError,
}

#[derive(Debug, PartialEq)]
enum PipelineReportSuccess {
    Pre(Success),
    Mid(Success),
    Post(Success),
}

#[derive(Debug, PartialEq)]
enum PipelineReportFailure {
    Pre(Failure),
    Mid(Failure),
    Post(Failure),
}

// Wrap a stop early Ok result in an Err so that try_fold will stop early.
fn make_stop_early_wrapper(
    result: Result<Success, Failure>,
) -> Result<Success, Result<Success, Failure>> {
    match result {
        Ok(Success::Stop) => Err(Ok(Success::Stop)),
        Ok(ok) => Ok(ok),
        Err(e) => Err(Err(e)),
    }
}

// Unwrap a stop early wrapped <Err(Ok(_))> result.
fn flatten_stop_early_wrapper(
    wrapper: Result<Success, Result<Success, Failure>>,
) -> Result<Success, Failure> {
    match wrapper {
        Ok(ok) => Ok(ok),
        Err(Ok(ok)) => Ok(ok),
        Err(Err(e)) => Err(e),
    }
}

// TODO{option2}
struct LinearSingleImagePipeline {
    buffer: RefCell<DynamicImage>,
    config: Config,
}

// TODO{option2}
trait RunPipeline {
    // stages
    // 1. pre/prepare/...
    // 2. mid/image_processing/...
    // 3. post/fin/finalize/convert/...

    fn run_prepare(&mut self, steps: &[impl Fn(&Config) -> Signal]) -> Signal;
    fn run_image_processing(&mut self, steps: &[impl Fn(&RefCell<DynamicImage>, &Config) -> Signal]) -> Signal;
    fn run_convert(&mut self, convert_step: Option<impl Fn(&RefCell<DynamicImage>, &Config) -> Signal>) -> Signal;
}

// TODO{option2}
impl RunPipeline for LinearSingleImagePipeline {
    fn run_prepare(&mut self, steps: &[impl Fn(&Config) -> Signal]) -> Signal {
        let step = steps
            .iter()
            .try_fold(Success::Empty, |acc, func| {
                let result = func(&self.config);
                make_stop_early_wrapper(result)
            });

        let result = flatten_stop_early_wrapper(step);

        println!("option2 | Stage I: {:?}", result);

        result
    }

    fn run_image_processing(&mut self, steps: &[impl Fn(&RefCell<DynamicImage>, &Config) -> Signal]) -> Signal {
        let step = steps
            .iter()
            .try_fold(Success::Empty, |acc, func| {
                let result = func(&self.buffer, &self.config);
                make_stop_early_wrapper(result)
            });

        let result = flatten_stop_early_wrapper(step);

        println!("option2 | Stage II: {:?}", result);

        result
    }

    // TODO{option2}: combinator version
    fn run_convert(&mut self, convert_step: Option<impl Fn(&RefCell<DynamicImage>, &Config) -> Signal>) -> Signal {
        if convert_step.is_some() {
            let func = convert_step.unwrap();
            func(&self.buffer, &self.config)
        }
        else {
            Err(Failure::TodoError)
        }

    }
}

// TODO{option2}
impl LinearSingleImagePipeline {
    pub fn run_all_stages(&mut self,
                          pre: &[impl Fn(&Config) -> Signal],
                          mid: &[impl Fn(&RefCell<DynamicImage>, &Config) -> Signal],
                          _fin: Option<impl Fn(&RefCell<DynamicImage>, &Config) -> Signal>) -> Result<PipelineReportSuccess, PipelineReportFailure> {
        // current code is written to be explicit; and as a proof of concept

        println!("\n>>> Stage I\n");

        // Part I: pre image processing
        let pre_image_phase = self.run_prepare(&pre);

        match pre_image_phase {
            Ok(Success::Stop) => return Ok(PipelineReportSuccess::Pre(Success::Stop)),
            Ok(_) => (),
            Err(e) => return Err(PipelineReportFailure::Pre(e)),
        }

        println!("\n>>> Stage II\n");

        // Part II: image processing
        let image_phase = self.run_image_processing(&mid);

        match image_phase {
            Ok(Success::Stop) => Ok(PipelineReportSuccess::Mid(Success::Stop)),
            Ok(joy) => Ok(PipelineReportSuccess::Mid(joy)),
            Err(e) => Err(PipelineReportFailure::Mid(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FormatEncodingSettings;
    use crate::config::JPEGEncodingSettings;
    use crate::config::PNMEncodingSettings;

    // Option 2 variants
    // --------------------

    fn success_impl() -> impl Fn(&Config) -> Signal {
        |config: &Config| Ok(Success::Continue)
    }

    fn stop_impl() -> impl Fn(&Config) -> Signal {
        |config: &Config| Ok(Success::Stop)
    }

    fn error_impl() -> impl Fn(&Config) -> Signal {
        |config: &Config| Err(Failure::TodoError)
    }

    fn success_mid_impl() -> impl Fn(&RefCell<DynamicImage>, &Config) -> Signal {
        |rc, config| Ok(Success::Continue)
    }

    // helper
    fn process_rot90_continue_impl() -> impl Fn(&RefCell<DynamicImage>, &Config) -> Signal {
        |cell: &RefCell<DynamicImage>, config: &Config| {
            println!("continue: image processing 1");

            use image::GenericImageView;
            use std::time::{SystemTime, UNIX_EPOCH};

            let mut buffer = cell.borrow_mut();

            let dim = buffer.dimensions();
            println!("{} x {}", dim.0, dim.1);

            let _id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if cfg!(feature = "debug-pipeline") {
                match buffer.save(format!("bbb$rot90_{}_a.png", _id)) {
                    Ok(_) => (),
                    Err(_) => return Err(Failure::TodoError),
                }
            }

            *buffer = buffer.rotate90();

            if cfg!(feature = "debug-pipeline") {
                match buffer.save(format!("bbb$rot90_{}_b.png", _id)) {
                    Ok(_) => (),
                    Err(_) => return Err(Failure::TodoError),
                }
            }

            println!("...");
            Ok(Success::Continue)
        }
    }

    // helper
    fn process_grayscale_continue_impl() -> impl Fn(&RefCell<DynamicImage>, &Config) -> Signal {
        |cell: &RefCell<DynamicImage>, config: &Config| {
            println!("continue: image processing 2");

            use image::GenericImageView;
            use std::time::{SystemTime, UNIX_EPOCH};

            let mut buffer = cell.borrow_mut();

            let dim = buffer.dimensions();
            println!("{} x {}", dim.0, dim.1);

            let _id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1;

            if cfg!(feature = "debug-pipeline") {
                match buffer.save(format!("bbb$grayscale_{}_a.png", _id)) {
                    Ok(_) => (),
                    Err(_) => return Err(Failure::TodoError),
                }
            }

            *buffer = buffer.grayscale();

            if cfg!(feature = "debug-pipeline") {
                match buffer.save(format!("bbb$grayscale_{}_b.png", _id)) {
                    Ok(_) => (),
                    Err(_) => return Err(Failure::TodoError),
                }
            }

            println!("...");
            Ok(Success::Continue)
        }
    }

    fn make_pipeline_impl() -> LinearSingleImagePipeline {
        LinearSingleImagePipeline {
            buffer: RefCell::new(image::open("resources/rainbow_8x6.bmp").unwrap()),
            config: Config {
                licenses: vec![],
                user_manual: None,
                script: None,
                forced_output_format: None,
                disable_automatic_color_type_adjustment: false,
                encoding_settings: FormatEncodingSettings {
                    jpeg_settings: JPEGEncodingSettings { quality: 80 },
                    pnm_settings: PNMEncodingSettings { ascii: true },
                },
                output: Some("hello_world".to_string()),
            },
        }
    }

    fn empty_fin() -> impl Fn(&RefCell<DynamicImage>, &Config) -> Signal {
        |cell: &RefCell<DynamicImage>, config: &Config| {
            Ok(Success::Continue)
        }
    }


//    pre: &[impl Fn(&Config) -> Signal],
//    mid: &[impl Fn(&RefCell<DynamicImage>, &Config) -> Signal]
//    fin: Option<impl Fn(&RefCell<DynamicImage>, &Config) -> Signal>) -> Result<FinalStageSuccess, FinalStageFailure>

    #[test]
    fn pre_stage_empty_impl() {
        let mut pre = Vec::new();
        pre.push(success_impl());

        let mut pipeline: LinearSingleImagePipeline = make_pipeline_impl();

        let result = pipeline.run_prepare(&pre);
        assert_eq!(Ok(Success::Empty), result);
    }

    #[test]
    fn mid_stage_proc_impl() {
        let mut mid = Vec::new();
        mid.push(process_grayscale_continue_impl());
        mid.push(process_grayscale_continue_impl());
        mid.push(process_rot90_continue_impl());

        let mut pipeline: LinearSingleImagePipeline = make_pipeline_impl();

        let result = pipeline.run_image_processing(&mid);
        assert_eq!(Ok(Success::Continue), result);
    }

//    #[test]
//    fn pre_stage_ok_stop_at_once() {
//        let pre = vec![stop_box()];
//        let mid = vec![];
//
//        let mut pipeline = make_pipeline(pre, mid);
//        let result = pipeline.process_pre_stage();
//
//        assert_eq!(Ok(Success::Stop), result);
//    }
//
//    #[test]
//    fn pre_stage_ok_stop_after_a_while() {
//        let pre = vec![
//            success_box(),
//            success_box(),
//            Box::new(success_alt()),
//            success_box(),
//            success_box(),
//            stop_box(),
//        ];
//        let mid = vec![];
//
//        let mut pipeline = make_pipeline(pre, mid);
//        let result = pipeline.process_pre_stage();
//
//        assert_eq!(Ok(Success::Stop), result);
//    }
//
//    #[test]
//    fn pre_stage_ok_stop_after_a_while_with_steps_left() {
//        let pre = vec![
//            success_box(),
//            success_box(),
//            success_box(),
//            stop_box(),    // returned Signal
//            success_box(), // not executed
//            success_box(), // not executed
//        ];
//        let mid = vec![];
//
//        let mut pipeline = make_pipeline(pre, mid);
//        let result = pipeline.process_pre_stage();
//
//        assert_eq!(Ok(Success::Stop), result);
//    }
//
//    #[test]
//    fn pre_stage_ok_stop_before_err() {
//        let pre = vec![
//            success_box(),
//            success_box(),
//            success_box(),
//            stop_box(),  // returned Signal
//            error_box(), // not executed
//        ];
//        let mid = vec![];
//
//        let mut pipeline = make_pipeline(pre, mid);
//        let result = pipeline.process_pre_stage();
//
//        assert_eq!(Ok(Success::Stop), result);
//    }
//
//    #[test]
//    fn pre_stage_err_before_stop() {
//        let pre = vec![error_box(), stop_box()];
//        let mid = vec![];
//
//        let mut pipeline = make_pipeline(pre, mid);
//        let result = pipeline.process_pre_stage();
//
//        assert_eq!(Err(Failure::TodoError), result);
//    }
//
//    #[test]
//    fn pre_stage_err() {
//        let pre = vec![error_box()];
//        let mid = vec![];
//
//        let mut pipeline = make_pipeline(pre, mid);
//        let result = pipeline.process_pre_stage();
//
//        assert_eq!(Err(Failure::TodoError), result);
//    }
}
