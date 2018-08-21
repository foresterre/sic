use std::fmt;

const SCRIPT: &str = include_str!("../../docs/cli_help_script.txt");
const SCRIPT_OPERATION_BLUR: &str = include_str!("../../docs/cli_help_script_blur.txt");

#[derive(Debug)]
pub enum HelpTopic {
    Script,

    Blur,
    Brighten,
    Contrast,
    Crop,
    Filter3x3,
    FlipH,
    FlipV,
    GrayScale,
    HueRotate,
    Invert,
    Resize,
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen,

    Unavailable,
}

impl HelpTopic {
    pub fn from_str(topic: &str) -> HelpTopic {
        match topic {
            "script" => HelpTopic::Script,
            "blur" => HelpTopic::Blur,
            "brighten" => HelpTopic::Brighten,
            "contrast" => HelpTopic::Contrast,
            "crop" => HelpTopic::Crop,
            "filter3x3" => HelpTopic::Filter3x3,
            "fliph" => HelpTopic::FlipH,
            "flipv" => HelpTopic::FlipV,
            "grayscale" => HelpTopic::GrayScale,
            "huerotate" => HelpTopic::HueRotate,
            "Invert" => HelpTopic::Invert,
            "Resize" => HelpTopic::Resize,
            "Rotate90" => HelpTopic::Rotate90,
            "Rotate180" => HelpTopic::Rotate180,
            "Rotate270" => HelpTopic::Rotate270,
            "Unsharpen" => HelpTopic::Unsharpen,
            _ => HelpTopic::Unavailable,
        }
    }
}

type HelpPage<'a> = Option<&'a str>;

pub trait HelpText {
    fn help(&self) -> HelpPage;
}

impl HelpText for HelpTopic {
    fn help(&self) -> HelpPage {
        match *self {
            HelpTopic::Script => Some(SCRIPT),
            HelpTopic::Blur => Some(SCRIPT_OPERATION_BLUR),
            _ => None,
        }
    }
}

impl fmt::Display for HelpTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let help = self.help();

        let text = help.unwrap_or_else(|| "Help topic unavailable.");

        write!(f, "{}", text)
    }
}
