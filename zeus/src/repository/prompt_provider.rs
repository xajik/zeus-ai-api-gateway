pub enum Prompt {
    PoiVisual,
    Poi,
    Ocr,
    Compact,
}

impl Prompt {
    pub fn prompt(&self) -> String {
        match self {
            Prompt::Poi => {
                include_str!("../../prompt/poi.txt").to_string()
            },
            Prompt::PoiVisual => {
                include_str!("../../prompt/poi_visual.txt").to_string()
            },
            Prompt::Ocr => {
                include_str!("../../prompt/ocr.txt").to_string()
            },
            Prompt::Compact => {
                include_str!("../../prompt/compact.txt").to_string()
            }
        }
    }
}
