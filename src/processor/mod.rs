use crate::config::Config;
use std::cell::Cell;
use image::DynamicImage;
use crate::config::FormatEncodingSettings;
use crate::config::JPEGEncodingSettings;
use crate::config::PNMEncodingSettings;
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
    buffer: RefCell<DynamicImage>,
    pre_image_processing_steps: Vec<Box<Fn(&Config) -> Signal>>,
    image_processing_steps: Vec<Box<Fn(&RefCell<DynamicImage>, &Config) -> Signal>>,
    config: Config,
}

impl Pipeline {
    fn run(&mut self) -> Result<(), String> {
        // current code is written to be explicit; and as a proof of concept

        // Part I: pre image processing

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

        // Part II: image processing

        self.image_processing_steps.reverse();

        let mut current = None;

        while !self.image_processing_steps.is_empty() {
            current = self.image_processing_steps.pop();

            match current {
                Some(f) => {
                    let func: Box<Fn(&RefCell<DynamicImage>, &Config) -> Signal> = f;

                    match func(&self.buffer, &self.config) {
                        Signal::Continue => (),
                        Signal::Stop => return Ok(()),
                        Signal::StopWithError(message) => return Err(message),
                    }
                }
                None => return Ok(())
            }
        }



        // Completed all steps
        Ok(())
    }

//    fn handle_signal(signal: Signal) ->


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

    fn example_image_processing(cell: &RefCell<DynamicImage>, config: &Config) -> Signal {
        use image::GenericImageView;
        use std::time::{SystemTime, UNIX_EPOCH};


        let mut buffer = cell.borrow_mut();

        let dim = buffer.dimensions();
        println!("{} x {}", dim.0, dim.1);

        buffer.save(format!("before_{}.png", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
        *buffer = buffer.rotate90();
        buffer.save(format!("after_{}.png", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));

        Signal::Continue
    }

    let mut pipeline = Pipeline {
        buffer: RefCell::new(image::open("resources/rainbow_8x6.bmp").unwrap()),
        pre_image_processing_steps: vec![
            Box::new(example_continue),
            Box::new(example_continue),
            Box::new(example_continue),
//            Box::new(example_stop_err)
        ],
        image_processing_steps: vec![
            Box::new(example_image_processing),
            Box::new(example_image_processing),
        ],
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
