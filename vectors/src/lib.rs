#[derive(Debug)]
pub struct TextAd {
    ad_text: String,
    budget: u32,
    target_url: String,
}
#[derive(Debug)]
pub struct VideoAd {
    ad_title: String,
    budget: u32,
    target_url: String,
}

impl TextAd {
    pub fn new(ad_text: String, budget: u32, target_url: String) -> TextAd {
        TextAd {
            ad_text,
            budget,
            target_url,
        }
    }
}

impl VideoAd {
    pub fn new(ad_title: String, budget: u32, target_url: String) -> VideoAd {
        VideoAd {
            ad_title,
            budget,
            target_url,
        }
    }
}

#[derive(Debug)]
pub enum Ad {
    Text(TextAd),
    Video(VideoAd),
}
