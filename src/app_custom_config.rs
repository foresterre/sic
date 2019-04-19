use combostew::config::ConfigItem;

fn unwrap_config_item(c: &ConfigItem) -> Option<&str> {
    match c {
        ConfigItem::OptionStringItem(Some(opt)) => Some(opt),
        ConfigItem::OptionStringItem(None) => None,
    }
}

pub fn script_arg(items: &[ConfigItem]) -> Option<&str> {
    items.get(0).and_then(unwrap_config_item)
}

pub fn manual_arg(items: &[ConfigItem]) -> Option<&str> {
    items.get(1).and_then(unwrap_config_item)
}
