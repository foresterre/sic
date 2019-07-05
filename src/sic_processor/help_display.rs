use std::process;

use combostew::config::Config;
use combostew::processor::ProcessWithConfig;

use crate::app::custom_config::manual_arg;
use crate::help::HelpIndex;

// TODO{foresterre}: User manual should be refactored later.
#[derive(Debug, Default)]
pub struct HelpDisplayProcessor;

impl HelpDisplayProcessor {
    fn print_help(help: &HelpIndex, topic: &str) {
        let page = help.get_topic(&*topic.to_lowercase());

        match page {
            Some(it) => println!("{}", it.help_text),
            None => println!("This topic is unavailable in the user manual. The following topics are available: \n\t* {}", help.get_available_topics()),
        }
    }
}

impl ProcessWithConfig<()> for HelpDisplayProcessor {
    fn process(&self, config: &Config) {
        if let Some(topic) = manual_arg(&config.application_specific) {
            let help = HelpIndex::default();

            if topic == "index" {
                println!(
                    "The following topics are available: \n\t* {}",
                    help.get_available_topics()
                );
            } else {
                HelpDisplayProcessor::print_help(&help, &topic);
            }

            // TODO: return Result
            process::exit(0);
        }
    }
}
