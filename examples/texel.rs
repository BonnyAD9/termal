use termal::{error::Result, sixel::RawImg, texel};

fn main() -> Result<()> {
    let img_data = include_bytes!("img256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);
    let mut res = String::new();
    texel::draw_blocks(&img, &mut res, "\n", 64, 32);
    println!("{}", res);

    Ok(())
}
