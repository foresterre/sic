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

type Signal = Result<Success, Failure>;

#[derive(Debug, PartialEq)]
enum Failure {
    TodoError,
}

#[derive(Debug, PartialEq)]
enum Success {
    Empty,
    Continue,
    Stop,
}

struct Pipeline {
    buffer: RefCell<DynamicImage>,
    pre_image_processing_steps: Vec<Box<Fn(&Config) -> Signal>>,
    image_processing_steps: Vec<Box<Fn(&RefCell<DynamicImage>, &Config) -> Signal>>,
    config: Config,
}

fn make_stop_early_wrapper(result: Result<Success, Failure>) -> Result<Success, Result<Success, Failure>> {
    match result {
        Ok(Success::Stop) => Err(Ok(Success::Stop)),
        Ok(ok) => Ok(ok),
        Err(e) => Err(Err(e)),

    }
}

fn flatten_stop_early_wrapper(wrapper: Result<Success, Result<Success, Failure>>) -> Result<Success, Failure> {
    match wrapper {
        Ok(ok) => Ok(ok),
        Err(Ok(ok)) => Ok(ok),
        Err(Err(e)) => Err(e),
    }
}

impl Pipeline {

    fn run_pre_image_phase(&mut self) -> Result<Success, Failure> {

        let step = self.pre_image_processing_steps
            .iter()
            .try_fold(Success::Empty, |acc, box_fn| {
                let result = box_fn(&self.config);
                make_stop_early_wrapper(result)
            });

        let result = flatten_stop_early_wrapper(step);

        println!("result: {:?}", result);

        result
    }


    fn run(&mut self) -> Result<(), String> {
        // current code is written to be explicit; and as a proof of concept

        println!("\n>>> Stage I\n");

        // Part I: pre image processing
        let pre_image_phase = self.run_pre_image_phase();

        match pre_image_phase {
            Ok(Success::Stop) => return Ok(()),
            Ok(joy) => (),
            Err(_) => return Err("Failure in the pre imageops stage.".to_string())
        }

        println!("\n>>> Stage II\n");

        // Part II: image processing
        while !self.image_processing_steps.is_empty() {
            let current = self.image_processing_steps.pop();

            match current {
                Some(f) => {
                    let func: Box<Fn(&RefCell<DynamicImage>, &Config) -> Signal> = f;

                    match func(&self.buffer, &self.config) {
                        Ok(Success::Empty) => (),
                        Ok(Success::Continue) => (),
                        Ok(Success::Stop) => return Ok(()),
                        Err(Failure::TodoError) => return Err(":todo_p2:".to_string()),
                    }
                }
                None => return Ok(()),
            }
        }

        // Completed all steps
        Ok(())
    }
}

#[test]
fn __debug__() {
    use crate::config::FormatEncodingSettings;
    use crate::config::JPEGEncodingSettings;
    use crate::config::PNMEncodingSettings;

    fn example_stop_err(config: &Config) -> Signal {
        println!("stop with err");
        println!("...");
        Err(Failure::TodoError)
    }

    fn example_stop(config: &Config) -> Signal {
        println!("stop");
        println!("...");
        Ok(Success::Stop)
    }

    fn example_continue(config: &Config) -> Signal {
        println!("continue");
        println!("...");
        Ok(Success::Continue)
    }

    fn example_image_processing(cell: &RefCell<DynamicImage>, config: &Config) -> Signal {
        println!("continue: image processing 1");

        use image::GenericImageView;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut buffer = cell.borrow_mut();

        let dim = buffer.dimensions();
        println!("{} x {}", dim.0, dim.1);

        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        match buffer.save(format!("a_{}.png", id)) {
            Ok(_) => (),
            Err(_) => return Err(Failure::TodoError)
        }

        *buffer = buffer.rotate90();

        match buffer.save(format!("b_{}.png", id)) {
            Ok(_) => (),
            Err(_) => return Err(Failure::TodoError),
        }

        println!("...");
        Ok(Success::Continue)
    }

    fn example_image_processing_2(cell: &RefCell<DynamicImage>, config: &Config) -> Signal {
        println!("continue: image processing 2");


        use image::GenericImageView;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut buffer = cell.borrow_mut();

        let dim = buffer.dimensions();
        println!("{} x {}", dim.0, dim.1);

        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + 1;

        match buffer.save(format!("a_{}.png", id)) {
            Ok(_) => (),
            Err(_) => return Err(Failure::TodoError)
        }

        *buffer = buffer.grayscale();

        match buffer.save(format!("b_{}.png", id)) {
            Ok(_) => (),
            Err(_) => return Err(Failure::TodoError),
        }


        println!("...");
        Ok(Success::Continue)
    }

    let mut pipeline = Pipeline {
        buffer: RefCell::new(image::open("resources/rainbow_8x6.bmp").unwrap()),

        pre_image_processing_steps: vec![
//            Box::new(example_stop),
//            Box::new(example_stop_err),
            Box::new(example_continue),
            Box::new(example_continue),
//            Box::new(example_stop_err),
            Box::new(example_continue),
            Box::new(example_stop),

        ],
        image_processing_steps: vec![
            Box::new(example_image_processing_2),
            Box::new(example_image_processing),
        ],
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
    };

    let r = pipeline.run();

    assert_eq!(Ok(()), r);
    assert!(false);
}
