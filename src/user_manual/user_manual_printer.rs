use std::error::Error;

use sic_core::combostew::config::Config;

use crate::app::custom_config::manual_arg;
use crate::user_manual::HelpIndex;

#[derive(Debug, Eq, PartialEq)]
pub enum UserManualCompletedWith {
    PrintedTopic,
    PrintingNotRequested,
    ShowIndex,
    TopicNotFound,
}

#[derive(Debug, Default)]
pub struct UserManualPrinter;

impl UserManualPrinter {
    fn print_help(help: &HelpIndex, topic: &str) -> UserManualCompletedWith {
        let page = help.get_topic(&*topic.to_lowercase());

        match page {
            Some(it) => {
                println!("{}", it.help_text);
                UserManualCompletedWith::PrintedTopic
            }
            None => {
                println!("This topic is unavailable in the user manual. The following topics are available: \n\t* {}", help.get_available_topics());
                UserManualCompletedWith::TopicNotFound
            }
        }
    }

    pub fn show(&self, config: &Config) -> Result<UserManualCompletedWith, Box<dyn Error>> {
        if let Some(topic) = manual_arg(&config.application_specific) {
            let help = HelpIndex::default();

            if topic == "index" {
                println!(
                    "The following topics are available: \n\t* {}",
                    help.get_available_topics()
                );
                return Ok(UserManualCompletedWith::ShowIndex);
            } else {
                let done = UserManualPrinter::print_help(&help, &topic);
                return Ok(done);
            }
        }

        Ok(UserManualCompletedWith::PrintingNotRequested)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sic_core::combostew::config::{
        Config, ConfigItem, FormatEncodingSettings, JPEGEncodingSettings, PNMEncodingSettings,
    };

    // Construct a default config with configurable help page.
    fn setup_config(page: Option<String>) -> Config {
        let mk = |item: Option<String>| ConfigItem::OptionStringItem(item);

        Config {
            tool_name: "sic#test#user_manual_printer",
            licenses: vec![],
            forced_output_format: None,
            disable_automatic_color_type_adjustment: false,
            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings::new_result((false, None)).expect(
                    "Unable to build a default config instance for sic#test#user_manual_printer.",
                ),
                pnm_settings: PNMEncodingSettings::new(false),
            },
            output: None,
            application_specific: vec![mk(None), mk(page)],
        }
    }

    #[test]
    fn show_index() {
        let config = setup_config(Some("index".to_string()));

        assert_eq!(
            UserManualPrinter::default()
                .show(&config)
                .expect("Show Index completion result."),
            UserManualCompletedWith::ShowIndex
        )
    }

    #[test]
    fn printed_help() {
        let config = setup_config(Some("blur".to_string()));

        assert_eq!(
            UserManualPrinter::default()
                .show(&config)
                .expect("Success display completion result."),
            UserManualCompletedWith::PrintedTopic
        )
    }

    #[test]
    fn topic_not_found() {
        let config = setup_config(Some("blurr".to_string()));

        assert_eq!(
            UserManualPrinter::default()
                .show(&config)
                .expect("Topic not found completion result."),
            UserManualCompletedWith::TopicNotFound
        )
    }

    #[test]
    fn dont_print() {
        let config = setup_config(None);

        assert_eq!(
            UserManualPrinter::default()
                .show(&config)
                .expect("No printing requested completion result."),
            UserManualCompletedWith::PrintingNotRequested
        )
    }
}
