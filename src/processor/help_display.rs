use std::process;

use crate::config::Config;
use crate::help::HelpIndex;
use crate::processor::ProcessWithConfig;

// TODO{foresterre}: User manual should be refactored later.
#[derive(Debug)]
pub(crate) struct HelpDisplayProcessor;

impl HelpDisplayProcessor {
    pub fn new() -> HelpDisplayProcessor {
        HelpDisplayProcessor {}
    }

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
        if let Some(topic) = &config.user_manual {
            let help = HelpIndex::new();

            if topic == "index" {
                println!(
                    "The following topics are available: \n\t* {}",
                    help.get_available_topics()
                );
            } else {
                HelpDisplayProcessor::print_help(&help, &topic);
            }

            process::exit(0);
        }
    }
}
