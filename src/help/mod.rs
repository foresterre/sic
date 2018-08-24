use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// Probably should create a macro to generate various of the entries. Now, I've to manually
// add a new topic to the name match function, and to the available topics list.
// The Rust documentation advises to use an enum if you now your values in advance.
// Here, the values are known in advance, but since I also need the string values, it might be
// an option to just use the string values, even for pattern matching.
// Another option is to generate help pages and the index by looking at the available
// pages (text files) in the docs/ directory. The category and page can be parsed
// (for example) from the file name.
// Or a similar option, include a 'docs.json' like file which specifies the meta data for the
// docs to be included.
#[derive(Debug, Clone)]
pub enum HelpTopicKind {
    Script,

    Blur,
    Brighten,
    Contrast,
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
            _ => "",
        };

        text.to_string()
    }

    pub fn from_name(topic: &str) -> HelpTopicKind {
        match topic {
            "script" => HelpTopicKind::Script,
            "blur" => HelpTopicKind::Blur,
            "brighten" => HelpTopicKind::Brighten,
            "contrast" => HelpTopicKind::Contrast,
            "filter3x3" => HelpTopicKind::Filter3x3,
            "fliph" => HelpTopicKind::FlipH,
            "flipv" => HelpTopicKind::FlipV,
            "grayscale" => HelpTopicKind::GrayScale,
            "huerotate" => HelpTopicKind::HueRotate,
            "invert" => HelpTopicKind::Invert,
            "resize" => HelpTopicKind::Resize,
            "rotate90" => HelpTopicKind::Rotate90,
            "rotate180" => HelpTopicKind::Rotate180,
            "rotate270" => HelpTopicKind::Rotate270,
            "unsharpen" => HelpTopicKind::Unsharpen,
            _ => HelpTopicKind::Unavailable,
        }
    }

    pub fn text(&self) -> String {
        let help_page = match self {
            HelpTopicKind::Script => include_str!("../../docs/cli_help_script.txt"),
            HelpTopicKind::Blur => include_str!("../../docs/cli_help_script_blur.txt"),
            HelpTopicKind::Brighten => include_str!("../../docs/cli_help_script_brighten.txt"),
            HelpTopicKind::Contrast => include_str!("../../docs/cli_help_script_contrast.txt"),
            HelpTopicKind::Filter3x3 => include_str!("../../docs/cli_help_script_filter3x3.txt"),
            HelpTopicKind::FlipH => include_str!("../../docs/cli_help_script_fliph.txt"),
            HelpTopicKind::FlipV => include_str!("../../docs/cli_help_script_flipv.txt"),
            HelpTopicKind::GrayScale => include_str!("../../docs/cli_help_script_grayscale.txt"),
            HelpTopicKind::HueRotate => include_str!("../../docs/cli_help_script_huerotate.txt"),
            HelpTopicKind::Invert => include_str!("../../docs/cli_help_script_invert.txt"),
            HelpTopicKind::Resize => include_str!("../../docs/cli_help_script_resize.txt"),
            HelpTopicKind::Rotate90 => include_str!("../../docs/cli_help_script_rotate90.txt"),
            HelpTopicKind::Rotate180 => include_str!("../../docs/cli_help_script_rotate180.txt"),
            HelpTopicKind::Rotate270 => include_str!("../../docs/cli_help_script_rotate270.txt"),
            HelpTopicKind::Unsharpen => include_str!("../../docs/cli_help_script_unsharpen.txt"),
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

// Up for consideration: perhaps a (short) description should be added.
// This might help to select a topic from the index.
// Same for an explicit category.
#[derive(Debug)]
pub struct HelpTopic {
    name: String,
    pub help_text: String,
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

        // ugh.
        insert_help_entry(&mut dict, &mut HelpTopicKind::Script);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Blur);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Brighten);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Contrast);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Filter3x3);
        insert_help_entry(&mut dict, &mut HelpTopicKind::FlipH);
        insert_help_entry(&mut dict, &mut HelpTopicKind::FlipV);
        insert_help_entry(&mut dict, &mut HelpTopicKind::GrayScale);
        insert_help_entry(&mut dict, &mut HelpTopicKind::HueRotate);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Invert);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Resize);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Rotate90);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Rotate180);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Rotate270);
        insert_help_entry(&mut dict, &mut HelpTopicKind::Unsharpen);

        HelpIndex { index: dict }
    }

    pub fn get_topic(&self, name: &str) -> Option<&HelpTopic> {
        let topic = &HelpTopicKind::from_name(name);
        self.index.get(topic)
    }

    pub fn get_available_topics(&self) -> String {
        let mut options = self
            .index
            .values()
            .map(|v| v.get_name())
            .collect::<Vec<_>>();

        options.sort_unstable();
        options.join("\n\t* ")
    }
}

// Dislike the cloning here, this should be possible in a different way (?)
fn insert_help_entry(
    dict: &mut HashMap<HelpTopicKind, HelpTopic>,
    kind: &mut HelpTopicKind,
) {
    dict.insert(
        kind.clone(),
        HelpTopic {
            name: String::from(kind.name()),
            help_text: String::from(kind.text()),
        },
    );
}
