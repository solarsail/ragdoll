/*
extern crate ini;

use ini::Ini;
*/


pub struct Settings {
    pub window_width: u32,
    pub window_height: u32,
    pub scroll_rate: u32
}

impl Settings {
    fn make_default() -> Settings {
        Settings {
            window_width: 640,
            window_height: 480,
            scroll_rate: 100
        }
    }

    pub fn load(file: &str) -> Settings {
        let mut settings = Settings::make_default();
        /*
        if let mut conf = Ini::load_from_file(file) {
            let section = conf.section(None);
            settings.window_width = section.get("window_width")
                .map_or(settings.window_width, |s| { s.parse::<i32>() });
            settings.window_height = section.get("window_height")
                .map_or(settings.window_height, |s| { s.parse::<i32>() });
            settings.scroll_rate = section.get("scroll_rate").unwrap_or(settings.scroll_rate);
        } else {
            let mut conf = Ini::new();
            conf.with_section(None)
                .set("window_width", settings.window_width)
                .set("window_height", settings.window_height)
                .set("scroll_rate", settings.scroll_rate);
            conf.write_to_file(file).unwrap_or_else(|| {});
        }
        */
        settings
    }
}