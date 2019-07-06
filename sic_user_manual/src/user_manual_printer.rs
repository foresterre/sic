use std::error::Error;

use crate::HelpIndex;

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

    pub fn show(&self, item: Option<&str>) -> Result<UserManualCompletedWith, Box<dyn Error>> {
        let help = || HelpIndex::default();
        match item {
            Some(topic) if topic == "index" => {
                println!(
                    "The following topics are available: \n\t* {}",
                    help().get_available_topics()
                );
                Ok(UserManualCompletedWith::ShowIndex)
            }
            Some(topic) => {
                let done = UserManualPrinter::print_help(&help(), &topic);
                Ok(done)
            }
            None => Ok(UserManualCompletedWith::PrintingNotRequested),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_index() {
        let page = Some("index");

        assert_eq!(
            UserManualPrinter::default()
                .show(page)
                .expect("Show Index completion result."),
            UserManualCompletedWith::ShowIndex
        )
    }

    #[test]
    fn printed_help() {
        let page = Some("blur");

        assert_eq!(
            UserManualPrinter::default()
                .show(page)
                .expect("Success display completion result."),
            UserManualCompletedWith::PrintedTopic
        )
    }

    #[test]
    fn topic_not_found() {
        let page = Some("blurr");

        assert_eq!(
            UserManualPrinter::default()
                .show(page)
                .expect("Topic not found completion result."),
            UserManualCompletedWith::TopicNotFound
        )
    }

    #[test]
    fn dont_print() {
        let page = None;

        assert_eq!(
            UserManualPrinter::default()
                .show(page)
                .expect("No printing requested completion result."),
            UserManualCompletedWith::PrintingNotRequested
        )
    }
}
