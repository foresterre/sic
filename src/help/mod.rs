use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const SCRIPT: &str = include_str!("../../docs/cli_help_script.txt");
const SCRIPT_OPERATION_BLUR: &str = include_str!("../../docs/cli_help_script_blur.txt");

// Probably should create a macro to generate various of the entries. Now, I've to manually
// add a new topic to the name match function, and to the available topics list.
#[derive(Debug, Clone)]
pub enum HelpTopicKind {
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

impl HelpTopicKind {
    pub fn name(&self) -> String {
        let text = match self {
            HelpTopicKind::Script => "script",
            HelpTopicKind::Blur => "blur",
            HelpTopicKind::Brighten => "brighten",
            HelpTopicKind::Contrast => "contrast",
            HelpTopicKind::Crop => "crop",
            HelpTopicKind::Filter3x3 => "filter3x3",
            HelpTopicKind::FlipH => "fliph",
            HelpTopicKind::FlipV => "flipv",
            HelpTopicKind::GrayScale => "grayscale",
            HelpTopicKind::HueRotate => "huerotate",
            HelpTopicKind::Invert => "invert",
            HelpTopicKind::Resize => "resize",
            HelpTopicKind::Rotate90 => "rotate90",
            HelpTopicKind::Rotate180 => "rotate180",
            HelpTopicKind::Rotate270 => "rotate270",
            HelpTopicKind::Unsharpen => "unsharpen",
            HelpTopicKind::Unavailable => "",
        };

        text.to_string()
    }

    pub fn from_name(topic: &str) -> HelpTopicKind {
        match topic {
            "script" => HelpTopicKind::Script,
            "blur" => HelpTopicKind::Blur,
            "brighten" => HelpTopicKind::Brighten,
            "contrast" => HelpTopicKind::Contrast,
            "crop" => HelpTopicKind::Crop,
            "filter3x3" => HelpTopicKind::Filter3x3,
            "fliph" => HelpTopicKind::FlipH,
            "flipv" => HelpTopicKind::FlipV,
            "grayscale" => HelpTopicKind::GrayScale,
            "huerotate" => HelpTopicKind::HueRotate,
            "Invert" => HelpTopicKind::Invert,
            "Resize" => HelpTopicKind::Resize,
            "Rotate90" => HelpTopicKind::Rotate90,
            "Rotate180" => HelpTopicKind::Rotate180,
            "Rotate270" => HelpTopicKind::Rotate270,
            "Unsharpen" => HelpTopicKind::Unsharpen,
            _ => HelpTopicKind::Unavailable,
        }
    }

    pub fn text(&self) -> String {
        let help_page = match self {
            HelpTopicKind::Script => SCRIPT,
            HelpTopicKind::Blur => SCRIPT_OPERATION_BLUR,
            HelpTopicKind::Brighten => "brighten",
            HelpTopicKind::Contrast => "contrast",
            HelpTopicKind::Crop => "crop",
            HelpTopicKind::Filter3x3 => "filter3x3",
            HelpTopicKind::FlipH => "fliph",
            HelpTopicKind::FlipV => "flipv",
            HelpTopicKind::GrayScale => "grayscale",
            HelpTopicKind::HueRotate => "huerotate",
            HelpTopicKind::Invert => "invert",
            HelpTopicKind::Resize => "resize",
            HelpTopicKind::Rotate90 => "rotate90",
            HelpTopicKind::Rotate180 => "rotate180",
            HelpTopicKind::Rotate270 => "rotate270",
            HelpTopicKind::Unsharpen => "unsharpen",
            HelpTopicKind::Unavailable => unreachable!(),
        };

        help_page.to_string()
    }
}

impl PartialEq for HelpTopicKind {
    fn eq(&self, other: &HelpTopicKind) -> bool {
        self.name() == other.name()
    }
}

impl Eq for HelpTopicKind {}

impl Hash for HelpTopicKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

#[derive(Debug)]
pub struct HelpTopic {
    //    kind: HelpTopicKind,
    name: String,
    pub help_text: String,
    indexed: bool,
}

impl HelpTopic {
    pub fn get_name(&self) -> &str {
        &*self.name
    }
}

pub struct HelpIndex {
    index: HashMap<HelpTopicKind, HelpTopic>,
}

impl HelpIndex {
    pub fn new() -> HelpIndex {
        let mut dict = HashMap::new();

        insert_help_entry(&mut dict, &mut HelpTopicKind::Script, true);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Blur, true);

        HelpIndex {
            index: dict,
        }

    }

    pub fn get_topic(&self, name: &str) -> Option<&HelpTopic> {
        let topic = &HelpTopicKind::from_name(name);
        self.index.get(topic)
    }

    pub fn get_available_topics(&self) -> Vec<&str> {
        self.index.values().filter(|v| v.indexed).map(|v| v.get_name()).collect()
    }
}

// Dislike the cloning here, this should be possible in a different way (?)
fn insert_help_entry(dict: &mut HashMap<HelpTopicKind, HelpTopic>, kind: &mut HelpTopicKind, should_be_indexed: bool) {
    dict.insert(
        kind.clone(),
        HelpTopic {
            name: String::from(kind.name()),
            help_text: String::from(kind.text()),
            indexed: should_be_indexed,
        }
    );
}