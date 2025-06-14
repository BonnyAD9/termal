use termal::{
    Result, codes,
    image::{RawImg, push_sixel},
};

pub fn show_push_sixel() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    let img_data = include_bytes!("../img2_256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);

    push_sixel(&mut buf, &img);

    println!("{buf}");

    Ok(())
}
