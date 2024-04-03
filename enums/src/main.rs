// These are borrowed from the Standard Library
// https://doc.rust-lang.org/std/net/enum.IpAddr.html
use chrono;
use std::mem::transmute;

#[derive(Debug)]
pub struct Ipv4Addr {
    octets: [u8; 4],
}

#[derive(Debug)]
pub struct Ipv6Addr {
    octets: [u8; 16],
}

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr {
            octets: [a, b, c, d],
        }
    }
    pub const fn octets(&self) -> [u8; 4] {
        self.octets
    }
}

impl Ipv6Addr {
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        let addr16 = [
            a.to_be(),
            b.to_be(),
            c.to_be(),
            d.to_be(),
            e.to_be(),
            f.to_be(),
            g.to_be(),
            h.to_be(),
        ];
        Ipv6Addr {
            // All elements in `addr16` are big endian.
            // SAFETY: `[u16; 8]` is always safe to transmute to `[u8; 16]`.
            octets: unsafe { transmute::<_, [u8; 16]>(addr16) },
        }
    }
    pub const fn octets(&self) -> [u8; 16] {
        self.octets
    }
}

#[derive(Debug)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
struct AdPlatform {
    visible_name: String,
    internal_name: String,
}
#[derive(Debug)]
struct DisplayAd {
    start_timestamp: i64,
    budget: u32,
    title: String,
    copy: String,
    call_to_action: String,
    media_asset_urls: Vec<String>,
    button_text: String,
    target_url: String,
}
#[derive(Debug)]
struct HoverAd {
    start_timestamp: i64,
    budget: u32,
    title: String,
    call_to_action: String,
    media_asset_urls: Vec<String>,
    target_url: String,
}
#[derive(Debug)]
struct FeedAd {
    start_timestamp: i64,
    budget: u32,
    title: String,
    copy: String,
    call_to_action: String,
    media_asset_urls: Vec<String>,
    button_text: String,
    target_url: String,
}
#[derive(Debug)]
struct VideoAd {
    start_timestamp: i64,
    budget: u32,
    title: String,
    copy: String,
    media_asset_urls: Vec<String>,
    target_url: String,
}
#[derive(Debug)]
struct InlineTextAd {
    start_timestamp: i64,
    budget: u32,
    title: String,
    copy: String,
    call_to_action: String,
    target_url: String,
}

#[derive(Debug)]
enum Ad {
    Display(DisplayAd),
    Hover(HoverAd),
    Feed(FeedAd),
    Video(VideoAd),
    InlineText(InlineTextAd),
}

impl Ad {
    fn init(&self) {
        match self {
            Ad::Display(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::Hover(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::Feed(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::Video(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
            Ad::InlineText(ad) => {
                println!("initializing: {:#?}", ad);
                send_notification(&ad.title)
            }
        }
    }
}
fn send_notification(title: &String) {
    println!("Sending email notification for campaign {}", title);
}
fn main() {
    let v4_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6_ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("v4 IP: {:#?}", v4_ip);
    println!("v6 IP: {:#?}", v6_ip);

    let start_timestamp: i64 = chrono::Utc::now().timestamp();
    let my_ad = Ad::Display(DisplayAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        call_to_action: String::from("On sale today only!"),
        button_text: String::from("Buy now"),
        target_url: String::from("https://tincre.com/agency"),
        media_asset_urls: vec![String::from("https://https://res.cloudinary.com/tincre/video/upload/v1708121578/nfpwzh1oslr8qhdyotzs.mov")],
    });
    my_ad.init();

    let my_ad = Ad::Hover(HoverAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        call_to_action: String::from("On sale today only!"),
        target_url: String::from("https://tincre.com/agency"),
        media_asset_urls: vec![String::from("https://https://res.cloudinary.com/tincre/video/upload/v1708121578/nfpwzh1oslr8qhdyotzs.mov")],
    });
    my_ad.init();

    let my_ad = Ad::Feed(FeedAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        call_to_action: String::from("On sale today only!"),
        target_url: String::from("https://tincre.com/agency"),
        media_asset_urls: vec![String::from("https://https://res.cloudinary.com/tincre/video/upload/v1708121578/nfpwzh1oslr8qhdyotzs.mov")],
        button_text: String::from("Buy now"),
    });
    my_ad.init();

    let my_ad = Ad::Video(VideoAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        target_url: String::from("https://tincre.com/agency"),
        media_asset_urls: vec![String::from("https://https://res.cloudinary.com/tincre/video/upload/v1708121578/nfpwzh1oslr8qhdyotzs.mov")],
    });
    my_ad.init();

    let my_ad = Ad::InlineText(InlineTextAd {
        start_timestamp,
        budget: 5000,
        title: String::from("My first ad"),
        copy: String::from("Buy whatever I'm selling. It's great!"),
        call_to_action: String::from("On sale today only!"),
        target_url: String::from("https://tincre.com/agency"),
    });
    my_ad.init();
}
