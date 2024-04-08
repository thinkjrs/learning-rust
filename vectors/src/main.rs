use vectors::*;

fn main() {
    let mut my_vec: Vec<u8> = Vec::new();
    my_vec.push(0);
    my_vec.push(1);
    my_vec.push(1);
    my_vec.push(0);
    println!("{:?}", my_vec);

    let my_vec2: Vec<u8> = vec![0, 1, 1, 0];
    println!("{:?}", my_vec2);

    let mut v: Vec<i32> = vec![1, 2, 3];

    let mut v2: Vec<&mut i32> = Vec::new();

    for val in &mut v {
        v2.push(val);
    }

    *v2[0] = 5;

    let a = *v2[0];

    let b = v[0];

    println!("{a} {b}");

    let mut ads = vec![
        Ad::Video(VideoAd::new(
            String::from("Test video title"),
            1000,
            String::from("https://tincre.com"),
        )),
        Ad::Text(TextAd::new(
            String::from("Test text"),
            1250,
            String::from("https://tincre.com/agency"),
        )),
    ];

    ads.push(Ad::Text(TextAd::new(
        String::from("Test text 2"),
        300,
        String::from("https://tincre.com/agency"),
    )));

    println!("{:#?}", ads);

    if let Some(last_ad) = ads.pop() {
        println!("Last ad: {:#?}", last_ad);
    }

    println!("{:#?}", ads);
}
