use crate::config::Config;
use std::cell::Cell;
use image::DynamicImage;
use crate::config::FormatEncodingSettings;
use crate::config::JPEGEncodingSettings;
use crate::config::PNMEncodingSettings;

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

// =================================
// ========== Prototype ============
// =================================

enum Signal {
    Continue,
    Stop,
    StopWithError(String)
}

struct Pipeline {
    buffer: Cell<DynamicImage>,
    pre_image_processing_steps: Vec<Box<Fn(&Config) -> Signal>>,
    image_processing_steps: Vec<Box<Fn(&Cell<DynamicImage>, &Config) -> Signal>>,
    config: Config,
}

impl Pipeline {
    fn run(&mut self) -> Result<(), String> {
        // current code is written to be explicit; and as a proof of concept

        // pop() pops from the back
        self.pre_image_processing_steps.reverse();

        let mut step = 0;
        let mut current = None;

        while !self.pre_image_processing_steps.is_empty() {
            current = self.pre_image_processing_steps.pop();
            step += 1;

            println!("step {}", step);
            match current {
                Some(f) => {
                    let func: Box<Fn(&Config) -> Signal> = f;

                    match func(&self.config) {
                        Signal::Continue => (),
                        Signal::Stop => return Ok(()),
                        Signal::StopWithError(message) => return Err(message),
                    }
                },
                None => return Ok(()),
            };

        }

        // Completed all steps
        Ok(())
    }


}


#[test]
fn __debug__() {
    fn example_stop_err(config: &Config) -> Signal {
        println!("stop with err");
        Signal::StopWithError("stop with err".to_string())
    }

    fn example_stop(config: &Config) -> Signal {
        println!("stop");
        Signal::Stop
    }

    fn example_continue(config: &Config) -> Signal {
        println!("continue");
        Signal::Continue
    }

    let mut pipeline = Pipeline {
        buffer: Cell::new(DynamicImage::new_luma8(2, 2)),
        pre_image_processing_steps: vec![
            Box::new(example_continue),
            Box::new(example_stop),
            Box::new(example_stop_err)
        ],
        image_processing_steps: vec![],
        config: Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: None,
            disable_automatic_color_type_adjustment: false,
            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings {
                    quality: 80,
                },
                pnm_settings: PNMEncodingSettings {
                    ascii: true,
                }
            },
            output: Some("hello_world".to_string()),
        }
    };

    let r = pipeline.run();


    assert_eq!(Ok(()), r);
}
