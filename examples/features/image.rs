use std::time::Duration;

use termal::{
    Result, codes,
    image::{
        RawImg, push_sixel, push_texel_half, push_texel_half_no_bg,
        push_texel_quater,
    },
    raw::request,
};

pub fn show_push_sixel() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    let img_data = include_bytes!("../img2_256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);

    push_sixel(&mut buf, &img);

    println!("{buf}");

    Ok(())
}

pub fn show_push_texel_half() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    let img_data = include_bytes!("../img2_256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);

    push_texel_half(&img, &mut buf, "\n", Some(60), None);

    println!("{buf}");

    Ok(())
}

pub fn show_push_texel_quater() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();

    let img_data = include_bytes!("../img2_256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);

    push_texel_quater(&img, &mut buf, "\n", Some(60), None);

    println!("{buf}");

    Ok(())
}

pub fn show_push_texel_half_no_bg() -> Result<()> {
    let mut buf = codes::CLEAR.to_string();
    buf += "any_bg:\n";

    let img_data = include_bytes!("../img3_256.data");
    let img = RawImg::from_rgb(img_data.into(), 256, 256);

    push_texel_half(&img, &mut buf, "\n", Some(60), None);

    let bg = request::default_bg_color(Duration::from_millis(100))?;
    buf += codes::RESET;
    buf += "\nwithout default bg:\n";

    push_texel_half_no_bg(&img, &mut buf, "\n", Some(60), None, bg.scale());

    println!("{buf}");

    Ok(())
}
